use anyhow::Result;
use dws::Config;
use dws::action::Action;
use dws::binding::Binding;
use dws::binding::Key;
use dws::compositor::Compositor;
use dws::startup::StartUp;

struct Simple;
impl Config for Simple {
    const BINDING_LIST: &'static [Binding] =
        //
        &[
            //
            Binding {
                key: Key::RETURN,
                action: Action::Execute("foot"),
            },
        ];
    const STARTUP_LIST: &'static [StartUp] =
        //
        &[
            //
            StartUp { command: "waybar" },
        ];
}

fn main() -> Result<()> {
    Compositor::<Simple>::start()
}
