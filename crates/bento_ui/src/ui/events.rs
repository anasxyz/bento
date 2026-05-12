use crate::Key;
use crate::MouseButton;

/// Event structs

#[derive(Clone, Copy)]
pub struct KeyPress {
    pub key: Key,
    pub ch: Option<char>,
}

#[derive(Clone, Copy)]
pub struct KeyRelease {
    pub key: Key,
}

#[derive(Clone, Copy)]
pub struct Click {
    pub x: f32,
    pub y: f32,
    pub button: MouseButton,
}



#[derive(Clone, Copy)]
pub struct MouseMove {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Clone, Copy)]
pub struct MouseDown {
    pub x: f32,
    pub y: f32,
    pub button: MouseButton,
}

#[derive(Clone, Copy)]
pub struct MouseUp {
    pub x: f32,
    pub y: f32,
    pub button: MouseButton,
}

#[derive(Clone, Copy)]
pub struct MouseScroll {
    pub x: f32,
    pub y: f32,
}

pub struct MouseEnter;

pub struct MouseLeave;

pub struct HoverEnter;

pub struct HoverLeave;

pub struct FocusGained;

pub struct FocusLost;
