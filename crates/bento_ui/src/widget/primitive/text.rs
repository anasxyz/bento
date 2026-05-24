use crate::layout::Size;
use crate::widget::{Canvas, Widget};
use bento_wgpu::{TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};

pub struct Text {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub width: Size,
    pub height: Size,
    pub content: String,
    pub size: f32,
    pub color: [f32; 4],
    pub z: i32,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            width: Size::Auto,
            height: Size::Auto,
            content: content.to_string(),
            size: 14.0,
            color: [1.0, 1.0, 1.0, 1.0],
            z: 0,
        }
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }
    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
}

impl Widget for Text {
    fn name(&self) -> &str {
        "Text"
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
    fn update(&mut self, measurer: &mut TextMeasurer) {
        let max_width = match &self.width {
            Size::Fixed(w) => Some(*w),
            Size::Auto => None,
            _ => None,
        };
        let result = measurer.measure(TextMeasureRequest {
            text: &self.content,
            font_family: "",
            size: self.size,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            max_width,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        self.w = match &self.width {
            Size::Auto => result.width,
            Size::Fixed(w) => *w,
            _ => result.width,
        };
        self.h = match &self.height {
            Size::Auto => result.height,
            Size::Fixed(h) => *h,
            _ => result.height,
        };
    }
    fn render(&self, canvas: &mut Canvas) {
        canvas.draw_list.push_text(TextDraw {
            x: canvas.x,
            y: canvas.y,
            w: self.w,
            h: self.h,
            text: self.content.clone(),
            size: self.size,
            color: self.color,
            weight: 400,
            italic: false,
            font_family: String::new(),
            max_width: if matches!(self.width, Size::Auto) {
                None
            } else {
                Some(self.w)
            },
            line_height: None,
            letter_spacing: 0.0,
            align: TextAlign::Left,
            opacity: canvas.opacity,
            clip: canvas.clip,
            rotate: canvas.rotate,
            scale_x: canvas.scale_x,
            scale_y: canvas.scale_y,
            z: canvas.z,
            color_ranges: vec![],
            background_ranges: vec![],
            underline_ranges: vec![],
            strikethrough_ranges: vec![],
            weight_ranges: vec![],
            italic_ranges: vec![],
            font_family_ranges: vec![],
        });
    }
    fn set_size(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
    }
    fn width_sizing(&self) -> &Size {
        &self.width
    }
    fn height_sizing(&self) -> &Size {
        &self.height
    }
}
