use crate::color::Color;
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::fonts::Fonts;
use crate::ui::Ui;

pub struct Label {
    pub layout: Layout,
    pub text: String,
    pub font_size: f32,
    pub font_weight: u16,
    pub font_italic: bool,
    pub text_color: Color,
    pub font_family: String,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            layout: Layout::default(),
            text: text.to_string(),
            font_size: 16.0,
            font_weight: 400,
            font_italic: false,
            text_color: Color::WHITE,
            font_family: "sans-serif".to_string(),
        }
    }

    pub fn measure(&self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)> {
        Some(fonts.measure_sized(
            &self.text,
            &self.font_family,
            self.font_size,
            self.font_weight,
            self.font_italic,
            max_width,
        ))
    }
}
