use crate::widget::HasBase;
use bento_shared::{TextMeasurer, scene::Scene};
use std::any::Any;

// automatically implemented for all widgets by deriving `Widget`
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Widget: AsAny + HasBase {
    fn build(&mut self, scene: &mut Scene);
    fn update(&mut self, scene: &mut Scene, measurer: &mut dyn TextMeasurer);
    fn measure(
        &self,
        known_w: Option<f32>,
        known_h: Option<f32>,
        measurer: &mut dyn TextMeasurer,
    ) -> (f32, f32) {
        (0.0, 0.0)
    }
}
