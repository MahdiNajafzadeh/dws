use smithay::reexports::wayland_server::backend::ClientData;
use smithay::reexports::wayland_server::backend::ClientId;
use smithay::reexports::wayland_server::backend::DisconnectReason;
use smithay::wayland::compositor::CompositorClientState;

#[derive(Default)]
pub struct ClientState {
    pub compositor_state: CompositorClientState,
}
impl ClientData for ClientState {
    fn initialized(&self, _id: ClientId) {}
    fn disconnected(&self, _id: ClientId, _why: DisconnectReason) {}
}
