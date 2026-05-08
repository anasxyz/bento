use bento_shared::{TextMeasurer, scene::Scene};
use std::any::Any;

// automatically implemented for all widgets by deriving `Widget`
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Widget: AsAny {
    fn build(&mut self, scene: &mut Scene);
    fn update(&mut self, scene: &mut Scene, measurer: &mut dyn TextMeasurer);
}
