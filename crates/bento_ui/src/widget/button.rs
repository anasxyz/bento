use crate::widget::{Rect, Text, Widget};
use bento_shared::{TextMeasureRequest, TextMeasurer, scene::Scene};

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub color: [f32; 4],
    pub label: String,
    pub font_size: f32,
    pub padding_x: f32,         // horizontal padding
    pub padding_y: f32,         // vertical padding
    pub max_width: Option<f32>, // constrain text width
    // computed
    pub w: f32,
    pub h: f32,
    background: Rect,
    label_text: Text,
}

impl Button {
    pub fn new(label: &str, x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            color: [0.2, 0.5, 1.0, 1.0],
            label: label.to_string(),
            font_size: 16.0,
            padding_x: 16.0,
            padding_y: 8.0,
            max_width: None,
            w: 0.0, // computed in update
            h: 0.0, // computed in update
            background: Rect::new(x, y, 0.0, 0.0),
            label_text: Text::new(label, x, y, 16.0),
        }
    }
}

impl Widget for Button {
    fn build(&mut self, scene: &mut Scene) {
        self.background.build(scene);
        self.label_text.build(scene);
    }

    fn update(&mut self, scene: &mut Scene, measurer: &mut dyn TextMeasurer) {
        // measure text with optional max width constraint
        let max_text_width = self.max_width.map(|mw| mw - self.padding_x * 2.0);

        let result = measurer.measure(TextMeasureRequest {
            text: &self.label,
            size: self.font_size,
            max_width: max_text_width,
            font_family: "",
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });

        // button size is text size + padding
        self.w = result.width + self.padding_x * 2.0;
        self.h = result.height + self.padding_y * 2.0;

        // constrain to max_width if set
        if let Some(mw) = self.max_width {
            self.w = self.w.min(mw);
        }

        // background fills the whole button
        self.background.x = self.x;
        self.background.y = self.y;
        self.background.w = self.w;
        self.background.h = self.h;
        self.background.color = self.color;
        self.background.update(scene, measurer);

        // text centered within button with padding as boundary
        let text_x = self.x + self.padding_x;
        let text_y = self.y + (self.h - result.height) / 2.0;

        self.label_text.x = text_x;
        self.label_text.y = text_y;
        self.label_text.text = self.label.clone();
        self.label_text.size = self.font_size;
        self.label_text.max_width = max_text_width;
        self.label_text.color = [1.0, 1.0, 1.0, 1.0];
        self.label_text.update(scene, measurer);
    }
}
