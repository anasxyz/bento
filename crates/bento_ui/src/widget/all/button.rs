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
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
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
        if let Some(bg) = ui.get_mut(self.bg) {
            bg.x = self.x;
            bg.y = self.y;
            bg.w = self.w;
            bg.h = self.h;
            bg.color = self.color;
        }

        if let Some(rect) = ui.get_mut(self.rect) {
            rect.x = self.x + 10.0;
            rect.y = self.y + 10.0;
            rect.w = self.w - 20.0;
            rect.h = self.h - 20.0;
            rect.color = [0.0, 0.0, 0.0, 1.0];
        }
    }
}
