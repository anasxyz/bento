use bento_shared::RectNode;

use crate::{Ui, widget::Widget};

pub struct Rect {
    id: usize,

    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

    pub color: [f32; 4],
}

impl Rect {
    pub fn new() -> Self {
        Self {
            id: 0,
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn set_w(&mut self, w: f32) {
        self.w = w;
    }
    pub fn set_h(&mut self, h: f32) {
        self.h = h;
    }
}

impl Widget for Rect {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn name(&self) -> &str {
        "Rect"
    }

    fn build(&mut self, ui: &mut Ui) {
        println!("building rect");
    }

    fn hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }
}
