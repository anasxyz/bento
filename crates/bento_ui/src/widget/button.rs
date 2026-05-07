use crate::widget::{Rect, Text, Widget};
use bento_shared::{
    TextMeasureRequest, TextMeasurer, scene::{Scene}
};

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub label: String,
    pub font_size: f32,
    background: Rect,
    label_text: Text,
}

impl Button {
    pub fn new(label: &str, x: f32, y: f32, w: f32, h: f32) -> Self {
        let mut background = Rect::new(x, y, w, h);
        background.color = [0.2, 0.5, 1.0, 1.0];

        let mut label_text = Text::new(label, x, y, 16.0);
        label_text.color = [1.0, 1.0, 1.0, 1.0];

        Self {
            x,
            y,
            w,
            h,
            color: [0.2, 0.5, 1.0, 1.0],
            label: label.to_string(),
            font_size: 16.0,
            background,
            label_text,
        }
    }
}

impl Widget for Button {
    fn build(&mut self, scene: &mut Scene) {
        self.background.build(scene);
        self.label_text.build(scene);
    }

    fn update(&mut self, scene: &mut Scene, measurer: &mut dyn TextMeasurer) {
        self.background.x = self.x;
        self.background.y = self.y;
        self.background.w = self.w;
        self.background.h = self.h;
        self.background.color = self.color;
        self.background.update(scene, measurer);

        let result = measurer.measure(TextMeasureRequest {
            text: &self.label,
            size: self.font_size,
            max_width: None,
            font_family: "",
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });

        let text_x = self.x + (self.w - result.width) / 2.0;
        let text_y = self.y + (self.h - result.height) / 2.0;

        self.label_text.x = text_x;
        self.label_text.y = text_y;
        self.label_text.text = self.label.clone();
        self.label_text.size = self.font_size;
        self.label_text.update(scene, measurer);
    }
}
