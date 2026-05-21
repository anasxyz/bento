use std::any::Any;

use bento_wgpu::DrawList;

use crate::Ui;
use crate::accumulated::Accumulated;

pub trait Widget {
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize) {}
    fn name(&self) -> &str { "unnamed" }
    fn build(&mut self, ui: &mut Ui) {}
    fn update(&mut self, ui: &mut Ui) {}
    fn remove(&mut self, ui: &mut Ui) {}
    fn hitbox(&self) -> (f32, f32, f32, f32) { (0.0, 0.0, 0.0, 0.0) }
    fn is_dirty(&self) -> bool { false }
    fn set_dirty(&mut self, dirty: bool) {}

    fn render(&self, draw_list: &mut DrawList, acc: &Accumulated) {}
}

pub trait AnyWidget: Widget + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<W: Widget + Any> AnyWidget for W {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
