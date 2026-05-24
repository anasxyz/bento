use crate::{Widget, widget::Canvas};
use bento_shared::{CosmicTextMeasurer, TextAlign, TextMeasureRequest, TextMeasurer};
use bento_wgpu::{RectDraw, TextDraw};

pub struct Button {
    pub label_text: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub z: i32,
    pub padding: f32,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            label_text: text.to_string(),
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            color: [0.2, 0.2, 0.2, 1.0],
            z: 0,
            padding: 16.0,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        if self.label_text == text {
            return;
        }
        self.label_text = text.to_string();
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
}

impl Widget for Button {
    fn name(&self) -> &str {
        "Button"
    }

    fn update(&mut self, measurer: &mut CosmicTextMeasurer) {
        let result = measurer.measure(TextMeasureRequest {
            text: &self.label_text,
            font_family: "",
            size: 14.0,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            max_width: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        self.w = result.width + self.padding * 2.0;
        self.h = result.height + self.padding * 2.0;
    }

    fn size(&self) -> (f32, f32) {
        (self.w, self.h)
    }
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    fn z(&self) -> i32 {
        self.z
    }

    fn render(&self, canvas: &mut Canvas) {
        let lw = self.w - self.padding * 2.0;
        let lh = self.h - self.padding * 2.0;

        canvas.draw_list.push_rect(RectDraw {
            x: canvas.x,
            y: canvas.y,
            w: self.w,
            h: self.h,
            color: self.color,
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            rotate: canvas.rotate,
            scale_x: canvas.scale_x,
            scale_y: canvas.scale_y,
            opacity: canvas.opacity,
            clip: canvas.clip,
            z: canvas.z,
        });

        canvas.draw_list.push_text(TextDraw {
            x: canvas.x + (self.w - lw) / 2.0,
            y: canvas.y + (self.h - lh) / 2.0,
            w: lw,
            h: lh,
            text: self.label_text.clone(),
            size: 14.0,
            color: [1.0, 1.0, 1.0, 1.0],
            weight: 400,
            italic: false,
            font_family: String::new(),
            max_width: None,
            line_height: None,
            letter_spacing: 0.0,
            align: TextAlign::Left,
            opacity: canvas.opacity,
            clip: canvas.clip,
            rotate: canvas.rotate,
            scale_x: canvas.scale_x,
            scale_y: canvas.scale_y,
            z: canvas.z + 1,
            color_ranges: vec![],
            background_ranges: vec![],
            underline_ranges: vec![],
            strikethrough_ranges: vec![],
            weight_ranges: vec![],
            italic_ranges: vec![],
            font_family_ranges: vec![],
        });
    }
}
