use std::any::Any;

use crate::Ui;

pub trait Widget {
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize) {}
    fn name(&self) -> &str { "unnamed" }
    fn build(&mut self, ui: &mut Ui) {}
    fn hitbox(&self) -> (f32, f32, f32, f32) { (0.0, 0.0, 0.0, 0.0) }
}

pub trait AnyWidget: Widget + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<W: Widget + Any> AnyWidget for W {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
