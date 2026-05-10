use bento_macros::Widget;
use bento_shared::{TextMeasureRequest, TextMeasurer, scene::Scene};

use crate::layout::{Overflow, Size};
use crate::widget::{Base, HasBase, Rect, Text, Widget};

#[derive(Widget)]
pub struct Button {
    pub base: Base,
    pub color: [f32; 4],
    pub label: String,
    pub font_size: f32,
    pub font_family: String,
    pub font_weight: u16,
    pub italic: bool,
    pub letter_spacing: f32,
    pub text_color: [f32; 4],
    pub padding_x: f32,
    pub padding_y: f32,
    pub border_color: [f32; 4],
    pub border_width: f32,
    pub radius: f32,
    pub opacity: f32,
    background: Rect,
    label_text: Text,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            base: Base::new(),
            color: [0.2, 0.5, 1.0, 1.0],
            label: label.to_string(),
            font_size: 16.0,
            font_family: String::new(),
            font_weight: 400,
            italic: false,
            letter_spacing: 0.0,
            text_color: [1.0, 1.0, 1.0, 1.0],
            padding_x: 16.0,
            padding_y: 8.0,
            border_color: [0.0; 4],
            border_width: 0.0,
            radius: 0.0,
            opacity: 1.0,
            background: Rect::new(),
            label_text: Text::new(label, 16.0),
        }
    }

    fn max_width_px(&self) -> Option<f32> {
        match &self.base.layout.max_width {
            Size::Px(v) => Some(*v),
            _ => None,
        }
    }
}

impl Widget for Button {
    fn build(&mut self, scene: &mut Scene) {
        self.background.build(scene);
        self.label_text.build(scene);
    }

    fn update(&mut self, scene: &mut Scene, measurer: &mut dyn TextMeasurer) {
        let l = &self.base.layout;
        let x = l.x;
        let y = l.y;
        let w = l.w;
        let h = l.h;

        let result = measurer.measure(TextMeasureRequest {
            text: &self.label,
            size: self.font_size,
            max_width: None,
            font_family: &self.font_family,
            weight: self.font_weight,
            italic: self.italic,
            letter_spacing: self.letter_spacing,
            line_height: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });

        self.background.base.layout.x = x;
        self.background.base.layout.y = y;
        self.background.base.layout.w = w;
        self.background.base.layout.h = h;
        self.background.color = self.color;
        self.background.border_color = self.border_color;
        self.background.border_widths = [self.border_width; 4];
        self.background.radii = [self.radius; 4];
        self.background.opacity = self.opacity;
        self.background.update(scene, measurer);

        println!("button x={} y={} w={} h={}", x, y, w, h);

        let text_x = x + self.padding_x;
        let text_y = y + (h - result.height) / 2.0;

        self.label_text.base.layout.x = text_x;
        self.label_text.base.layout.y = text_y;
        self.label_text.base.layout.max_width = match self.max_width_px() {
            Some(mw) => Size::Px(mw - self.padding_x * 2.0),
            None => Size::Auto,
        };
        self.label_text.text = self.label.clone();
        self.label_text.size = self.font_size;
        self.label_text.color = self.text_color;
        self.label_text.font_family = self.font_family.clone();
        self.label_text.weight = self.font_weight;
        self.label_text.italic = self.italic;
        self.label_text.letter_spacing = self.letter_spacing;
        self.label_text.opacity = self.opacity;
        self.label_text.update(scene, measurer);
    }

    fn measure(
        &self,
        _known_w: Option<f32>,
        _known_h: Option<f32>,
        measurer: &mut dyn TextMeasurer,
    ) -> (f32, f32) {
        let result = measurer.measure(TextMeasureRequest {
            text: &self.label,
            size: self.font_size,
            max_width: None,
            font_family: &self.font_family,
            weight: self.font_weight,
            italic: self.italic,
            letter_spacing: self.letter_spacing,
            line_height: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        (
            result.width + self.padding_x * 2.0,
            result.height + self.padding_y * 2.0,
        )
    }
}
