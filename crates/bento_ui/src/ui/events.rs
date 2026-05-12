use crate::Key;

/// Event structs

pub struct KeyPress { pub key: Key, pub ch: Option<char> }
pub struct KeyRelease { pub key: Key }
pub struct Click { pub x: f32, pub y: f32 }
pub struct Hover { }
