use std::cell::Cell;
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
    render_subscriber: runtime::SubscriberId,
}

impl Ui {
    pub fn new(view: impl View + 'static, request_redraw: impl Fn() + 'static) -> Self {
        let root = Box::new(view).build();
        let render_subscriber = runtime::create_subscriber(Rc::new(move || request_redraw()));

        Self {
            root,
            measurer: TextMeasurer::new(),
            input: InputState::new(),
            render_subscriber,
        }
    }

    pub fn draw(&mut self) -> DrawList {
        let mut draw_list = DrawList::new();

        runtime::push_observer(self.render_subscriber);
        let t = web_time::Instant::now();
        tree::layout(self.root, 0.0, 0.0, &mut self.measurer);
        println!("[ui] layout took {:?}", t.elapsed());
        let t = web_time::Instant::now();
        tree::render(self.root, &mut draw_list);
        println!("[ui] render took {:?}", t.elapsed());
        runtime::pop_observer();

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
