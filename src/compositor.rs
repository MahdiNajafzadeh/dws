use crate::Config;
use crate::action::Action;
use crate::state::State;
use anyhow::Context;
use anyhow::Result;
use smithay::backend::allocator::Fourcc;
use smithay::backend::allocator::gbm::GbmAllocator;
use smithay::backend::allocator::gbm::GbmBufferFlags;
use smithay::backend::allocator::gbm::GbmDevice;
use smithay::backend::drm::DrmDevice;
use smithay::backend::drm::DrmDeviceFd;
use smithay::backend::drm::exporter::gbm::GbmFramebufferExporter;
use smithay::backend::drm::output::{DrmOutputManager, DrmOutputRenderElements};
use smithay::backend::egl::EGLContext;
use smithay::backend::egl::EGLDisplay;
use smithay::backend::renderer::element::surface::WaylandSurfaceRenderElement;
use smithay::backend::renderer::gles::GlesRenderer;
use smithay::backend::session::Session;
use smithay::backend::session::libseat::LibSeatSession;
use smithay::backend::udev::UdevBackend;
use smithay::desktop::Space;
use smithay::output::OutputModeSource;
use smithay::reexports::calloop::EventLoop;
use smithay::reexports::calloop::Interest;
use smithay::reexports::calloop::Mode;
use smithay::reexports::calloop::PostAction;
use smithay::reexports::calloop::generic::Generic;
use smithay::reexports::drm::control::Device as ControlDevice;
use smithay::reexports::drm::control::Mode as DrmMode;
use smithay::reexports::drm::control::ModeTypeFlags;
use smithay::reexports::drm::control::connector;
use smithay::reexports::drm::control::crtc;
use smithay::reexports::rustix::fs::OFlags;
use smithay::reexports::wayland_server::Display;
use smithay::utils::DeviceFd;
use smithay::wayland;
use smithay::wayland::compositor::CompositorState;
use smithay_drm_extras::display_info;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::os::fd::AsFd;
use std::sync::Arc;
use std::time::Duration;

struct ConnectorPlan {
    handle: connector::Handle,
    mode: DrmMode,
    crtc: crtc::Handle,
    name: String,
    make: String,
    model: String,
    physical_size: (i32, i32),
}

pub struct Compositor<C: Config> {
    _config: PhantomData<C>,
    _state: State,
}

impl<C: Config> Compositor<C> {
    pub fn start() -> Result<()> {
        // for startup in C::STARTUP_LIST {
        //     startup.spawn()?;
        // }
        let bindings: HashMap<u64, Action> = C::BINDING_LIST.iter().map(|b| (b.key.bits(), b.action.clone())).collect();
        let mut event_loop = EventLoop::<State>::try_new().context("failed to init event_loop")?;
        let signal = event_loop.get_signal();
        let mut display = Display::<State>::new().context("failed to init display")?;
        let display_fd = display.as_fd().try_clone_to_owned().context("failed to create a display fd")?;
        let display_source = Generic::new(display_fd, Interest::READ, Mode::Level);
        let socket = wayland::socket::ListeningSocketSource::new_auto().context("failed to init wayland-socket")?;
        println!("socket_name: {:?}", socket.socket_name());
        let (mut session, _session_notifier) = LibSeatSession::new().context("failed to acquire session")?;
        println!("seat_name: {:?}", session.seat());
        let udev = UdevBackend::new(session.seat()).context("failed to init udev backend")?;
        let path = udev.device_list().map(|(_, path)| path.to_owned()).next().context("failed to find a drm device")?;
        println!("drm device path : {path:?}");
        let drm_device_fd = session.open(&path, OFlags::RDWR | OFlags::CLOEXEC | OFlags::NOCTTY | OFlags::NONBLOCK).context("failed to open drm device")?;
        let drm_device_fd = DrmDeviceFd::new(DeviceFd::from(drm_device_fd));
        let (drm_device, _drm_device_notifier) = DrmDevice::new(drm_device_fd.clone(), true).context("failed to init drm device")?;
        let gbm = GbmDevice::new(drm_device_fd).context("failed to init gbm device")?;
        let egl_display = unsafe { EGLDisplay::new(gbm.clone()).context("failed to init egl_display")? };
        let egl_context = EGLContext::new(&egl_display).context("failed to init egl_context")?;
        let mut renderer = unsafe { GlesRenderer::new(egl_context).context("failed to init renderer")? };
        let allocator = GbmAllocator::new(gbm.clone(), GbmBufferFlags::RENDERING | GbmBufferFlags::SCANOUT);
        let framebuffer_exporter = GbmFramebufferExporter::new(gbm.clone(), None);
        let renderer_formats = renderer.egl_context().dmabuf_render_formats().clone();
        let mut output_manager = DrmOutputManager::new(
            //
            drm_device,
            allocator,
            framebuffer_exporter,
            Some(gbm.clone()),
            [Fourcc::Argb8888, Fourcc::Abgr8888],
            renderer_formats,
        );
        let mut space = Space::default();
        let device = output_manager.device();
        let _resources = device.resource_handles().context("failed to init resource_handles")?;
        let plans: Vec<ConnectorPlan> = {
            let device = output_manager.device();
            let resources = device.resource_handles().context("failed to init resource_handles")?;
            let mut v = Vec::new();
            for connector_handle in resources.connectors() {
                let info = device.get_connector(*connector_handle, true).context("failed to get info from connector handle")?;
                if info.state() != connector::State::Connected {
                    continue;
                }
                let Some(drm_mode) = Self::preferred_mode(info.modes()) else {
                    continue;
                };
                let Some(crtc_handle) = info.encoders().iter().find_map(|encoder_handle| {
                    let encoder = device.get_encoder(*encoder_handle).ok()?;
                    resources.filter_crtcs(encoder.possible_crtcs()).first().copied()
                }) else {
                    continue;
                };
                let name = format!("{}-{}", info.interface().as_str(), info.interface_id());
                let display_info = display_info::for_connector(device, *connector_handle).unwrap();
                let size = info.size().unwrap();
                v.push(ConnectorPlan {
                    handle: *connector_handle,
                    mode: drm_mode,
                    crtc: crtc_handle,
                    name,
                    make: display_info.make().unwrap_or("Unknown".to_string()),
                    model: display_info.model().unwrap_or("Unknown".to_string()),
                    physical_size: (size.0 as i32, size.1 as i32),
                });
            }
            v
        };
        let mut outputs: HashMap<crtc::Handle, crate::state::Output> = HashMap::new();
        for plan in plans {
            let smithay_mode = smithay::output::Mode {
                size: (plan.mode.size().0 as i32, plan.mode.size().1 as i32).into(),
                refresh: plan.mode.vrefresh() as i32 * 1000,
            };
            let output = smithay::output::Output::new(
                plan.name.clone(),
                smithay::output::PhysicalProperties {
                    size: plan.physical_size.into(),
                    subpixel: smithay::output::Subpixel::Unknown,
                    make: plan.make,
                    model: plan.model,
                },
            );
            output.change_current_state(Some(smithay_mode), Some(smithay::utils::Transform::Normal), None, Some((0, 0).into()));
            output.create_global::<State>(&display.handle());
            let drm_output = output_manager
                .initialize_output::<_, WaylandSurfaceRenderElement<GlesRenderer>>(
                    //
                    plan.crtc,
                    plan.mode,
                    &[plan.handle],
                    OutputModeSource::from(&output),
                    None,
                    &mut renderer,
                    &DrmOutputRenderElements::default(),
                )
                .context("failed to initialize_output")?;
            println!(
                //
                "output initialized: {} ({}x{} @ {} hz) on crtc {:?}",
                plan.name,
                plan.mode.size().0,
                plan.mode.size().1,
                plan.mode.vrefresh(),
                plan.crtc,
            );
            space.map_output(&output, (0, 0));
            outputs.insert(plan.crtc, drm_output);
        }

        let mut state = State {
            compositor_state: CompositorState::new::<State>(&display.handle()),
            bindings,
            signal,
            session,
            space,
            gbm,
            renderer,
            output_manager,
            outputs,
        };

        let mut display_handle = display.handle();
        event_loop
            //
            .handle()
            .insert_source(socket, move |client_stream, _, _| {
                match display_handle
                    //
                    .insert_client(client_stream, Arc::new(crate::client::ClientState::default()))
                {
                    Ok(_client) => println!("new client connected"),
                    Err(error) => eprintln!("failed to connected new client : {error}"),
                }
            })
            .context("failed to insert_source into the event_loop")?;
        event_loop
            //
            .handle()
            .insert_source(display_source, move |_, _, state: &mut State| {
                display.dispatch_clients(state)?;
                display.flush_clients()?;
                Ok(PostAction::Continue)
            })
            .context("failed to insert_source into the event_loop")?;

        event_loop
            .run(
                //
                Duration::from_millis(100),
                &mut state,
                |_state| {},
            )
            .context("failed to run event_loop")?;
        Ok(())
    }

    fn preferred_mode(modes: &[DrmMode]) -> Option<DrmMode> {
        modes
            //
            .iter()
            .find(|m| m.mode_type().contains(ModeTypeFlags::PREFERRED))
            .or_else(|| modes.first())
            .copied()
    }
}
