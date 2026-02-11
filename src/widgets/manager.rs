use crate::{Drawer, Fonts, MouseState, widgets::{ButtonWidget, SliderWidget, TextInputWidget, Widget, WidgetHandle}};

pub struct WidgetManager {
    pub(crate) widgets: Vec<Box<dyn Widget>>,
    next_id: usize,
    dirty: bool,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self { widgets: Vec::new(), next_id: 0, dirty: true }
    }

    fn alloc_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn button(&mut self, text: &str) -> WidgetHandle<ButtonWidget> {
        let id = self.alloc_id();
        self.widgets.push(Box::new(ButtonWidget::new(id, text)));
        self.dirty = true;
        WidgetHandle::new(id)
    }

    pub fn slider(&mut self) -> WidgetHandle<SliderWidget> {
        let id = self.alloc_id();
        self.widgets.push(Box::new(SliderWidget::new(id)));
        self.dirty = true;
        WidgetHandle::new(id)
    }

    pub fn text_input(&mut self) -> WidgetHandle<TextInputWidget> {
        let id = self.alloc_id();
        self.widgets.push(Box::new(TextInputWidget::new(id)));
        self.dirty = true;
        WidgetHandle::new(id)
    }

    pub(crate) fn type_char(&mut self, c: char) {
        for widget in self.widgets.iter_mut() {
            if let Some(input) = widget.as_any_mut().downcast_mut::<TextInputWidget>() {
                if input.focused {
                    input.insert_char(c);
                    self.dirty = true;
                }
            }
        }
    }

    pub(crate) fn key_press(&mut self, key: winit::keyboard::KeyCode) {
        use winit::keyboard::KeyCode;
        for widget in self.widgets.iter_mut() {
            if let Some(input) = widget.as_any_mut().downcast_mut::<TextInputWidget>() {
                if input.focused {
                    match key {
                        KeyCode::Backspace => { input.backspace(); self.dirty = true; }
                        KeyCode::Delete    => { input.delete_forward(); self.dirty = true; }
                        KeyCode::ArrowLeft  => { input.move_cursor_left(); self.dirty = true; }
                        KeyCode::ArrowRight => { input.move_cursor_right(); self.dirty = true; }
                        KeyCode::Home      => { input.move_cursor_home(); self.dirty = true; }
                        KeyCode::End       => { input.move_cursor_end(); self.dirty = true; }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn get<T: Widget + 'static>(&self, handle: WidgetHandle<T>) -> &T {
        for widget in self.widgets.iter() {
            if widget.id() == handle.id {
                return widget.as_any().downcast_ref::<T>()
                    .expect("widget type mismatch");
            }
        }
        panic!("widget with id {} not found", handle.id);
    }

    pub fn get_mut<T: Widget + 'static>(&mut self, handle: WidgetHandle<T>) -> &mut T {
        for widget in self.widgets.iter_mut() {
            if widget.id() == handle.id {
                return widget.as_any_mut().downcast_mut::<T>()
                    .expect("widget type mismatch");
            }
        }
        panic!("widget with id {} not found", handle.id);
    }

    pub(crate) fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub(crate) fn update_all(&mut self, mouse: &MouseState) -> bool {
        let mut changed = false;
        for widget in self.widgets.iter_mut() {
            widget.update(mouse);

            if mouse.left_just_pressed
                || mouse.left_just_released
                || mouse.right_just_pressed
                || mouse.right_just_released
                || widget.is_dirty()
            {
                changed = true;
            }
            widget.clear_dirty();
        }
        changed
    }

    pub(crate) fn take_dirty(&mut self) -> bool {
        let d = self.dirty;
        self.dirty = false;
        d
    }

    pub(crate) fn layout_all(&mut self, fonts: &mut Fonts) {
        for widget in self.widgets.iter_mut() {
            widget.layout(fonts);
        }
    }

    pub(crate) fn render_all(&mut self, drawer: &mut Drawer) {
        for widget in self.widgets.iter_mut() {
            widget.render(drawer);
        }
    }
}
