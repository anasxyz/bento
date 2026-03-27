use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Default)]
pub struct Modifiers {
    pub shift:     bool,
    pub ctrl:      bool,
    pub alt:       bool,
    pub cmd:       bool,
    pub super_key: bool,
}

impl Modifiers {
    pub fn none(&self) -> bool {
        !self.shift && !self.ctrl && !self.cmd && !self.alt && !self.super_key
    }
}

/// keyboard state
#[derive(Debug, Default)]
pub struct KeyState {
    pub modifiers:      Modifiers,
    held:               HashSet<Key>,
    just_pressed:       Vec<(Key, Option<char>)>,
    just_released:      Vec<Key>,
}

impl KeyState {
    pub fn new() -> Self { Self::default() }

    pub fn is_held(&self, key: &Key) -> bool {
        self.held.contains(key)
    }

    pub fn just_pressed(&self) -> &[(Key, Option<char>)] {
        &self.just_pressed
    }

    pub fn just_released(&self) -> &[Key] {
        &self.just_released
    }

    pub(crate) fn on_press(&mut self, key: Key, text: Option<char>) {
        self.held.insert(key.clone());
        self.just_pressed.push((key, text));
    }

    pub(crate) fn on_release(&mut self, key: Key) {
        self.held.remove(&key);
        self.just_released.push(key);
    }

    pub(crate) fn reset(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
}

impl From<winit::keyboard::KeyCode> for Key {
    fn from(k: winit::keyboard::KeyCode) -> Self {
        use winit::keyboard::KeyCode::*;
        match k {
            KeyA => Key::A, KeyB => Key::B, KeyC => Key::C, KeyD => Key::D,
            KeyE => Key::E, KeyF => Key::F, KeyG => Key::G, KeyH => Key::H,
            KeyI => Key::I, KeyJ => Key::J, KeyK => Key::K, KeyL => Key::L,
            KeyM => Key::M, KeyN => Key::N, KeyO => Key::O, KeyP => Key::P,
            KeyQ => Key::Q, KeyR => Key::R, KeyS => Key::S, KeyT => Key::T,
            KeyU => Key::U, KeyV => Key::V, KeyW => Key::W, KeyX => Key::X,
            KeyY => Key::Y, KeyZ => Key::Z,

            Digit0 => Key::Num0, Digit1 => Key::Num1, Digit2 => Key::Num2,
            Digit3 => Key::Num3, Digit4 => Key::Num4, Digit5 => Key::Num5,
            Digit6 => Key::Num6, Digit7 => Key::Num7, Digit8 => Key::Num8,
            Digit9 => Key::Num9,

            F1 => Key::F1,   F2 => Key::F2,   F3 => Key::F3,  F4 => Key::F4,
            F5 => Key::F5,   F6 => Key::F6,   F7 => Key::F7,  F8 => Key::F8,
            F9 => Key::F9,   F10 => Key::F10, F11 => Key::F11, F12 => Key::F12,

            ArrowUp    => Key::Up,     ArrowDown  => Key::Down,
            ArrowLeft  => Key::Left,   ArrowRight => Key::Right,
            Home       => Key::Home,   End        => Key::End,
            PageUp     => Key::PageUp, PageDown   => Key::PageDown,

            Enter     => Key::Enter,     Backspace => Key::Backspace,
            Delete    => Key::Delete,    Tab       => Key::Tab,
            Escape    => Key::Escape,    Insert    => Key::Insert,
            Space     => Key::Space,

            ShiftLeft    => Key::LShift,  ShiftRight   => Key::RShift,
            ControlLeft  => Key::LCtrl,   ControlRight => Key::RCtrl,
            AltLeft      => Key::LAlt,    AltRight     => Key::RAlt,
            SuperLeft    => Key::LSuper,  SuperRight   => Key::RSuper,

            Minus       => Key::Minus,        Equal       => Key::Equals,
            BracketLeft => Key::LeftBracket,  BracketRight => Key::RightBracket,
            Backslash   => Key::Backslash,    Semicolon   => Key::Semicolon,
            Quote       => Key::Apostrophe,   Backquote   => Key::Grave,
            Comma       => Key::Comma,        Period      => Key::Period,
            Slash       => Key::Slash,

            CapsLock    => Key::CapsLock,   ScrollLock => Key::ScrollLock,
            NumLock     => Key::NumLock,    PrintScreen => Key::PrintScreen,
            Pause       => Key::Pause,

            _ => Key::Unknown,
        }
    }
}
