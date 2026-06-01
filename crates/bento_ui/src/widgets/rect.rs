use bento_wgpu::{DrawList, RectDraw};
use crate::View;

pub struct Rect {
    color: Box<dyn Fn() -> [f32; 4]>,
    children: Vec<Box<dyn View>>,
}

impl Rect {
    pub fn color(mut self, f: impl Fn() -> [f32; 4] + 'static) -> Self {
        self.color = Box::new(f);
        self
    }

    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl View for Rect {
    fn render(&self, x: f32, y: f32, draw_list: &mut DrawList) {
        draw_list.push_rect(RectDraw {
            x,
            y,
            w: 100.0,
            h: 100.0,
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
        for child in &self.children {
            child.render(x, y, draw_list);
        }
    }
}

pub fn rect() -> Rect {
    Rect {
        color: Box::new(|| [0.0; 4]),
        children: Vec::new(),
    }
}

