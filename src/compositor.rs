use crate::{Config, action::Action, state::State};
use anyhow::{Context, Result};
use smithay::{
    backend::{renderer::gles::GlesRenderer, winit},
    reexports::{calloop::EventLoop, wayland_server::Display},
    wayland,
};
use std::{collections::HashMap, marker::PhantomData, time::Duration};

pub struct Compositor<C: Config> {
    _config: PhantomData<C>,
    _state: State,
}

impl<C: Config> Compositor<C> {
    pub fn start() -> Result<()> {
        // for startup in C::STARTUP_LIST {
        //     startup.spawn()?;
        // }
        let bindings: HashMap<u64, Action> = C::BINDING_LIST
            .iter()
            .map(|b| (b.key.bits(), b.action.clone()))
            .collect();
        let mut event_loop = EventLoop::<State>::try_new().context("event_loop")?;
        let display = Display::<State>::new().context("display")?;
        let (backend, _events) = winit::init::<GlesRenderer>().expect("backend");
        let signal = event_loop.get_signal();
        let socket = wayland::socket::ListeningSocketSource::new_auto().context("socket")?;
        let socket_name = socket.socket_name();
        println!("{socket_name:?}");
        let mut state = State::new(display, backend, bindings, signal);
        event_loop
            .run(Duration::from_millis(100), &mut state, |_| {})
            .context("event_loop.run")?;
        Ok(())
    }
}
