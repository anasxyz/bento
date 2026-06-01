use crate::View;
use bento_wgpu::{DrawList, RectDraw, TextMeasurer};

pub struct Rect {
    color: Box<dyn Fn() -> [f32; 4]>,
}

impl Rect {
    pub fn color(mut self, f: impl Fn() -> [f32; 4] + 'static) -> Self {
        self.color = Box::new(f);
        self
    }
}

impl View for Rect {
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        let mut w: f32 = 0.0;
        let mut h: f32 = 0.0;
        (w, h)
    }

    fn render(&self, x: f32, y: f32, measurer: &mut TextMeasurer, draw_list: &mut DrawList) {
        let (w, h) = self.measure(measurer);
        draw_list.push_rect(RectDraw {
            x,
            y,
            w,
            h,
            color: (self.color)(),
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
    }
}

pub fn rect() -> Rect {
    Rect {
        color: Box::new(|| [0.0; 4]),
    }
}
