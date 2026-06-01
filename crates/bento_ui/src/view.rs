use crate::reactive::owner::Owner;
use bento_wgpu::{DrawList, RectDraw, TextAlign, TextDraw};

pub trait View {
    fn render(&self, x: f32, y: f32, draw_list: &mut DrawList);
}

pub struct OwnedView {
    _owner: Owner,
    inner: Box<dyn View>,
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
    fn render(&self, x: f32, y: f32, draw_list: &mut DrawList) {
        self.inner.render(x, y, draw_list);
    }
}
