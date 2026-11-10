pub struct Binding {
    pub mods: Key,
    pub key: Key,
    pub action: Action,
}
pub enum Key {
    Super = (1 << 0),
    Alt = (1 << 1),
    Ctrl = (1 << 2),
    Shift = (1 << 3),
}
pub enum Action {
    Execute(&'static str),
    MoveWindow(Direction),
    SwitchWindow(Direction),
    MoveWorkspace(u8),
    SwitchWorkspace(u8),
    ChangeMode(Mode),
    Resize(Side, u8),
    Close,
}
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}
pub enum Side {
    Width,
    Height,
}
pub enum Mode {
    Till,
    Float,
    Stick,
}
