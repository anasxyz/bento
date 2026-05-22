use crate::Ui;
use crate::accumulated::Accumulated;
use crate::widget::Widget;
use bento_wgpu::DrawList;

#[derive(Clone)]
pub enum Layout {
    None,
    Row { gap: f32 },
    Column { gap: f32 },
}

pub struct Group {
    id: usize,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub layout: Layout,
    dirty: bool,
}

impl Group {
    pub fn new() -> Self {
        Self {
            id: 0,
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            scroll_x: 0.0,
            scroll_y: 0.0,
            layout: Layout::None,
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
    pub fn set_scroll_x(&mut self, x: f32) {
        if self.scroll_x == x {
            return;
        }
        self.scroll_x = x;
        self.dirty = true;
    }
    pub fn set_scroll_y(&mut self, y: f32) {
        if self.scroll_y == y {
            return;
        }
        self.scroll_y = y;
        self.dirty = true;
    }
}

impl Widget for Group {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn name(&self) -> &str {
        "Group"
    }
    fn hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
    fn render_offset(&self) -> (f32, f32) {
        (self.scroll_x, self.scroll_y)
    }
    fn update(&mut self, ui: &mut Ui) {}
    fn render(&self, draw_list: &mut DrawList, acc: &Accumulated) {}
    fn set_position(&mut self, x: f32, y: f32) {
        self.set_x(x);
        self.set_y(y);
    }
}
