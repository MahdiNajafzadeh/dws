use smithay::{
    reexports::wayland_server::backend::{ClientData, ClientId, DisconnectReason},
    wayland::compositor::CompositorClientState,
};

#[derive(Default)]
pub struct ClientState {
    compositor_state: CompositorClientState,
}
impl ClientData for ClientState {
    fn initialized(&self, _id: ClientId) {}
    fn disconnected(&self, _id: ClientId, _why: DisconnectReason) {}
}
