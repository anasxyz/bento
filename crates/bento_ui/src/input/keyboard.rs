use std::fmt;
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}

impl Modifiers {
    pub fn none(&self) -> bool {
        !self.shift && !self.ctrl && !self.alt && !self.super_key
    }
}

#[derive(Debug, Default)]
pub struct Keyboard {
    pub modifiers: Modifiers,
    held: HashSet<Key>,
    just_pressed: Vec<(Key, Option<char>)>,
    just_released: Vec<Key>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_held(&self, key: &Key) -> bool {
        self.held.contains(key)
    }

    pub fn just_pressed(&self) -> &[(Key, Option<char>)] {
        &self.just_pressed
    }

    pub fn just_released(&self) -> &[Key] {
        &self.just_released
    }

    pub fn on_press(&mut self, key: Key, text: Option<char>) {
        self.held.insert(key.clone());
        self.just_pressed.push((key, text));
    }

    pub fn on_release(&mut self, key: Key) {
        self.held.remove(&key);
        self.just_released.push(key);
    }

    pub fn clear(&mut self) {
        println!("clear");
        self.just_pressed.clear();
        self.just_released.clear();
    }
}

impl fmt::Display for Keyboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Keyboard:")?;
        writeln!(f, "  held: {:?}", self.held)?;
        writeln!(f, "  just_pressed: {:?}", self.just_pressed)?;
        writeln!(f, "  just_released: {:?}", self.just_released)?;
        writeln!(f, "  modifiers: shift={} ctrl={} alt={} super={}", 
            self.modifiers.shift, self.modifiers.ctrl, 
            self.modifiers.alt, self.modifiers.super_key)
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Key {
    // letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // numbers
    Num0, Num1, Num2, Num3, Num4,
    Num5, Num6, Num7, Num8, Num9,

    // function keys
    F1, F2, F3, F4, F5, F6,
    F7, F8, F9, F10, F11, F12,

    // navigation
    Up, Down, Left, Right,
    Home, End, PageUp, PageDown,

    // editing
    Enter, Backspace, Delete, Tab, Escape,
    Insert, Space,

    // modifiers
    LShift, RShift,
    LCtrl, RCtrl,
    LAlt, RAlt,
    LSuper, RSuper,

    // symbols
    Minus, Equals, LeftBracket, RightBracket,
    Backslash, Semicolon, Apostrophe, Grave,
    Comma, Period, Slash,

    // other
    CapsLock, ScrollLock, NumLock,
    PrintScreen, Pause,

    Unknown,
}
