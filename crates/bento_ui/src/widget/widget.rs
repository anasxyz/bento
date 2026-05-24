use crate::acc::Accumulated;
use bento_shared::CosmicTextMeasurer;
use bento_wgpu::DrawList;
use std::any::Any;

pub trait Widget {
    fn name(&self) -> &str { "unnamed" }
    fn update(&mut self, measurer: &mut CosmicTextMeasurer) {}
    fn size(&self) -> (f32, f32) { (0.0, 0.0) }
    fn position(&self) -> (f32, f32) { (0.0, 0.0) }
    fn set_position(&mut self, x: f32, y: f32) {}
    fn z(&self) -> i32 { 0 }
    fn render(&self, canvas: &mut Canvas) {}
}

pub trait AnyWidget: Widget + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<W: Widget + Any> AnyWidget for W {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

pub struct Canvas<'a> {
    pub draw_list: &'a mut DrawList,
    pub x: f32,
    pub y: f32,
    pub z: i32,
    pub opacity: f32,
    pub clip: Option<[f32; 4]>,
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}
