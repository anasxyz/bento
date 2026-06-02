use bento_wgpu::{DrawList, TextMeasurer};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct ViewId(pub usize);

pub trait View {
    fn build(self) -> ViewId;
}
