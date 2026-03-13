use std::any::Any;
use crate::element::layout::Layout;
use crate::fonts::Fonts;

pub trait Element {
    fn layout(&self) -> &Layout;
    fn layout_mut(&mut self) -> &mut Layout;
    fn measure(&self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)>;
    fn has_measure(&self) -> bool { false }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
