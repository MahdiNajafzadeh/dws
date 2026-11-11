pub mod action;
pub mod binding;
mod client;
pub mod color;
pub mod compositor;
pub mod input;
pub mod output;
pub mod startup;
mod state;

pub trait Config {
    const BORDER: Option<u8> = None;
    const BORDER_COLOR: color::Color = color::Color(0, 0, 0);
    const GAPS: Option<u8> = None;
    const GAPS_SMART: bool = false;
    const BINDING_LIST: &'static [binding::Binding] = &[];
    const INPUT_LIST: &'static [input::Input] = &[];
    const OUTPUT_LIST: &'static [output::Output] = &[];
    const STARTUP_LIST: &'static [startup::StartUp] = &[];
}
