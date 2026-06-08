use bento_wgpu::{DrawCommand, TextMeasurer};

use crate::{
    tree,
    views::{View, ViewId},
    reactive::{owner, owner::Owner},
};

pub struct OwnedView {
    pub(crate) _owner: Owner,
    pub(crate) inner: Box<dyn View>,
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
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    fn build(self: Box<Self>) -> ViewId {
        let owner = Owner::new();
        owner::store(self._owner);
        let id = Box::new(self.inner).build();
        let owner = owner.collect();
        tree::store_owner(id, owner);
        id
    }

    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        self.inner.render(x, y, w, h)
    }

    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        self.inner.measure(measurer)
    }
}

