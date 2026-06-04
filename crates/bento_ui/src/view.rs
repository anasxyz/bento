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
    inset_left: Option<Box<dyn Fn() -> LengthPercentageAuto>>,
    inset_right: Option<Box<dyn Fn() -> LengthPercentageAuto>>,
    inset_top: Option<Box<dyn Fn() -> LengthPercentageAuto>>,
    inset_bottom: Option<Box<dyn Fn() -> LengthPercentageAuto>>,
    handlers: Vec<Box<dyn FnOnce(ViewId)>>,
    node_ref: Option<NodeRef>,
}

impl<V: View> ViewConfig<V> {
    fn new(inner: V) -> Self {
        Self {
            inner,
            layout: LayoutProps::default(),
            inset_left: None,
            inset_right: None,
            inset_top: None,
            inset_bottom: None,
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

    pub fn m_left(mut self, v: f32) -> Self {
        self.layout.margin.left = LengthPercentageAuto::length(v);
        self
    }

    pub fn m_right(mut self, v: f32) -> Self {
        self.layout.margin.right = LengthPercentageAuto::length(v);
        self
    }

    pub fn m_top(mut self, v: f32) -> Self {
        self.layout.margin.top = LengthPercentageAuto::length(v);
        self
    }

    pub fn m_bottom(mut self, v: f32) -> Self {
        self.layout.margin.bottom = LengthPercentageAuto::length(v);
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

    pub fn position(mut self, v: Position) -> Self {
        self.layout.position = v;
        self
    }

    pub fn inset_left(mut self, v: impl Fn() -> LengthPercentageAuto + 'static) -> Self {
        self.inset_left = Some(Box::new(v));
        self
    }

    pub fn inset_right(mut self, v: impl Fn() -> LengthPercentageAuto + 'static) -> Self {
        self.inset_right = Some(Box::new(v));
        self
    }

    pub fn inset_top(mut self, v: impl Fn() -> LengthPercentageAuto + 'static) -> Self {
        self.inset_top = Some(Box::new(v));
        self
    }

    pub fn inset_bottom(mut self, v: impl Fn() -> LengthPercentageAuto + 'static) -> Self {
        self.inset_bottom = Some(Box::new(v));
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
        let inset_left = self.inset_left;
        let inset_right = self.inset_right;
        let inset_top = self.inset_top;
        let inset_bottom = self.inset_bottom;
        let handlers = self.handlers;

        let id = Box::new(self.inner).build();

        if let Some(r) = self.node_ref {
            r.set(id);
        }

        tree::set_layout(id, layout);

        for handler in handlers {
            handler(id);
        }

        if inset_left.is_some()
            || inset_right.is_some()
            || inset_top.is_some()
            || inset_bottom.is_some()
        {
            effect(move || {
                let left = inset_left.as_ref().map(|f| f());
                let right = inset_right.as_ref().map(|f| f());
                let top = inset_top.as_ref().map(|f| f());
                let bottom = inset_bottom.as_ref().map(|f| f());
                tree::update_inset(id, left, right, top, bottom);
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

    fn m_left(self, v: f32) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).m_left(v)
    }

    fn m_right(self, v: f32) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).m_right(v)
    }

    fn m_top(self, v: f32) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).m_top(v)
    }

    fn m_bottom(self, v: f32) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).m_bottom(v)
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

    fn position(self, v: Position) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).position(v)
    }

    fn inset_left(self, v: impl Fn() -> LengthPercentageAuto + 'static) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).inset_left(v)
    }

    fn inset_right(self, v: impl Fn() -> LengthPercentageAuto + 'static) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).inset_right(v)
    }

    fn inset_top(self, v: impl Fn() -> LengthPercentageAuto + 'static) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).inset_top(v)
    }

    fn inset_bottom(self, v: impl Fn() -> LengthPercentageAuto + 'static) -> ViewConfig<Self>
    where
        Self: Sized,
    {
        ViewConfig::new(self).inset_bottom(v)
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
