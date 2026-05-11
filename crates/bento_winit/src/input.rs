use bento_ui::Key;
use winit::keyboard::KeyCode::*;

pub fn keycode_to_key(k: winit::keyboard::KeyCode) -> bento_ui::Key {
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

