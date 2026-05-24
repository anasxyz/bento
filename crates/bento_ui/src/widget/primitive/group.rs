use crate::Ui;
use crate::acc::Accumulated;
use crate::widget::Widget;
use bento_wgpu::DrawList;

#[derive(Clone)]
pub enum Layout {
    None,
    Row { gap: f32 },
    Column { gap: f32 },
}

pub struct Group {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub layout: Layout,
    pub z: i32,
}

impl Group {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            scroll_x: 0.0,
            scroll_y: 0.0,
            layout: Layout::None,
            z: 0,
        }
    }

    pub fn set_x(&mut self, x: f32) {
        if self.x == x {
            return;
        }
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        if self.y == y {
            return;
        }
        self.y = y;
    }
    pub fn set_scroll_x(&mut self, x: f32) {
        if self.scroll_x == x {
            return;
        }
        self.scroll_x = x;
    }
    pub fn set_scroll_y(&mut self, y: f32) {
        if self.scroll_y == y {
            return;
        }
        self.scroll_y = y;
    }
    pub fn set_z(&mut self, z: i32) {
        if self.z == z {
            return;
        }
        self.z = z;
    }
}

impl Widget for Group {
    fn name(&self) -> &str {
        "Group"
    }
    fn size(&self) -> (f32, f32) {
        (self.w, self.h)
    }
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    fn z(&self) -> i32 {
        self.z
    }
}
