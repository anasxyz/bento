use crate::{
    Ui,
    widgets::{ButtonWidget, Widget, WidgetHandle},
};

pub struct WidgetManager {
    widgets: Vec<Box<dyn Widget>>,
    next_id: usize,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            next_id: 0,
        }
    }

    fn alloc_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add<T: Widget + 'static>(&mut self, widget: T) -> WidgetHandle<T> {
        let handle = WidgetHandle::new(widget.id());
        self.widgets.push(Box::new(widget));
        handle
    }

    pub fn button(&mut self, text: &str) -> WidgetHandle<ButtonWidget> {
        let id = self.alloc_id();
        self.widgets.push(Box::new(ButtonWidget::new(id, text)));
        WidgetHandle::new(id)
    }

    pub fn get_mut<T: Widget + 'static>(&mut self, handle: WidgetHandle<T>) -> &mut T {
        for widget in self.widgets.iter_mut() {
            if widget.id() == handle.id {
                return widget
                    .as_any_mut()
                    .downcast_mut::<T>()
                    .expect("widget type mismatch — this should never happen with a valid handle");
            }
        }
        panic!(
            "widget with id {} not found — handle outlived widget",
            handle.id
        );
    }

    pub fn try_get_mut<T: Widget + 'static>(&mut self, handle: WidgetHandle<T>) -> Option<&mut T> {
        for widget in self.widgets.iter_mut() {
            if widget.id() == handle.id {
                return widget.as_any_mut().downcast_mut::<T>();
            }
        }
        None
    }

    pub(crate) fn render_all(&self, ui: &mut Ui) {
        for widget in &self.widgets {
            widget.render(ui);
        }
    }
}
