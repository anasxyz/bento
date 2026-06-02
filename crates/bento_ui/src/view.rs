use bento_wgpu::{DrawList, TextMeasurer};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct ViewId(pub usize);

pub trait View {
    fn name(&self) -> &'static str;
    fn build(self) -> ViewId;
    fn render(&self, x: f32, y: f32, w: f32, h: f32, draw_list: &mut DrawList);
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32);
}
