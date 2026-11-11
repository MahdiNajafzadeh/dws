#[derive(Clone)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone)]
pub enum Side {
    Width,
    Height,
}

#[derive(Clone)]
pub enum Mode {
    Till,
    Float,
    Stick,
}

#[derive(Clone)]
pub enum Action {
    Execute(&'static str),
    MoveWindow(Direction),
    SwitchWindow(Direction),
    MoveWorkspace(u8),
    SwitchWorkspace(u8),
    ChangeMode(Mode),
    Resize(Side, u8),
    Close,
    Reload,
    Exit,
}
