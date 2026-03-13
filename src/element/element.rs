use crate::element::callbacks::Callbacks;
use crate::element::layout::Layout;
use crate::fonts::Fonts;
use std::any::Any;

pub trait Element {
    fn layout(&self) -> &Layout;
    fn layout_mut(&mut self) -> &mut Layout;
    fn callbacks(&self) -> &Callbacks;
    fn callbacks_mut(&mut self) -> &mut Callbacks;
    fn measure(&self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)>;
    fn has_measure(&self) -> bool {
        false
    }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
