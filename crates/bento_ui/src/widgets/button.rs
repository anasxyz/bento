use crate::{Ui, View};
use crate::reactive::{Effect, Signal, effect, state};
use bento_wgpu::{DrawList, RectDraw, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};

pub struct Button {
    label: Signal<String>,
    color: Signal<[f32; 4]>,
    font_size: f32,
    padding: f32,
    _effects: Vec<Effect>,
}

impl Button {
    pub fn color(mut self, f: impl Fn() -> [f32; 4] + 'static) -> Self {
        let color = self.color;
        self._effects.push(effect(move || {
            color.set(f());
            Ui::request_redraw();
        }));
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }
}

impl View for Button {
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        let label = self.label.get();
        let r = measurer.measure(TextMeasureRequest {
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
        (r.width + self.padding * 2.0, r.height + self.padding * 2.0)
    }

    fn render(&self, x: f32, y: f32, measurer: &mut TextMeasurer, draw_list: &mut DrawList) {
        let label = self.label.get();
        let (w, h) = self.measure(measurer);
        draw_list.push_rect(RectDraw {
            x,
            y,
            w,
            h,
            color: self.color.get(),
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
            w: w - self.padding * 2.0,
            h: h - self.padding * 2.0,
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

pub fn button(f: impl Fn() -> String + 'static) -> Button {
    let label = state(f());
    let eff = effect(move || {
        label.set(f());
        Ui::request_redraw();
    });
    Button {
        label,
        color: state([0.2, 0.2, 0.2, 1.0]),
        font_size: 14.0,
        padding: 12.0,
        _effects: vec![eff],
    }
}
