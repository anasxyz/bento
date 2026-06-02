use bento_wgpu::{DrawList, TextMeasurer};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct ViewId(pub usize);

pub trait View {
    fn name(&self) -> &'static str;
    fn build(self) -> ViewId;
    fn render(&self, draw_list: &mut DrawList);
}
