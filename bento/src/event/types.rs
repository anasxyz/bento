use crate::input::{Key, Modifiers};

#[derive(Clone, Debug)]
pub enum Event {
    Click { x: f32, y: f32 },
    RightClick { x: f32, y: f32 },
    DoubleClick { x: f32, y: f32 },
    Press { x: f32, y: f32 },
    Release { x: f32, y: f32 },
    MouseMove { x: f32, y: f32 },
    Scroll { x: f32, y: f32 },
    Hover,
    HoverEnd,
    FocusGained,
    FocusLost,
    KeyPress { key: Key, mods: Modifiers, text: Option<char> },
    KeyRelease { key: Key, mods: Modifiers },
    Change(String),
    Custom(u32),
}
