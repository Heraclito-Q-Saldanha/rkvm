mod button;
mod key;

pub use button::Button;
pub use key::Key;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

pub type EventPack = SmallVec<[Event; 2]>;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Event {
    MouseScroll { axis: Axis, delta: i32 },
    MouseMove { axis: Axis, delta: i32 },
    Key { direction: Direction, kind: KeyKind },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Direction {
    Up,   // The key is released.
    Down, // The key is pressed.
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum KeyKind {
    Key(Key),
    Button(Button),
}

impl KeyKind {
    pub(crate) fn from_raw(code: u32) -> Option<KeyKind> {
        Key::from_raw(code)
            .map(KeyKind::Key)
            .or_else(|| Button::from_raw(code).map(KeyKind::Button))
    }

    pub(crate) fn to_raw(&self) -> u32 {
        match self {
            KeyKind::Key(key) => key.to_raw(),
            KeyKind::Button(button) => button.to_raw(),
        }
    }
}
