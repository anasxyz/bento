use crate::Widget;

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
