pub trait Config {
    const BORDER: Option<u8>;
    const BORDER_COLOR: Color;
    const GAPS: Option<u8>;
    const GAPS_SMART: bool;
    // TODO: add binding config
    const BINDING_LIST: &[Binding];
    // TODO: add output config
    // const OUTPUT_LIST: &[Output];
    // TODO: add input config
    // const INPUT_LIST: &[Input];
}

pub struct Color(pub u8, pub u8, pub u8);
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
    Close,
}
