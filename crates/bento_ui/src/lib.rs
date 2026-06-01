#![allow(dead_code)]
#![allow(unused)]

pub(crate) mod events;
pub(crate) mod input;
pub mod reactive;
pub(crate) mod types;
pub(crate) mod ui;
pub(crate) mod view;

pub use events::types::*;
pub use input::keyboard::Key;
pub use types::*;

pub use reactive::{derived, effect, state};
pub use ui::Ui;
pub use view::{Rect, Text, View, OwnedView, rect, text};

use std::cell::Cell;

thread_local! {
    static NEEDS_REDRAW: Cell<bool> = Cell::new(false);
}

pub fn request_redraw() {
    NEEDS_REDRAW.with(|f| f.set(true));
}

pub fn take_needs_redraw() -> bool {
    NEEDS_REDRAW.with(|f| {
        let v = f.get();
        f.set(false);
        v
    })
}
