use crate::element::layout::Layout;
use crate::fonts::Fonts;
use crate::keyboard::{Key, Modifiers};
use std::any::Any;

pub trait Element {
    fn layout(&self) -> &Layout;
    fn layout_mut(&mut self) -> &mut Layout;
    fn measure(&self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)>;
    fn has_measure(&self) -> bool { false }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn on_mouse_enter(&mut self) -> Option<u32> { None }
    fn on_mouse_leave(&mut self) -> Option<u32> { None }

    fn on_left_press(&mut self) -> Option<u32> { None }
    fn on_left_release(&mut self) -> Option<u32> { None }
    fn on_left_click(&mut self) -> Option<u32> { None }
    fn on_left_double_click(&mut self) -> Option<u32> { None }

    fn on_right_press(&mut self) -> Option<u32> { None }
    fn on_right_release(&mut self) -> Option<u32> { None }
    fn on_right_click(&mut self) -> Option<u32> { None }

    fn on_middle_press(&mut self) -> Option<u32> { None }
    fn on_middle_release(&mut self) -> Option<u32> { None }
    fn on_middle_click(&mut self) -> Option<u32> { None }

    fn on_key_press(&mut self, key: Key, modifiers: Modifiers, text: Option<char>) -> Option<u32> { None }
    fn on_key_release(&mut self, key: Key, modifiers: Modifiers) -> Option<u32> { None }

    fn on_focus_gained(&mut self) -> Option<u32> { None }
    fn on_focus_lost(&mut self) -> Option<u32> { None }
}
