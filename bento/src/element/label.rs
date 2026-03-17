use crate::color::Color;
use crate::element::base::Base;
use crate::element::element::Element;
use crate::element::layout::Layout;
use crate::fonts::Fonts;
use crate::input::{Key, Modifiers, MouseButton};
use crate::render::DrawCall;
use bento_derive::Element;

#[derive(Element)]
pub struct Label {
    base: Base,
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
            base: Base::new(),
            text: text.to_string(),
            font_size: 16.0,
            font_weight: 400,
            font_italic: false,
            text_color: Color::WHITE,
            font_family: "sans-serif".to_string(),
        }
    }

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

    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self.base.dirty = true;
        self
    }
    pub fn set_font_size(&mut self, size: f32) -> &mut Self {
        self.font_size = size;
        self.base.dirty = true;
        self
    }
    pub fn set_font_weight(&mut self, weight: u16) -> &mut Self {
        self.font_weight = weight;
        self.base.dirty = true;
        self
    }
    pub fn set_font_italic(&mut self, italic: bool) -> &mut Self {
        self.font_italic = italic;
        self.base.dirty = true;
        self
    }
    pub fn set_text_color(&mut self, color: Color) -> &mut Self {
        self.text_color = color;
        self.base.dirty = true;
        self
    }
    pub fn set_font_family(&mut self, family: &str) -> &mut Self {
        self.font_family = family.to_string();
        self.base.dirty = true;
        self
    }
}

impl Element for Label {
    fn draw_calls(&self, clip: Option<[f32; 4]>, z: i32, opacity: f32) -> Vec<DrawCall> {
        let l = &self.base.layout;
        let mut color = self.text_color.to_array();
        color[3] *= opacity;
        vec![DrawCall::Text {
            x: l.x,
            y: l.y,
            content: self.text.clone(),
            family: self.font_family.clone(),
            size: self.font_size,
            weight: self.font_weight,
            italic: self.font_italic,
            color,
            width: if l.w > 0.0 { l.w } else { f32::MAX },
            clip,
            z_index: z,
        }]
    }

    fn measure(&self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)> {
        Some(fonts.measure_sized(
            &self.text,
            &self.font_family,
            self.font_size,
            self.font_weight,
            self.font_italic,
            max_width,
        ))
    }

    fn has_measure(&self) -> bool {
        true
    }
}
