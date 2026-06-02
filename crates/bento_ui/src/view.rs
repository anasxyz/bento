use bento_wgpu::{DrawList, TextMeasurer};

pub trait View {
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32);
    fn render(&self, x: f32, y: f32, measurer: &mut TextMeasurer, draw_list: &mut DrawList);
}
