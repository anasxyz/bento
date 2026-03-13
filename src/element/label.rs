use crate::color::Color;
use crate::element::element::Element;
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::fonts::Fonts;
use crate::ui::Ui;
use std::any::Any;

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
    pub fn new(ui: &mut Ui, text: &str) -> Handle<Self> {
        ui.add(Self {
            layout: Layout::default(),
            text: text.to_string(),
            font_size: 16.0,
            font_weight: 400,
            font_italic: false,
            text_color: Color::WHITE,
            font_family: "sans-serif".to_string(),
        })
    }
}

impl Element for Label {
    fn layout(&self) -> &Layout {
        &self.layout
    }
    fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }
    fn has_measure(&self) -> bool {
        true
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
