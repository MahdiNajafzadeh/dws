mod binding;
mod color;
mod input;
mod output;

pub trait Config {
    const BORDER: Option<u8> = None;
    const BORDER_COLOR: color::Color = color::Color(0, 0, 0);
    const GAPS: Option<u8> = None;
    const GAPS_SMART: bool = false;
    const BINDING_LIST: &[binding::Binding] = &[];
    // TODO: add input config
    // const INPUT_LIST: &[input::Input];
    // TODO: add output config
    // const OUTPUT_LIST: &[output::Output];
}
