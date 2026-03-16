use crate::color::Color;
use crate::element::layout::Layout;
use crate::fonts::Fonts;

pub struct Rect {
    pub(crate) layout: Layout,
    pub(crate) dirty: bool,
    bg_color: Color,
    border_color: Option<Color>,
    border_radius: Option<f32>,
    border_widths: [f32; 4], // [top, right, bottom, left]
    focused: bool,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
            dirty: true,
            bg_color: Color::BLACK,
            border_color: None,
            border_radius: None,
            border_widths: [0.0; 4],
            focused: false,
        }
    }

    // getters
    pub fn bg_color(&self) -> Color {
        self.bg_color
    }
    pub fn border_color(&self) -> Option<Color> {
        self.border_color
    }
    pub fn border_radius(&self) -> Option<f32> {
        self.border_radius
    }
    pub fn border_widths(&self) -> [f32; 4] {
        self.border_widths
    }
    pub fn focused(&self) -> bool {
        self.focused
    }

    // setters
    pub fn set_bg_color(&mut self, color: Color) -> &mut Self {
        self.bg_color = color;
        self.dirty = true;
        self
    }
    pub fn set_border_color(&mut self, color: Option<Color>) -> &mut Self {
        self.border_color = color;
        self.dirty = true;
        self
    }
    pub fn set_border_radius(&mut self, radius: Option<f32>) -> &mut Self {
        self.border_radius = radius;
        self.dirty = true;
        self
    }
    pub fn set_border(&mut self, widths: [f32; 4]) -> &mut Self {
        self.border_widths = widths;
        self.dirty = true;
        self
    }

    pub(crate) fn on_focus_gained(&mut self) {
        self.focused = true;
        self.dirty = true;
    }
    pub(crate) fn on_focus_lost(&mut self) {
        self.focused = false;
        self.dirty = true;
    }
}
