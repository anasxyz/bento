use crate::View;
use bento_wgpu::{DrawList, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};

pub struct Text {
    content: Box<dyn Fn() -> String>,
    font_size: f32,
    color: Box<dyn Fn() -> [f32; 4]>,
}

impl Text {
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn color(mut self, f: impl Fn() -> [f32; 4] + 'static) -> Self {
        self.color = Box::new(f);
        self
    }
}

impl View for Text {
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        let content = (self.content)();
        let result = measurer.measure(TextMeasureRequest {
            text: &content,
            font_family: "",
            size: self.font_size,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            tab_width: 4,
            max_width: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        (result.width, result.height)
    }

    fn render(&self, x: f32, y: f32, measurer: &mut TextMeasurer, draw_list: &mut DrawList) {
        let (w, h) = self.measure(measurer);
        let content = (self.content)();
        draw_list.push_text(TextDraw {
            x,
            y,
            w,
            h,
            text: content,
            size: self.font_size,
            color: (self.color)(),
            weight: 400,
            italic: false,
            font_family: String::new(),
            max_width: None,
            line_height: None,
            tab_width: 4,
            letter_spacing: 0.0,
            align: TextAlign::Left,
            opacity: 1.0,
            clip: None,
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            z: 0,
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

pub fn text(f: impl Fn() -> String + 'static) -> Text {
    Text {
        content: Box::new(f),
        font_size: 14.0,
        color: Box::new(|| [1.0, 1.0, 1.0, 1.0]),
    }
}
