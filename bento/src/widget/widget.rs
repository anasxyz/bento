use crate::fonts::Fonts;
use crate::input::{Key, Modifiers, MouseButton};
use crate::widget::base::HasBase;
use bento_wgpu::SceneGraph;
use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Widget: HasBase + AsAny + Any + 'static {
    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, h: f32);

    fn measure(&self, _fonts: &mut Fonts, _max_width: Option<f32>) -> Option<(f32, f32)> {
        None
    }
    fn has_measure(&self) -> bool {
        false
    }

    // focus
    fn on_focus_gained(&mut self) {
        self.base_mut().focused = true;
    }
    fn on_focus_lost(&mut self) {
        self.base_mut().focused = false;
    }

    // keyboard
    fn on_key_press(&mut self, _key: Key, _mods: Modifiers, _text: Option<char>) {}
    fn on_key_release(&mut self, _key: Key, _mods: Modifiers) {}

    // mouse
    fn on_mouse_press(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    fn on_mouse_release(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    fn on_mouse_click(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    fn on_mouse_double_click(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    fn on_mouse_move(&mut self, _x: f32, _y: f32) {}
    fn on_mouse_scroll(&mut self, _dx: f32, _dy: f32) {}
    fn on_mouse_enter(&mut self) {}
    fn on_mouse_leave(&mut self) {}
}

pub type AnyWidget = Box<dyn Widget>;
