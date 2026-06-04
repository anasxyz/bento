use std::marker::PhantomData;

use bento_wgpu::{DrawCommand, TextMeasurer};
use taffy::prelude::*;

use crate::layout::LayoutProps;
use crate::node::NodeRef;
use crate::reactive::owner::{self, Owner};
use crate::{effect, tree};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct ViewId(pub usize);

pub struct ViewConfig<V: View> {
    inner: V,
    layout: LayoutProps,
    x: Option<Box<dyn Fn() -> f32>>,
    y: Option<Box<dyn Fn() -> f32>>,
    handlers: Vec<Box<dyn FnOnce(ViewId)>>,
    node_ref: Option<NodeRef>,
}

impl<V: View> ViewConfig<V> {
    fn new(inner: V) -> Self {
        Self {
            inner,
            layout: LayoutProps::default(),
            x: None,
            y: None,
            handlers: Vec::new(),
            node_ref: None,
        }
    }

    pub fn node_ref(mut self, r: NodeRef) -> Self {
        self.node_ref = Some(r);
        self
    }

    pub fn w(mut self, width: Dimension) -> Self {
        self.layout.width = width;
        self
    }

    pub fn h(mut self, height: Dimension) -> Self {
        self.layout.height = height;
        self
    }

    pub fn min_width(mut self, width: Dimension) -> Self {
        self.layout.min_width = width;
        self
    }

    pub fn min_height(mut self, height: Dimension) -> Self {
        self.layout.min_height = height;
        self
    }

    pub fn max_width(mut self, width: Dimension) -> Self {
        self.layout.max_width = width;
        self
    }

    pub fn max_height(mut self, height: Dimension) -> Self {
        self.layout.max_height = height;
        self
    }

    pub fn flex_grow(mut self, v: f32) -> Self {
        self.layout.flex_grow = v;
        self
    }

    pub fn flex_shrink(mut self, v: f32) -> Self {
        self.layout.flex_shrink = v;
        self
    }

    pub fn flex_basis(mut self, v: Dimension) -> Self {
        self.layout.flex_basis = v;
        self
    }

    pub fn align_self(mut self, v: AlignSelf) -> Self {
        self.layout.align_self = Some(v);
        self
    }

    pub fn justify_self(mut self, v: AlignSelf) -> Self {
        self.layout.justify_self = Some(v);
        self
    }

    pub fn p(mut self, v: f32) -> Self {
        self.layout.padding = Rect {
            left: LengthPercentage::length(v),
            right: LengthPercentage::length(v),
            top: LengthPercentage::length(v),
            bottom: LengthPercentage::length(v),
        };
        self
    }

    pub fn m(mut self, v: f32) -> Self {
        self.layout.margin = Rect {
            left: LengthPercentageAuto::length(v),
            right: LengthPercentageAuto::length(v),
            top: LengthPercentageAuto::length(v),
            bottom: LengthPercentageAuto::length(v),
        };
        self
    }

    pub fn border(mut self, v: Rect<LengthPercentage>) -> Self {
        self.layout.border = v;
        self
    }

    pub fn aspect_ratio(mut self, v: f32) -> Self {
        self.layout.aspect_ratio = Some(v);
        self
    }

    pub fn grid_row(mut self, v: Line<GridPlacement>) -> Self {
        self.layout.grid_row = v;
        self
    }

    pub fn grid_column(mut self, v: Line<GridPlacement>) -> Self {
        self.layout.grid_column = v;
        self
    }

    pub fn x(mut self, x: impl Fn() -> f32 + 'static) -> Self {
        self.layout.position = Position::Absolute;
        self.x = Some(Box::new(x));
        self
    }

    pub fn y(mut self, y: impl Fn() -> f32 + 'static) -> Self {
        self.layout.position = Position::Absolute;
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
        let layout = self.layout;
        let x = self.x;
        let y = self.y;
        let handlers = self.handlers;

        let id = Box::new(self.inner).build();

        if let Some(r) = self.node_ref {
            r.0.set(Some(id));
        }

        tree::set_layout(id, layout);

        for handler in handlers {
            handler(id);
        }

        if x.is_some() || y.is_some() {
            let ix = x.as_ref().map(|f| f()).unwrap_or(0.0);
            let iy = y.as_ref().map(|f| f()).unwrap_or(0.0);
            tree::set_inset(id, ix, iy);
            effect(move || {
                let nx = x.as_ref().map(|f| f()).unwrap_or(ix);
                let ny = y.as_ref().map(|f| f()).unwrap_or(iy);
                tree::set_inset(id, nx, ny);
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

    fn on<E: 'static>(self, f: impl Fn(&E) + 'static) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).on(f)
    }

    fn node_ref(self, r: NodeRef) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).node_ref(r)
    }

    fn w(self, width: Dimension) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).w(width)
    }

    fn h(self, height: Dimension) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).h(height)
    }

    fn min_width(self, width: Dimension) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).min_width(width)
    }

    fn min_height(self, height: Dimension) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).min_height(height)
    }

    fn max_width(self, width: Dimension) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).max_width(width)
    }

    fn max_height(self, height: Dimension) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).max_height(height)
    }

    fn flex_grow(self, v: f32) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).flex_grow(v)
    }

    fn flex_shrink(self, v: f32) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).flex_shrink(v)
    }

    fn flex_basis(self, v: Dimension) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).flex_basis(v)
    }

    fn align_self(self, v: AlignSelf) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).align_self(v)
    }

    fn justify_self(self, v: AlignSelf) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).justify_self(v)
    }

    fn p(self, v: f32) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).p(v)
    }

    fn m(self, v: f32) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).m(v)
    }

    fn border(self, v: Rect<LengthPercentage>) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).border(v)
    }

    fn aspect_ratio(self, v: f32) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).aspect_ratio(v)
    }

    fn grid_row(self, v: Line<GridPlacement>) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).grid_row(v)
    }

    fn grid_column(self, v: Line<GridPlacement>) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).grid_column(v)
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
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        self.inner.measure(measurer)
    }
}
