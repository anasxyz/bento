use crate::ui::Ui;
use std::any::Any;

pub trait Widget {
    fn build(&mut self, ui: &mut Ui);
    fn update(&mut self, ui: &mut Ui);
    fn remove(&mut self, ui: &mut Ui);
    fn is_dirty(&self) -> bool;
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
