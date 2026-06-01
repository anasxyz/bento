use crate::View;
use bento_wgpu::{DrawList, TextAlign, TextDraw};

pub struct Text {
    content: Box<dyn Fn() -> String>,
}

impl View for Text {
    fn render(&self, x: f32, y: f32, draw_list: &mut DrawList) {
        draw_list.push_text(TextDraw {
            x,
            y,
            w: 100.0,
            h: 20.0,
            text: (self.content)(),
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
        content: Box::new(f),
    }
}
