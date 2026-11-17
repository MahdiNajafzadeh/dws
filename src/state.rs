use crate::action::Action;
use smithay::backend::allocator::Fourcc;
use smithay::backend::allocator::gbm::GbmAllocator;
use smithay::backend::allocator::gbm::GbmDevice;
use smithay::backend::drm::DrmDeviceFd;
use smithay::backend::drm::exporter::gbm::GbmFramebufferExporter;
use smithay::backend::drm::output::DrmOutput;
use smithay::backend::drm::output::DrmOutputManager;
use smithay::backend::renderer::gles::GlesRenderer;
use smithay::backend::session::libseat::LibSeatSession;
use smithay::delegate_compositor;
use smithay::delegate_output;
use smithay::desktop::Space;
use smithay::desktop::Window;
use smithay::reexports::calloop::LoopSignal;
use smithay::reexports::drm::control::crtc;
use smithay::reexports::wayland_server::Client;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::wayland::compositor::CompositorClientState;
use smithay::wayland::compositor::CompositorHandler;
use smithay::wayland::compositor::CompositorState;
use smithay::wayland::output::OutputHandler;
use std::collections::HashMap;

pub type Output = DrmOutput<
    //
    GbmAllocator<DrmDeviceFd>,
    GbmFramebufferExporter<DrmDeviceFd>,
    Fourcc,
    DrmDeviceFd,
>;
pub type OutputManager = DrmOutputManager<
    //
    GbmAllocator<DrmDeviceFd>,
    GbmFramebufferExporter<DrmDeviceFd>,
    Fourcc,
    DrmDeviceFd,
>;

pub struct State {
    pub compositor_state: CompositorState,
    pub signal: LoopSignal,
    pub bindings: HashMap<u64, Action>,
    pub space: Space<Window>,
    pub session: LibSeatSession,
    pub gbm: GbmDevice<DrmDeviceFd>,
    pub renderer: GlesRenderer,
    pub output_manager: OutputManager,
    pub outputs: HashMap<crtc::Handle, Output>,
}

impl State {
    pub fn new() {}
}

impl OutputHandler for State {}
impl CompositorHandler for State {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }
    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<crate::client::ClientState>().unwrap().compositor_state
    }
    fn commit(&mut self, _surface: &WlSurface) {}
}

delegate_output!(State);
delegate_compositor!(State);
