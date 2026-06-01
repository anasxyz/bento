use crate::reactive::owner::Owner;
use bento_wgpu::{DrawList, TextMeasurer};

pub trait View {
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32);
    fn render(&self, x: f32, y: f32, measurer: &mut TextMeasurer, draw_list: &mut DrawList);
}

pub struct OwnedView {
    pub _owner: Owner,
    pub inner: Box<dyn View>,
}

impl OwnedView {
    pub fn new(owner: Owner, inner: impl View + 'static) -> Self {
        Self {
            _owner: owner,
            inner: Box::new(inner),
        }
    }
}

impl View for OwnedView {
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        self.inner.measure(measurer)
    }

    fn render(&self, x: f32, y: f32, measurer: &mut TextMeasurer, draw_list: &mut DrawList) {
        self.inner.render(x, y, measurer, draw_list);
    }
}
