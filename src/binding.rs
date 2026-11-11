use crate::action::Action;
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Key: u64 {
        const SUPER        = 1 << 0;
        const ALT          = 1 << 1;
        const CTRL         = 1 << 2;
        const SHIFT        = 1 << 3;
        const RETURN       = 1 << 4;
        const ESCAPE       = 1 << 5;
        const TAB          = 1 << 6;
        const BACKSPACE    = 1 << 7;
        const SPACE        = 1 << 8;
        const HOME         = 1 << 9;
        const END          = 1 << 10;
        const PAGE_UP      = 1 << 11;
        const PAGE_DOWN    = 1 << 12;
        const DELETE       = 1 << 13;
        const INSERT       = 1 << 14;
        const PRINT_SCREEN = 1 << 15;
        const UP           = 1 << 16;
        const DOWN         = 1 << 17;
        const LEFT         = 1 << 18;
        const RIGHT        = 1 << 19;
        const A            = 1 << 20;
        const B            = 1 << 21;
        const C            = 1 << 22;
        const D            = 1 << 23;
        const E            = 1 << 24;
        const F            = 1 << 25;
        const G            = 1 << 26;
        const H            = 1 << 27;
        const I            = 1 << 28;
        const J            = 1 << 29;
        const K            = 1 << 30;
        const L            = 1 << 31;
        const M            = 1 << 32;
        const N            = 1 << 33;
        const O            = 1 << 34;
        const P            = 1 << 35;
        const Q            = 1 << 36;
        const R            = 1 << 37;
        const S            = 1 << 38;
        const T            = 1 << 39;
        const U            = 1 << 40;
        const V            = 1 << 41;
        const W            = 1 << 42;
        const X            = 1 << 43;
        const Y            = 1 << 44;
        const Z            = 1 << 45;
        const F1           = 1 << 46;
        const F2           = 1 << 47;
        const F3           = 1 << 48;
        const F4           = 1 << 49;
        const F5           = 1 << 50;
        const F6           = 1 << 51;
        const F7           = 1 << 52;
        const F8           = 1 << 53;
        const F9           = 1 << 54;
        const F10          = 1 << 55;
        const F11          = 1 << 56;
        const F12          = 1 << 57;
    }
}

pub struct Binding {
    pub key: Key,
    pub action: Action,
}
