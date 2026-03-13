use crate::element::layout::Layout;
use crate::fonts::Fonts;

pub trait Element {
    fn layout(&self) -> &Layout;
    fn layout_mut(&mut self) -> &mut Layout;
    fn measure(&self, fonts: &mut Fonts) -> Option<(f32, f32)>;
}
