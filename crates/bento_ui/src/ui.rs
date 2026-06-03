use std::cell::{Cell, RefCell};
use std::rc::Rc;

use bento_wgpu::{DrawList, TextMeasurer};

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
        }
    }

    pub fn draw(&mut self) -> DrawList {
        let mut draw_list = DrawList::new();

        tree::layout(self.root, 0.0, 0.0, &mut self.measurer);
        tree::render(self.root, &mut draw_list);

        draw_list
    }

    pub fn process_input(&mut self) {
        self.keyboard_stuff();
        self.mouse_stuff();
    }

    pub fn keyboard_stuff(&mut self) {
        // keyboard events later
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
                    &events::Click {
                        x: self.input.mouse.x,
                        y: self.input.mouse.y,
                        button: mouse::MouseButton::Left,
                    },
                );
            }
        }
    }

    pub fn mouse_move_events(&mut self) {
        if self.input.mouse.dx != 0.0 || self.input.mouse.dy != 0.0 {
            if let Some(id) = tree::hit_test(self.root, self.input.mouse.x, self.input.mouse.y) {
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
