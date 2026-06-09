use std::cell::{Cell, RefCell};
use std::rc::Rc;

use bento_wgpu::{DrawList, TextMeasurer};

use crate::Key;
use crate::events::KeyPress;
use crate::reactive::runtime;

use crate::{
    events,
    input::InputState,
    input::mouse,
    tree,
    views::{View, ViewId},
};

pub struct Ui {
    pub root: ViewId,
    pub measurer: TextMeasurer,
    pub input: InputState,
    pub viewport_w: f32,
    pub viewport_h: f32,
    pub pointer_capture: Option<ViewId>,
    pub focused: Option<ViewId>,
    pub layout_dirty: bool,
}

impl Ui {
    pub fn new(view: impl View + 'static, request_redraw: impl Fn() + 'static) -> Self {
        let request_redraw = Rc::new(request_redraw);
        set_redraw_fn(request_redraw.clone());

        let root = Box::new(view).build();

        request_redraw();

        Self {
            root,
            measurer: TextMeasurer::new(),
            input: InputState::new(),
            viewport_w: 800.0,
            viewport_h: 600.0,
            pointer_capture: None,
            focused: None,
            layout_dirty: true,
        }
    }

    pub fn set_viewport(&mut self, w: f32, h: f32) {
        self.viewport_w = w;
        self.viewport_h = h;
        self.layout_dirty = true;
        request_redraw();
    }

    pub fn draw(&mut self) -> DrawList {
        let mut draw_list = DrawList::new();

        let layout_dirty = self.layout_dirty || LAYOUT_DIRTY.with(|d| d.replace(false));

        if layout_dirty {
            let t = web_time::Instant::now();
            tree::layout(
                self.root,
                self.viewport_w,
                self.viewport_h,
                &mut self.measurer,
            );
            // println!("layout: {:?}", t.elapsed());
            self.layout_dirty = false;
        }

        tree::render(self.root, &mut draw_list, 0.0, 0.0, None);

        draw_list
    }

    pub fn process_input(&mut self) {
        self.keyboard_stuff();
        self.mouse_stuff();
    }

    pub fn keyboard_stuff(&mut self) {
        if self.input.keyboard.just_pressed().iter().any(|(k, _)| *k == Key::Equals) {
            tree::print_tree(self.root, 0);
        }

        for (key, ch) in self.input.keyboard.just_pressed().iter().cloned() {
            if let Some(id) = self.focused {
                tree::dispatch(id, &KeyPress { key, ch });
            }
        }
    }

    pub fn mouse_stuff(&mut self) {
        self.mouse_click_events();
        self.mouse_move_events();
        self.mouse_scroll_events();
    }

    pub fn mouse_click_events(&mut self) {
        if self.input.mouse.left.just_pressed {
            let new_focus = tree::hit_test(self.root, self.input.mouse.x, self.input.mouse.y, 0.0, 0.0);

            if self.focused != new_focus {
                if let Some(old_id) = self.focused {
                    tree::dispatch(old_id, &events::FocusLost);
                }
                if let Some(new_id) = new_focus {
                    tree::dispatch(new_id, &events::FocusGained);
                }
                self.focused = new_focus;
            }

            if let Some(id) = new_focus {
                tree::dispatch(id, &events::MouseDown {
                    x: self.input.mouse.x,
                    y: self.input.mouse.y,
                    button: mouse::MouseButton::Left,
                });
                self.pointer_capture = Some(id);
            } else {
                self.focused = None;
            }
        }

        if self.input.mouse.left.just_released {
            if let Some(id) = self.pointer_capture {
                tree::dispatch(id, &events::MouseUp {
                    x: self.input.mouse.x,
                    y: self.input.mouse.y,
                    button: mouse::MouseButton::Left,
                });
                tree::dispatch(id, &events::Click {
                    x: self.input.mouse.x,
                    y: self.input.mouse.y,
                    button: mouse::MouseButton::Left,
                });
            }
            self.pointer_capture = None;
        }
    }

    pub fn mouse_move_events(&mut self) {
        if self.input.mouse.dx != 0.0 || self.input.mouse.dy != 0.0 {
            let target = self.pointer_capture.or_else(|| {
                tree::hit_test(self.root, self.input.mouse.x, self.input.mouse.y, 0.0, 0.0)
            });
            if let Some(id) = target {
                tree::dispatch(
                    id,
                    &events::MouseMove {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        dx: self.input.mouse.dx,
                        dy: self.input.mouse.dy,
                    },
                );
            }
        }
    }

    pub fn mouse_scroll_events(&mut self) {
        if self.input.mouse.scroll_x != 0.0 || self.input.mouse.scroll_y != 0.0 {
            if let Some(id) =
                tree::hit_test(self.root, self.input.mouse.x, self.input.mouse.y, 0.0, 0.0)
            {
                tree::dispatch(
                    id,
                    &events::MouseScroll {
                        x: self.input.mouse.scroll_x,
                        y: self.input.mouse.scroll_y,
                    },
                );
            }
        }
    }
}

thread_local! {
    static REDRAW_FN: RefCell<Option<Rc<dyn Fn()>>> = RefCell::new(None);
    static LAYOUT_DIRTY: Cell<bool> = Cell::new(false);
}

pub(crate) fn set_redraw_fn(f: Rc<dyn Fn()>) {
    REDRAW_FN.with(|r| *r.borrow_mut() = Some(f));
}

pub(crate) fn request_redraw() {
    REDRAW_FN.with(|r| {
        if let Some(f) = r.borrow().as_ref() {
            f();
        }
    });
}

pub(crate) fn request_layout() {
    LAYOUT_DIRTY.with(|d| d.set(true));
    request_redraw();
}
