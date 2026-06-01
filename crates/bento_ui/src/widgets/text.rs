use crate::{Ui, View};
use crate::reactive::{Effect, Signal, effect, state};
use bento_wgpu::{DrawList, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};

pub struct Text {
    content: Signal<String>,
    color: Signal<[f32; 4]>,
    font_size: f32,
    _effects: Vec<Effect>,
}

impl Text {
    pub fn color(mut self, f: impl Fn() -> [f32; 4] + 'static) -> Self {
        let color = self.color;
        self._effects.push(effect(move || color.set(f())));
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
}

impl View for Text {
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        let content = self.content.get();
        let r = measurer.measure(TextMeasureRequest {
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
        (r.width, r.height)
    }

    fn render(&self, x: f32, y: f32, measurer: &mut TextMeasurer, draw_list: &mut DrawList) {
        let (w, h) = self.measure(measurer);
        draw_list.push_text(TextDraw {
            x,
            y,
            w,
            h,
            text: self.content.get(),
            size: self.font_size,
            color: self.color.get(),
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
    let content = state(f());

    let eff = effect(move || {
        content.set(f());
        Ui::request_redraw();
    });

    Text {
        content,
        color: state([1.0, 1.0, 1.0, 1.0]),
        font_size: 14.0,
        _effects: vec![eff],
    }
}
