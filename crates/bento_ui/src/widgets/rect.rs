use crate::{Ui, View};
use crate::reactive::{Effect, Signal, effect, state};
use bento_wgpu::{DrawList, RectDraw, TextMeasurer};

pub struct Rect {
    color: Signal<[f32; 4]>,
    children: Vec<Box<dyn View>>,
    _effects: Vec<Effect>,
}

impl Rect {
    pub fn color(mut self, f: impl Fn() -> [f32; 4] + 'static) -> Self {
        let color = self.color;
        self._effects.push(effect(move || {
            color.set(f());
            Ui::request_redraw();
        }));
        self
    }

    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl View for Rect {
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        let mut w: f32 = 0.0;
        let mut h: f32 = 0.0;
        for child in &self.children {
            let (cw, ch) = child.measure(measurer);
            w = w.max(cw);
            h += ch;
        }
        (w, h)
    }

    fn render(&self, x: f32, y: f32, measurer: &mut TextMeasurer, draw_list: &mut DrawList) {
        let (w, h) = self.measure(measurer);
        draw_list.push_rect(RectDraw {
            x,
            y,
            w,
            h,
            color: self.color.get(),
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: None,
            z: 0,
        });
        let mut child_y = y;
        for child in &self.children {
            let (_, ch) = child.measure(measurer);
            child.render(x, child_y, measurer, draw_list);
            child_y += ch;
        }
    }
}

pub fn rect() -> Rect {
    Rect {
        color: state([0.0; 4]),
        children: Vec::new(),
        _effects: Vec::new(),
    }
}
