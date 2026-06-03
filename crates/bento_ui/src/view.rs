use std::marker::PhantomData;

use bento_wgpu::{DrawCommand, DrawList, TextMeasurer};

use crate::layout::{Container, Size};
use crate::reactive::owner::{self, Owner};
use crate::tree;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct ViewId(pub usize);

pub trait View {
    fn name(&self) -> &'static str {
        "unnamed"
    }
    fn build(self: Box<Self>) -> ViewId;
    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand>;
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        (0.0, 0.0)
    }
    fn as_container(&self) -> Option<&dyn Container> {
        None
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

    fn width(self, size: Size) -> WithSize<Self>
    where
        Self: Sized,
    {
        WithSize {
            inner: self,
            width: Some(size),
            height: None,
        }
    }

    fn height(self, size: Size) -> WithSize<Self>
    where
        Self: Sized,
    {
        WithSize {
            inner: self,
            width: None,
            height: Some(size),
        }
    }
}

pub struct WithSize<V: View> {
    inner: V,
    width: Option<Size>,
    height: Option<Size>,
}

impl<V: View> View for WithSize<V> {
    fn build(self: Box<Self>) -> ViewId {
        let id = Box::new(self.inner).build();
        if let Some(w) = self.width {
            tree::set_width(id, w);
        }
        if let Some(h) = self.height {
            tree::set_height(id, h);
        }
        id
    }

    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        self.inner.render(x, y, w, h)
    }

    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        self.inner.measure(measurer)
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

    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        self.inner.render(x, y, w, h)
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
        let owner = Owner::new();
        // move _owner into the scope so it's kept alive inside the build owner
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
