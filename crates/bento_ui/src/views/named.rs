use bento_wgpu::{DrawCommand, TextMeasurer};

use crate::{
    tree,
    views::{View, ViewId},
};

pub struct NamedView<V: View> {
    pub(crate) inner: V,
    pub(crate) name: &'static str,
}

impl<V: View> View for NamedView<V> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn build(self: Box<Self>) -> ViewId {
        let name = self.name;
        let id = Box::new(self.inner).build();
        tree::set_name(id, name);
        id
    }

    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        self.inner.render(x, y, w, h)
    }

    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        self.inner.measure(measurer)
    }
}
