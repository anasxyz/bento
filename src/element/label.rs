use crate::color::Color;
use crate::element::layout::Layout;
use crate::fonts::Fonts;

pub struct Label {
    pub(crate) layout: Layout,
    pub(crate) dirty: bool,
    text: String,
    font_size: f32,
    font_weight: u16,
    font_italic: bool,
    text_color: Color,
    font_family: String,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            layout: Layout::default(),
            dirty: true,
            text: text.to_string(),
            font_size: 16.0,
            font_weight: 400,
            font_italic: false,
            text_color: Color::WHITE,
            font_family: "sans-serif".to_string(),
        }
    }

    // getters
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
    pub fn font_weight(&self) -> u16 {
        self.font_weight
    }
    pub fn font_italic(&self) -> bool {
        self.font_italic
    }
    pub fn text_color(&self) -> Color {
        self.text_color
    }
    pub fn font_family(&self) -> &str {
        &self.font_family
    }

    // setters
    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self.dirty = true;
        self
    }
    pub fn set_font_size(&mut self, size: f32) -> &mut Self {
        self.font_size = size;
        self.dirty = true;
        self
    }
    pub fn set_font_weight(&mut self, weight: u16) -> &mut Self {
        self.font_weight = weight;
        self.dirty = true;
        self
    }
    pub fn set_font_italic(&mut self, italic: bool) -> &mut Self {
        self.font_italic = italic;
        self.dirty = true;
        self
    }
    pub fn set_text_color(&mut self, color: Color) -> &mut Self {
        self.text_color = color;
        self.dirty = true;
        self
    }
    pub fn set_font_family(&mut self, family: &str) -> &mut Self {
        self.font_family = family.to_string();
        self.dirty = true;
        self
    }

    pub(crate) fn measure(&self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)> {
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
