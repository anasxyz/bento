use bento_wgpu::{DrawList, TextMeasurer};

pub trait View {
    fn render(&self, draw_list: &mut DrawList);
}
