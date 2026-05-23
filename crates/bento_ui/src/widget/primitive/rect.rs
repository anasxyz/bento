use bento_shared::{RectNode, SceneNode, SceneNodeId};
use bento_wgpu::{DrawList, RectDraw};

use crate::{Ui, acc::Accumulated, widget::Widget};

pub struct Rect {
    id: usize,

    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub z: i32,

    dirty: bool,
}

impl Rect {
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            id: 0,

            x: 0.0,
            y: 0.0,
            w,
            h,
            color: [0.0, 0.0, 0.0, 1.0],
            z: 0,

            dirty: true,
        }
    }

    pub fn set_x(&mut self, x: f32) {
        if self.x == x {
            return;
        }
        self.x = x;
        self.dirty = true;
    }
    pub fn set_y(&mut self, y: f32) {
        if self.y == y {
            return;
        }
        self.y = y;
        self.dirty = true;
    }
    pub fn set_w(&mut self, w: f32) {
        if self.w == w {
            return;
        }
        self.w = w;
        self.dirty = true;
    }
    pub fn set_h(&mut self, h: f32) {
        if self.h == h {
            return;
        }
        self.h = h;
        self.dirty = true;
    }
    pub fn set_color(&mut self, color: [f32; 4]) {
        if self.color == color {
            return;
        }
        self.color = color;
        self.dirty = true;
    }
    pub fn set_z(&mut self, z: i32) {
        if self.z == z {
            return;
        }
        self.z = z;
        self.dirty = true;
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

    fn build(&mut self, ui: &mut Ui) {}

    fn update(&mut self, ui: &mut Ui) {}

    fn remove(&mut self, ui: &mut Ui) {}

    fn hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn render(&self, draw_list: &mut DrawList, acc: &Accumulated) {
        draw_list.push_rect(
            RectDraw {
                x: acc.offset_x,
                y: acc.offset_y,
                w: self.w,
                h: self.h,
                color: self.color,
                radii: [0.0; 4],
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                rotate: acc.rotate,
                scale_x: acc.scale_x,
                scale_y: acc.scale_y,
                opacity: acc.opacity,
                clip: acc.clip,
                z: acc.z,
            },
        );
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.set_x(x);
        self.set_y(y);
    }

    fn render_offset(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn z(&self) -> i32 {
        self.z
    }
}
