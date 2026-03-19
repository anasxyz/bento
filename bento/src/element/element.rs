use crate::element::base::HasBase;
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::fonts::Fonts;
use crate::input::{Key, Modifiers, MouseButton};
use crate::render::DrawCall;
use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventResult {
    Handled,
    Propagate,
}

pub trait Element: HasBase + AsAny + Any + 'static {
    // required 
    fn draw_calls(&self, clip: Option<[f32; 4]>, z: i32, opacity: f32) -> Vec<DrawCall>;

    // provided via HasBase 
    fn layout(&self) -> &Layout {
        self.base().layout()
    }
    fn layout_mut_internal(&mut self) -> &mut Layout {
        self.base_mut().layout_mut()
    }
    fn is_dirty(&self) -> bool {
        self.base().is_dirty()
    }
    fn set_dirty(&mut self, v: bool) {
        self.base_mut().set_dirty(v);
    }

    // measure 
    fn measure(&self, _fonts: &mut Fonts, _max_width: Option<f32>) -> Option<(f32, f32)> {
        None
    }
    fn has_measure(&self) -> bool {
        false
    }

    // event hooks 
    // ui and handle are provided so elements can emit events, scroll parents, etc
    fn on_focus_gained(&mut self, _ui: &mut crate::ui::Ui, _handle: Handle<()>) {
        self.base_mut().focused = true;
        self.base_mut().dirty = true;
    }
    fn on_focus_lost(&mut self, _ui: &mut crate::ui::Ui, _handle: Handle<()>) {
        self.base_mut().focused = false;
        self.base_mut().dirty = true;
    }
    fn on_key_press(
        &mut self,
        _ui: &mut crate::ui::Ui,
        _handle: Handle<()>,
        _key: Key,
        _mods: Modifiers,
        _text: Option<char>,
    ) -> EventResult {
        EventResult::Propagate
    }
    fn on_key_release(
        &mut self,
        _ui: &mut crate::ui::Ui,
        _handle: Handle<()>,
        _key: Key,
        _mods: Modifiers,
    ) -> EventResult {
        EventResult::Propagate
    }
    fn on_mouse_press(
        &mut self,
        _ui: &mut crate::ui::Ui,
        _handle: Handle<()>,
        _x: f32,
        _y: f32,
        _button: MouseButton,
    ) -> EventResult {
        EventResult::Propagate
    }
    fn on_mouse_release(
        &mut self,
        _ui: &mut crate::ui::Ui,
        _handle: Handle<()>,
        _x: f32,
        _y: f32,
        _button: MouseButton,
    ) -> EventResult {
        EventResult::Propagate
    }
    fn on_mouse_click(
        &mut self,
        _ui: &mut crate::ui::Ui,
        _handle: Handle<()>,
        _x: f32,
        _y: f32,
        _button: MouseButton,
    ) -> EventResult {
        EventResult::Propagate
    }
    fn on_mouse_double_click(
        &mut self,
        _ui: &mut crate::ui::Ui,
        _handle: Handle<()>,
        _x: f32,
        _y: f32,
        _button: MouseButton,
    ) -> EventResult {
        EventResult::Propagate
    }
    fn on_mouse_move(
        &mut self,
        _ui: &mut crate::ui::Ui,
        _handle: Handle<()>,
        _x: f32,
        _y: f32,
    ) -> EventResult {
        EventResult::Propagate
    }
    fn on_mouse_scroll(
        &mut self,
        _ui: &mut crate::ui::Ui,
        _handle: Handle<()>,
        _delta_x: f32,
        _delta_y: f32,
    ) -> EventResult {
        EventResult::Propagate
    }
    fn on_mouse_enter(&mut self, _ui: &mut crate::ui::Ui, _handle: Handle<()>) {}
    fn on_mouse_leave(&mut self, _ui: &mut crate::ui::Ui, _handle: Handle<()>) {}
}

pub type AnyElement = Box<dyn Element>;
