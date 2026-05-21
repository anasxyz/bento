use crate::{Rect, Ui, Widget, widget::WidgetHandle};

pub struct Button {
    id: usize,
    bg: WidgetHandle<Rect>,
    rect: WidgetHandle<Rect>,

    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],

    dirty: bool,
}

impl Button {
    pub fn new() -> Self {
        Self {
            id: 0,
            bg: WidgetHandle::from_id(0),
            rect: WidgetHandle::from_id(0),

            x: 0.0,
            y: 0.0,
            w: 400.0,
            h: 200.0,
            color: [0.0, 0.0, 0.0, 1.0],

            dirty: false,
        }
    }

    pub fn set_x(&mut self, x: f32) {
        if self.x == x { return; }
        self.x = x;
        self.dirty = true;
    }
    pub fn set_y(&mut self, y: f32) {
        if self.y == y { return; }
        self.y = y;
        self.dirty = true;
    }
    pub fn set_w(&mut self, w: f32) {
        if self.w == w { return; }
        self.w = w;
        self.dirty = true;
    }
    pub fn set_h(&mut self, h: f32) {
        if self.h == h { return; }
        self.h = h;
        self.dirty = true;
    }
    pub fn set_color(&mut self, color: [f32; 4]) {
        if self.color == color { return; }
        self.color = color;
        self.dirty = true;
    }
}

impl Widget for Button {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn name(&self) -> &str {
        "Button"
    }

    fn build(&mut self, ui: &mut Ui) {
        self.bg = ui.add_child(self, Rect::new());
        self.rect = ui.add_child(self, Rect::new());
    }

    fn update(&mut self, ui: &mut Ui) {
        println!("updating button");
        let bg = ui.get_mut(self.bg);
        if let Some(bg) = ui.get_mut(self.bg) {
            bg.set_x(self.x);
            bg.set_y(self.y);
            bg.set_w(self.w);
            bg.set_h(self.h);
            bg.set_color(self.color);
        }
        if let Some(rect) = ui.get_mut(self.rect) {
            rect.set_x(self.x + 10.0);
            rect.set_y(self.y + 10.0);
            rect.set_w(self.w - 20.0);
            rect.set_h(self.h - 20.0);
            rect.set_color([0.0, 0.0, 0.0, 1.0]);
        }
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
}
