use std::any::Any;

use bento_shared::Scene;

use crate::Ui;

// automatically implemented for all widgets by deriving `Widget`
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Widget: AsAny {
    fn name(&self) -> &str;
    fn build(&mut self, scene: &mut Scene);
    fn update(&mut self, scene: &mut Scene);
}
