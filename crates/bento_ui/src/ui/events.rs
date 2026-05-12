use crate::Key;
use crate::MouseButton;

/// Event structs

pub struct KeyPress {
    pub key: Key,
    pub ch: Option<char>,
}

pub struct KeyRelease {
    pub key: Key,
}

pub struct Click {
    pub x: f32,
    pub y: f32,
    pub button: MouseButton,
}

pub struct Hover {}

pub struct MouseMove {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

pub struct MouseDown {
    pub x: f32,
    pub y: f32,
    pub button: MouseButton,
}

pub struct MouseUp {
    pub x: f32,
    pub y: f32,
    pub button: MouseButton,
}

pub struct MouseScroll {
    pub x: f32,
    pub y: f32,
}

pub struct MouseEnter;

pub struct MouseLeave;

pub struct FocusGained;

pub struct FocusLost;
