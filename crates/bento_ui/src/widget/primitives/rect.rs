use std::any::Any;

use crate::{AsAny, Widget};

#[derive(Debug)]
pub struct Rect {}

impl Widget for Rect {
    fn name(&self) -> &str {
        "Rect"
    }

    fn build(&mut self) {
        // add a rect to the scene
        // set its position and size
    }

    fn update(&mut self) {
        // update the rect in the scene
        // set its position and size
    }
}

// TODO: add to proc macro
impl AsAny for Rect {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
