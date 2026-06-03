use std::marker::PhantomData;

use bento_wgpu::{DrawList, TextMeasurer};

use crate::reactive::owner::Owner;
use crate::tree;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct ViewId(pub usize);

pub trait View {
    fn name(&self) -> &'static str {
        "unnamed"
    }
    fn build(self: Box<Self>) -> ViewId;
    fn render(&self, x: f32, y: f32, w: f32, h: f32, draw_list: &mut DrawList) {}
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        (0.0, 0.0)
    }
    fn on<E: 'static>(self, f: impl Fn(&E) + 'static) -> WithHandler<Self, E>
    where
        Self: Sized,
    {
        WithHandler {
            inner: self,
            handler: Box::new(f),
            _phantom: PhantomData,
        }
    }
}

pub struct WithHandler<V: View, E: 'static> {
    inner: V,
    handler: Box<dyn Fn(&E)>,
    _phantom: PhantomData<E>,
}

impl<V: View, E: 'static> View for WithHandler<V, E> {
    fn name(&self) -> &'static str {
        "WithHandler"
    }

    fn build(self: Box<Self>) -> ViewId {
        let id = Box::new(self.inner).build();
        tree::add_handler(id, self.handler);
        id
    }

    fn render(&self, x: f32, y: f32, w: f32, h: f32, draw_list: &mut DrawList) {
        self.inner.render(x, y, w, h, draw_list);
    }

    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        self.inner.measure(measurer)
    }
}

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
        let id = self.inner.build();
        tree::store_owner(id, self._owner);
        id
    }
    fn render(&self, x: f32, y: f32, w: f32, h: f32, draw_list: &mut DrawList) {
        self.inner.render(x, y, w, h, draw_list);
    }
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        self.inner.measure(measurer)
    }
}
