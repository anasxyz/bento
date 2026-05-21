use crate::{Rect, Ui, Widget, widget::WidgetHandle};

pub struct Button {
    id: usize,
    bg: WidgetHandle<Rect>,
    rect: WidgetHandle<Rect>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            id: 0,
            bg: WidgetHandle::from_id(0),
            rect: WidgetHandle::from_id(0),
        }
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
}
