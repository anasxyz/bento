use std::marker::PhantomData;

use bento_wgpu::{DrawCommand, DrawList, TextMeasurer};

use crate::layout::{Container, Position, Size};
use crate::reactive::owner::{self, Owner};
use crate::{effect, tree};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct ViewId(pub usize);

pub struct ViewConfig<V: View> {
    inner: V,
    width: Option<Size>,
    height: Option<Size>,
    x: Option<Box<dyn Fn() -> f32>>,
    y: Option<Box<dyn Fn() -> f32>>,
    handlers: Vec<Box<dyn FnOnce(ViewId)>>,
}

impl<V: View> ViewConfig<V> {
    fn new(inner: V) -> Self {
        Self {
            inner,
            width: None,
            height: None,
            x: None,
            y: None,
            handlers: Vec::new(),
        }
    }

    pub fn width(mut self, size: Size) -> Self {
        self.width = Some(size);
        self
    }

    pub fn height(mut self, size: Size) -> Self {
        self.height = Some(size);
        self
    }

    pub fn x(mut self, x: impl Fn() -> f32 + 'static) -> Self {
        self.x = Some(Box::new(x));
        self
    }

    pub fn y(mut self, y: impl Fn() -> f32 + 'static) -> Self {
        self.y = Some(Box::new(y));
        self
    }

    pub fn on<E: 'static>(mut self, f: impl Fn(&E) + 'static) -> Self {
        self.handlers.push(Box::new(move |id| {
            tree::add_handler(id, f);
        }));
        self
    }
}

impl<V: View> View for ViewConfig<V> {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    fn build(self: Box<Self>) -> ViewId {
        let width = self.width;
        let height = self.height;
        let x = self.x;
        let y = self.y;
        let handlers = self.handlers;

        let id = Box::new(self.inner).build();

        if let Some(w) = width {
            tree::set_width(id, w);
        }
        if let Some(h) = height {
            tree::set_height(id, h);
        }
        for handler in handlers {
            handler(id);
        }
        if x.is_some() || y.is_some() {
            let ix = x.as_ref().map(|f| f()).unwrap_or(0.0);
            let iy = y.as_ref().map(|f| f()).unwrap_or(0.0);
            tree::set_position(id, Position::Absolute { x: ix, y: iy });
            effect(move || {
                let nx = x.as_ref().map(|f| f()).unwrap_or(ix);
                let ny = y.as_ref().map(|f| f()).unwrap_or(iy);
                tree::set_position(id, Position::Absolute { x: nx, y: ny });
            });
        }

        id
    }

    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        self.inner.render(x, y, w, h)
    }

    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        self.inner.measure(measurer)
    }

    fn as_container(&self) -> Option<&dyn Container> {
        self.inner.as_container()
    }
}

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

    fn on<E: 'static>(self, f: impl Fn(&E) + 'static) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).on(f)
    }

    fn width(self, size: Size) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).width(size)
    }

    fn height(self, size: Size) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).height(size)
    }

    fn x(self, x: impl Fn() -> f32 + 'static) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).x(x)
    }

    fn y(self, y: impl Fn() -> f32 + 'static) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).y(y)
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
        owner::store(self._owner);
        let id = Box::new(self.inner).build();
        let owner = owner.collect();
        tree::store_owner(id, owner);
        id
    }
    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        self.inner.render(x, y, w, h)
    }
    fn as_container(&self) -> Option<&dyn Container> {
        self.inner.as_container()
    }
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        self.inner.measure(measurer)
    }
}
