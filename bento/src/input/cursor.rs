#[derive(Debug, Clone, PartialEq, Default)]
pub enum Cursor {
    #[default]
    Default,
    Pointer,
    Text,
    Grab,
    Grabbing,
    Move,
    NotAllowed,
    CrossHair,
    Wait,
    Help,
}

pub fn map_cursor(c: &Cursor) -> winit::window::CursorIcon {
    match c {
        Cursor::Default => winit::window::CursorIcon::Default,
        Cursor::Pointer => winit::window::CursorIcon::Pointer,
        Cursor::Text => winit::window::CursorIcon::Text,
        Cursor::Grab => winit::window::CursorIcon::Grab,
        Cursor::Grabbing => winit::window::CursorIcon::Grabbing,
        Cursor::Move => winit::window::CursorIcon::Move,
        Cursor::NotAllowed => winit::window::CursorIcon::NotAllowed,
        Cursor::CrossHair => winit::window::CursorIcon::Crosshair,
        Cursor::Wait => winit::window::CursorIcon::Wait,
        Cursor::Help => winit::window::CursorIcon::Help,
    }
}
