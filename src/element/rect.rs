use crate::color::Color;
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::fonts::Fonts;
use crate::mouse::MouseButton;
use crate::ui::Ui;

pub struct Rect {
    pub layout: Layout,
    pub bg_color: Color,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,
    pub border_thickness: f32,
    pub focused: bool,
}

impl Rect {
    pub const FOCUS_GAINED: u32 = 0;
    pub const FOCUS_LOST: u32 = 1;

    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
            bg_color: Color::new(0.0, 0.0, 0.0, 1.0),
            border_color: None,
            border_radius: None,
            border_thickness: 0.0,
            focused: false,
        }
    }

    pub fn on_focus_gained(&mut self) -> Option<u32> {
        self.focused = true;
        Some(Self::FOCUS_GAINED)
    }

    pub fn on_focus_lost(&mut self) -> Option<u32> {
        self.focused = false;
        Some(Self::FOCUS_LOST)
    }
}
