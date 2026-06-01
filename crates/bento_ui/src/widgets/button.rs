use crate::View;
use bento_wgpu::{DrawList, RectDraw, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};

pub struct Button {
    label: Box<dyn Fn() -> String>,
    color: Box<dyn Fn() -> [f32; 4]>,
    font_size: f32,
    padding: f32,
}

impl Button {
    pub fn color(mut self, f: impl Fn() -> [f32; 4] + 'static) -> Self {
        self.color = Box::new(f);
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
}

impl View for Button {
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        let label = (self.label)();
        let result = measurer.measure(TextMeasureRequest {
            text: &label,
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
        (
            result.width + self.padding * 2.0,
            result.height + self.padding * 2.0,
        )
    }

    fn render(&self, x: f32, y: f32, measurer: &mut TextMeasurer, draw_list: &mut DrawList) {
        let label = (self.label)();
        let result = measurer.measure(TextMeasureRequest {
            text: &label,
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
        let w = result.width + self.padding * 2.0;
        let h = result.height + self.padding * 2.0;
        draw_list.push_rect(RectDraw {
            x,
            y,
            w,
            h,
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
            x: x + self.padding,
            y: y + self.padding,
            w: result.width,
            h: result.height,
            text: label,
            size: self.font_size,
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
        font_size: 14.0,
        padding: 12.0,
    }
}
