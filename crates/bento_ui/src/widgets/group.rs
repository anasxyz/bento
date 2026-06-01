use crate::View;
use bento_wgpu::{DrawList, RectDraw, TextMeasurer};

pub struct Group {
    color: Option<Box<dyn Fn() -> [f32; 4]>>,
    children: Vec<Box<dyn View>>,
}

impl Group {
    pub fn color(mut self, f: impl Fn() -> [f32; 4] + 'static) -> Self {
        self.color = Some(Box::new(f));
        self
    }

    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl View for Group {
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
        if let Some(color) = &self.color {
            draw_list.push_rect(RectDraw {
                x,
                y,
                w,
                h,
                color: color(),
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
        let mut child_y = y;
        for child in &self.children {
            let (_, ch) = child.measure(measurer);
            child.render(x, child_y, measurer, draw_list);
            child_y += ch;
        }
    }
}

pub fn group() -> Group {
    Group {
        color: None,
        children: Vec::new(),
    }
}
