use crate::element::base::HasBase;
use crate::element::layout::Layout;
use crate::fonts::Fonts;
use crate::input::{Key, Modifiers, MouseButton};
use crate::render::DrawCall;
use std::any::Any;

// Separate trait so the derive macro can generate it independently
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Element: HasBase + AsAny + Any + 'static {
    // ── the only method you must implement ───────────────────────────────────
    fn draw_calls(&self, clip: Option<[f32; 4]>, z: i32, opacity: f32) -> Vec<DrawCall>;

    // ── provided via HasBase — never override these ───────────────────────────
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

    // ── measure — override if element needs intrinsic sizing ─────────────────
    fn measure(&self, _fonts: &mut Fonts, _max_width: Option<f32>) -> Option<(f32, f32)> {
        None
    }
    fn has_measure(&self) -> bool {
        false
    }

    // ── event hooks — override only what you need ─────────────────────────────
    fn on_focus_gained(&mut self) {}
    fn on_focus_lost(&mut self) {}
    fn on_key_press(&mut self, _key: Key, _mods: Modifiers, _text: Option<char>) {}
    fn on_key_release(&mut self, _key: Key, _mods: Modifiers) {}
    fn on_mouse_press(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    fn on_mouse_release(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    fn on_mouse_click(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    fn on_mouse_double_click(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    fn on_mouse_enter(&mut self) {}
    fn on_mouse_leave(&mut self) {}
}

pub type AnyElement = Box<dyn Element>;
