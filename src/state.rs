use crate::action::Action;
use smithay::{
    backend::{renderer::gles::GlesRenderer, winit::WinitGraphicsBackend},
    reexports::{calloop::LoopSignal, wayland_server::Display},
};
use std::collections::HashMap;

pub struct State {
    pub display: Display<State>,
    pub backend: WinitGraphicsBackend<GlesRenderer>,
    pub bindings: HashMap<u64, Action>,
    pub signal: LoopSignal,
}

impl State {
    pub fn new(
        display: Display<State>,
        backend: WinitGraphicsBackend<GlesRenderer>,
        bindings: HashMap<u64, Action>,
        signal: LoopSignal,
    ) -> Self {
        Self {
            display,
            backend,
            bindings,
            signal,
        }
    }
}
