use bento_wgpu::{DrawList, RectDraw, TextDraw, TextAlign};
use crate::View;

pub struct Button {
    label: Box<dyn Fn() -> String>,
    color: Box<dyn Fn() -> [f32; 4]>,
}

impl Button {
    pub fn color(mut self, f: impl Fn() -> [f32; 4] + 'static) -> Self {
        self.color = Box::new(f);
        self
    }
}

impl View for Button {
    fn render(&self, x: f32, y: f32, draw_list: &mut DrawList) {
        draw_list.push_rect(RectDraw {
            x,
            y,
            w: 120.0,
            h: 40.0,
            color: (self.color)(),
            radii: [4.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: None,
            z: 0,
        });
        draw_list.push_text(TextDraw {
            x: x + 10.0,
            y: y + 13.0,
            w: 100.0,
            h: 20.0,
            text: (self.label)(),
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
            z: 1,
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

pub fn button(label: impl Fn() -> String + 'static) -> Button {
    Button {
        label: Box::new(label),
        color: Box::new(|| [0.2, 0.2, 0.2, 1.0]),
    }
}
