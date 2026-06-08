use taffy::prelude::*;
use bento_wgpu::{DrawCommand, TextMeasurer};

use crate::layout::{LayoutProps, Val};
use crate::node::NodeRef;
use crate::{effect, tree};
use crate::views::{View, ViewId};
use crate::reactive::value::Reactive;

pub struct ViewConfig<V: View> {
    pub(crate) inner: V,
    width: Option<Reactive<Val>>,
    height: Option<Reactive<Val>>,
    min_width: Option<Reactive<Val>>,
    min_height: Option<Reactive<Val>>,
    max_width: Option<Reactive<Val>>,
    max_height: Option<Reactive<Val>>,
    flex_grow: Option<Reactive<f32>>,
    flex_shrink: Option<Reactive<f32>>,
    flex_basis: Option<Reactive<Val>>,
    align_self: Option<Reactive<AlignSelf>>,
    justify_self: Option<Reactive<AlignSelf>>,
    padding: Option<Reactive<Val>>,
    padding_left: Option<Reactive<Val>>,
    padding_right: Option<Reactive<Val>>,
    padding_top: Option<Reactive<Val>>,
    padding_bottom: Option<Reactive<Val>>,
    margin: Option<Reactive<Val>>,
    margin_left: Option<Reactive<Val>>,
    margin_right: Option<Reactive<Val>>,
    margin_top: Option<Reactive<Val>>,
    margin_bottom: Option<Reactive<Val>>,
    border: Option<Reactive<Val>>,
    border_left: Option<Reactive<Val>>,
    border_right: Option<Reactive<Val>>,
    border_top: Option<Reactive<Val>>,
    border_bottom: Option<Reactive<Val>>,
    aspect_ratio: Option<Reactive<f32>>,
    direction: Option<Reactive<FlexDirection>>,
    gap: Option<Reactive<Val>>,
    align_items: Option<Reactive<AlignItems>>,
    justify_content: Option<Reactive<JustifyContent>>,
    align_content: Option<Reactive<AlignContent>>,
    flex_wrap: Option<Reactive<FlexWrap>>,
    display: Option<Reactive<Display>>,
    grid_row: Option<Reactive<Line<GridPlacement>>>,
    grid_column: Option<Reactive<Line<GridPlacement>>>,
    position: Option<Reactive<Position>>,
    inset_left: Option<Reactive<Val>>,
    inset_right: Option<Reactive<Val>>,
    inset_top: Option<Reactive<Val>>,
    inset_bottom: Option<Reactive<Val>>,
    handlers: Vec<Box<dyn FnOnce(ViewId)>>,
    node_ref: Option<NodeRef>,
}

impl<V: View> ViewConfig<V> {
    pub fn new(inner: V) -> Self {
        Self {
            inner,
            width: None, height: None,
            min_width: None, min_height: None,
            max_width: None, max_height: None,
            flex_grow: None, flex_shrink: None, flex_basis: None,
            align_self: None, justify_self: None,
            padding: None, padding_left: None, padding_right: None,
            padding_top: None, padding_bottom: None,
            margin: None, margin_left: None, margin_right: None,
            margin_top: None, margin_bottom: None,
            border: None, border_left: None, border_right: None,
            border_top: None, border_bottom: None,
            aspect_ratio: None, direction: None, gap: None,
            align_items: None, justify_content: None, align_content: None,
            flex_wrap: None, display: None,
            grid_row: None, grid_column: None, position: None,
            inset_left: None, inset_right: None,
            inset_top: None, inset_bottom: None,
            handlers: Vec::new(), node_ref: None,
        }
    }

    pub fn node_ref(mut self, r: NodeRef) -> Self { self.node_ref = Some(r); self }

    pub fn on<E: 'static>(mut self, f: impl Fn(&E) + 'static) -> Self {
        self.handlers.push(Box::new(move |id| tree::add_handler(id, f)));
        self
    }

    pub fn w(mut self, v: impl Into<Reactive<Val>>) -> Self { self.width = Some(v.into()); self }
    pub fn h(mut self, v: impl Into<Reactive<Val>>) -> Self { self.height = Some(v.into()); self }
    pub fn min_w(mut self, v: impl Into<Reactive<Val>>) -> Self { self.min_width = Some(v.into()); self }
    pub fn min_h(mut self, v: impl Into<Reactive<Val>>) -> Self { self.min_height = Some(v.into()); self }
    pub fn max_w(mut self, v: impl Into<Reactive<Val>>) -> Self { self.max_width = Some(v.into()); self }
    pub fn max_h(mut self, v: impl Into<Reactive<Val>>) -> Self { self.max_height = Some(v.into()); self }
    pub fn flex_grow(mut self, v: impl Into<Reactive<f32>>) -> Self { self.flex_grow = Some(v.into()); self }
    pub fn flex_shrink(mut self, v: impl Into<Reactive<f32>>) -> Self { self.flex_shrink = Some(v.into()); self }
    pub fn flex_basis(mut self, v: impl Into<Reactive<Val>>) -> Self { self.flex_basis = Some(v.into()); self }
    pub fn align_self(mut self, v: impl Into<Reactive<AlignSelf>>) -> Self { self.align_self = Some(v.into()); self }
    pub fn justify_self(mut self, v: impl Into<Reactive<AlignSelf>>) -> Self { self.justify_self = Some(v.into()); self }
    pub fn p(mut self, v: impl Into<Reactive<Val>>) -> Self { self.padding = Some(v.into()); self }
    pub fn p_left(mut self, v: impl Into<Reactive<Val>>) -> Self { self.padding_left = Some(v.into()); self }
    pub fn p_right(mut self, v: impl Into<Reactive<Val>>) -> Self { self.padding_right = Some(v.into()); self }
    pub fn p_top(mut self, v: impl Into<Reactive<Val>>) -> Self { self.padding_top = Some(v.into()); self }
    pub fn p_bottom(mut self, v: impl Into<Reactive<Val>>) -> Self { self.padding_bottom = Some(v.into()); self }
    pub fn m(mut self, v: impl Into<Reactive<Val>>) -> Self { self.margin = Some(v.into()); self }
    pub fn m_left(mut self, v: impl Into<Reactive<Val>>) -> Self { self.margin_left = Some(v.into()); self }
    pub fn m_right(mut self, v: impl Into<Reactive<Val>>) -> Self { self.margin_right = Some(v.into()); self }
    pub fn m_top(mut self, v: impl Into<Reactive<Val>>) -> Self { self.margin_top = Some(v.into()); self }
    pub fn m_bottom(mut self, v: impl Into<Reactive<Val>>) -> Self { self.margin_bottom = Some(v.into()); self }
    pub fn border(mut self, v: impl Into<Reactive<Val>>) -> Self { self.border = Some(v.into()); self }
    pub fn border_left(mut self, v: impl Into<Reactive<Val>>) -> Self { self.border_left = Some(v.into()); self }
    pub fn border_right(mut self, v: impl Into<Reactive<Val>>) -> Self { self.border_right = Some(v.into()); self }
    pub fn border_top(mut self, v: impl Into<Reactive<Val>>) -> Self { self.border_top = Some(v.into()); self }
    pub fn border_bottom(mut self, v: impl Into<Reactive<Val>>) -> Self { self.border_bottom = Some(v.into()); self }
    pub fn aspect_ratio(mut self, v: impl Into<Reactive<f32>>) -> Self { self.aspect_ratio = Some(v.into()); self }
    pub fn direction(mut self, v: impl Into<Reactive<FlexDirection>>) -> Self { self.direction = Some(v.into()); self }
    pub fn gap(mut self, v: impl Into<Reactive<Val>>) -> Self { self.gap = Some(v.into()); self }
    pub fn align_items(mut self, v: impl Into<Reactive<AlignItems>>) -> Self { self.align_items = Some(v.into()); self }
    pub fn justify_content(mut self, v: impl Into<Reactive<JustifyContent>>) -> Self { self.justify_content = Some(v.into()); self }
    pub fn align_content(mut self, v: impl Into<Reactive<AlignContent>>) -> Self { self.align_content = Some(v.into()); self }
    pub fn flex_wrap(mut self, v: impl Into<Reactive<FlexWrap>>) -> Self { self.flex_wrap = Some(v.into()); self }
    pub fn display(mut self, v: impl Into<Reactive<Display>>) -> Self { self.display = Some(v.into()); self }
    pub fn grid_row(mut self, v: impl Into<Reactive<Line<GridPlacement>>>) -> Self { self.grid_row = Some(v.into()); self }
    pub fn grid_column(mut self, v: impl Into<Reactive<Line<GridPlacement>>>) -> Self { self.grid_column = Some(v.into()); self }
    pub fn position(mut self, v: impl Into<Reactive<Position>>) -> Self { self.position = Some(v.into()); self }
    pub fn inset_left(mut self, v: impl Into<Reactive<Val>>) -> Self { self.inset_left = Some(v.into()); self }
    pub fn inset_right(mut self, v: impl Into<Reactive<Val>>) -> Self { self.inset_right = Some(v.into()); self }
    pub fn inset_top(mut self, v: impl Into<Reactive<Val>>) -> Self { self.inset_top = Some(v.into()); self }
    pub fn inset_bottom(mut self, v: impl Into<Reactive<Val>>) -> Self { self.inset_bottom = Some(v.into()); self }
}

impl<V: View> View for ViewConfig<V> {
    fn name(&self) -> &'static str { self.inner.name() }

    fn build(self: Box<Self>) -> ViewId {
        let handlers = self.handlers;
        let id = Box::new(self.inner).build();
        if let Some(r) = self.node_ref { r.set(id); }
        for handler in handlers { handler(id); }

        let width = self.width; let height = self.height;
        let min_width = self.min_width; let min_height = self.min_height;
        let max_width = self.max_width; let max_height = self.max_height;
        let flex_grow = self.flex_grow; let flex_shrink = self.flex_shrink;
        let flex_basis = self.flex_basis;
        let align_self = self.align_self; let justify_self = self.justify_self;
        let padding = self.padding;
        let padding_left = self.padding_left; let padding_right = self.padding_right;
        let padding_top = self.padding_top; let padding_bottom = self.padding_bottom;
        let margin = self.margin;
        let margin_left = self.margin_left; let margin_right = self.margin_right;
        let margin_top = self.margin_top; let margin_bottom = self.margin_bottom;
        let border = self.border;
        let border_left = self.border_left; let border_right = self.border_right;
        let border_top = self.border_top; let border_bottom = self.border_bottom;
        let aspect_ratio = self.aspect_ratio;
        let direction = self.direction; let gap = self.gap;
        let align_items = self.align_items; let justify_content = self.justify_content;
        let align_content = self.align_content; let flex_wrap = self.flex_wrap;
        let display = self.display;
        let grid_row = self.grid_row; let grid_column = self.grid_column;
        let position = self.position;
        let inset_left = self.inset_left; let inset_right = self.inset_right;
        let inset_top = self.inset_top; let inset_bottom = self.inset_bottom;

        effect(move || {
            let mut l = LayoutProps::default();
            if let Some(f) = &width { l.width = f.get().to_dimension(); }
            if let Some(f) = &height { l.height = f.get().to_dimension(); }
            if let Some(f) = &min_width { l.min_width = f.get().to_dimension(); }
            if let Some(f) = &min_height { l.min_height = f.get().to_dimension(); }
            if let Some(f) = &max_width { l.max_width = f.get().to_dimension(); }
            if let Some(f) = &max_height { l.max_height = f.get().to_dimension(); }
            if let Some(f) = &flex_grow { l.flex_grow = f.get(); }
            if let Some(f) = &flex_shrink { l.flex_shrink = f.get(); }
            if let Some(f) = &flex_basis { l.flex_basis = f.get().to_dimension(); }
            if let Some(f) = &align_self { l.align_self = Some(f.get()); }
            if let Some(f) = &justify_self { l.justify_self = Some(f.get()); }
            if let Some(f) = &padding { let lp = f.get().to_length_percentage(); l.padding = Rect { left: lp, right: lp, top: lp, bottom: lp }; }
            if let Some(f) = &padding_left { l.padding.left = f.get().to_length_percentage(); }
            if let Some(f) = &padding_right { l.padding.right = f.get().to_length_percentage(); }
            if let Some(f) = &padding_top { l.padding.top = f.get().to_length_percentage(); }
            if let Some(f) = &padding_bottom { l.padding.bottom = f.get().to_length_percentage(); }
            if let Some(f) = &margin { let lpa = f.get().to_length_percentage_auto(); l.margin = Rect { left: lpa, right: lpa, top: lpa, bottom: lpa }; }
            if let Some(f) = &margin_left { l.margin.left = f.get().to_length_percentage_auto(); }
            if let Some(f) = &margin_right { l.margin.right = f.get().to_length_percentage_auto(); }
            if let Some(f) = &margin_top { l.margin.top = f.get().to_length_percentage_auto(); }
            if let Some(f) = &margin_bottom { l.margin.bottom = f.get().to_length_percentage_auto(); }
            if let Some(f) = &border { let lp = f.get().to_length_percentage(); l.border = Rect { left: lp, right: lp, top: lp, bottom: lp }; }
            if let Some(f) = &border_left { l.border.left = f.get().to_length_percentage(); }
            if let Some(f) = &border_right { l.border.right = f.get().to_length_percentage(); }
            if let Some(f) = &border_top { l.border.top = f.get().to_length_percentage(); }
            if let Some(f) = &border_bottom { l.border.bottom = f.get().to_length_percentage(); }
            if let Some(f) = &aspect_ratio { l.aspect_ratio = Some(f.get()); }
            if let Some(f) = &direction { l.flex_direction = f.get(); }
            if let Some(f) = &gap { let lp = f.get().to_length_percentage(); l.gap = Size { width: lp, height: lp }; }
            if let Some(f) = &align_items { l.align_items = Some(f.get()); }
            if let Some(f) = &justify_content { l.justify_content = Some(f.get()); }
            if let Some(f) = &align_content { l.align_content = Some(f.get()); }
            if let Some(f) = &flex_wrap { l.flex_wrap = f.get(); }
            if let Some(f) = &display { l.display = f.get(); }
            if let Some(f) = &grid_row { l.grid_row = f.get_clone(); }
            if let Some(f) = &grid_column { l.grid_column = f.get_clone(); }
            if let Some(f) = &position { l.position = f.get(); }
            if let Some(f) = &inset_left { l.inset.left = f.get().to_length_percentage_auto(); }
            if let Some(f) = &inset_right { l.inset.right = f.get().to_length_percentage_auto(); }
            if let Some(f) = &inset_top { l.inset.top = f.get().to_length_percentage_auto(); }
            if let Some(f) = &inset_bottom { l.inset.bottom = f.get().to_length_percentage_auto(); }
            tree::set_layout(id, l);
        });

        id
    }

    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        self.inner.render(x, y, w, h)
    }

    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        self.inner.measure(measurer)
    }
}
