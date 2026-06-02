use std::cell::Cell;

use bento_wgpu::{DrawList, TextMeasurer};

use crate::{
    input::InputState,
    tree,
    view::{View, ViewId},
};

pub struct Ui {
    pub root: ViewId,
    pub measurer: TextMeasurer,
    pub input: InputState,
}

impl Ui {
    pub fn new(view: impl View + 'static) -> Self {
        Self {
            root: view.build(),
            measurer: TextMeasurer::new(),
            input: InputState::new(),
        }
    }

    pub fn collect_draw_list(&mut self) -> DrawList {
        let mut draw_list = DrawList::new();
        tree::render(self.root, &mut draw_list);
        draw_list
    }
}

/// Redraw stuff
thread_local! {
    static NEEDS_REDRAW: Cell<bool> = Cell::new(false);
}
impl Ui {
    pub fn request_redraw() {
        NEEDS_REDRAW.with(|f| f.set(true));
    }
    pub fn needs_redraw() -> bool {
        NEEDS_REDRAW.with(|f| {
            let v = f.get();
            f.set(false);
            v
        })
    }
}
