use std::cell::{Cell, RefCell};
use std::rc::Rc;

use bento_wgpu::{DrawList, TextMeasurer};

use crate::Key;
use crate::reactive::runtime;

use crate::{
    events,
    input::InputState,
    input::mouse,
    tree,
    view::{View, ViewId},
};

pub struct Ui {
    pub root: ViewId,
    pub measurer: TextMeasurer,
    pub input: InputState,
    pub viewport_w: f32,
    pub viewport_h: f32,
    pub pointer_capture: Option<ViewId>,
}

impl Ui {
    pub fn new(view: impl View + 'static, request_redraw: impl Fn() + 'static) -> Self {
        let request_redraw = Rc::new(request_redraw);
        set_redraw_fn(request_redraw.clone());

        let root = Box::new(view).build();

        Self {
            root,
            measurer: TextMeasurer::new(),
            input: InputState::new(),
            viewport_w: 800.0,
            viewport_h: 600.0,
            pointer_capture: None,
        }
    }

    pub fn set_viewport(&mut self, w: f32, h: f32) {
        self.viewport_w = w;
        self.viewport_h = h;
        request_redraw();
    }

    pub fn draw(&mut self) -> DrawList {
        println!("---------------");
        let mut draw_list = DrawList::new();

        let t = web_time::Instant::now();
        tree::layout(
            self.root,
            self.viewport_w,
            self.viewport_h,
            &mut self.measurer,
        );
        println!("[ui] layout took {:?}", t.elapsed());

        tree::render(self.root, &mut draw_list);

        println!("---------------");

        draw_list
    }

    pub fn process_input(&mut self) {
        self.keyboard_stuff();
        self.mouse_stuff();
    }

    pub fn keyboard_stuff(&mut self) {
        if self
            .input
            .keyboard
            .just_pressed()
            .iter()
            .any(|(k, _)| *k == Key::D)
        {
            tree::print_tree(self.root, 0);
        }
    }

    pub fn mouse_stuff(&mut self) {
        self.mouse_click_events();
        self.mouse_move_events();
        self.mouse_scroll_events();
    }

    pub fn mouse_click_events(&mut self) {
        if self.input.mouse.left.just_pressed {
            if let Some(id) = tree::hit_test(self.root, self.input.mouse.x, self.input.mouse.y) {
                tree::dispatch(
                    id,
                    &events::MouseDown {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        button: mouse::MouseButton::Left,
                    },
                );
                self.pointer_capture = Some(id);
            }
        }
        if self.input.mouse.left.just_released {
            if let Some(id) = self.pointer_capture {
                tree::dispatch(
                    id,
                    &events::MouseUp {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        button: mouse::MouseButton::Left,
                    },
                );
                tree::dispatch(
                    id,
                    &events::Click {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        button: mouse::MouseButton::Left,
                    },
                );
            }
            self.pointer_capture = None;
        }
    }

    pub fn mouse_move_events(&mut self) {
        if self.input.mouse.dx != 0.0 || self.input.mouse.dy != 0.0 {
            let target = self
                .pointer_capture
                .or_else(|| tree::hit_test(self.root, self.input.mouse.x, self.input.mouse.y));
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
            if let Some(id) = tree::hit_test(self.root, self.input.mouse.x, self.input.mouse.y) {
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
