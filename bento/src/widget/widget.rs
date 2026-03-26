use crate::fonts::Fonts;
use crate::widget::base::HasBase;
use bento_wgpu::SceneGraph;
use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Widget: HasBase + AsAny + Any + 'static {
    /// push computed position/size into scene graph nodes
    /// called every frame after layout resolves
    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, h: f32);

    fn measure(&self, _fonts: &mut Fonts, _max_width: Option<f32>) -> Option<(f32, f32)> {
        None
    }
    fn has_measure(&self) -> bool {
        false
    }

    fn on_focus_gained(&mut self) {
        self.base_mut().focused = true;
    }
    fn on_focus_lost(&mut self) {
        self.base_mut().focused = false;
    }

    // event hooks
    // input types will be added when input module exists
    fn on_mouse_move(&mut self, _x: f32, _y: f32) {}
    fn on_mouse_scroll(&mut self, _dx: f32, _dy: f32) {}
    fn on_mouse_enter(&mut self) {}
    fn on_mouse_leave(&mut self) {}
}

pub type AnyWidget = Box<dyn Widget>;
