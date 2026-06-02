use crate::{Ui, View};
use crate::reactive::{Effect, Signal, effect, state};
use bento_wgpu::{DrawList, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};

pub struct Text {
    text: String,
}

impl Text {
    pub fn text(mut self, text: impl Fn() -> String + 'static) -> Self {
        self.text = text();
        self
    }
}

impl View for Text {
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        let r = measurer.measure(TextMeasureRequest {
            text: &self.text,
            font_family: "",
            size: 14.0,
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
        (r.width, r.height)
    }

    fn render(&self, x: f32, y: f32, measurer: &mut TextMeasurer, draw_list: &mut DrawList) {
        let (w, h) = self.measure(measurer);
        draw_list.push_text(TextDraw {
            x,
            y,
            w,
            h,
            text: self.text.clone(),
            size: 14.0,
            color: [1.0, 1.0, 1.0, 1.0],
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
        text: f(),
    }
}
