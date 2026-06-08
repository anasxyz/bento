use bento_wgpu::{DrawCommand, TextMeasurer};
use taffy::prelude::*;

use crate::layout::{LayoutProps, Val};
use crate::node::NodeRef;
use crate::reactive::owner::{self, Owner};
use crate::{effect, tree};
use crate::views::{NamedView, ViewConfig};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct ViewId(pub usize);

pub trait View {
    // must implement
    fn name(&self) -> &'static str { "unnamed" }
    fn build(self: Box<Self>) -> ViewId;
    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand>;
    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) { (0.0, 0.0) }

    // auto implemented
    fn named(self, name: &'static str) -> NamedView<Self> where Self: Sized { NamedView { inner: self, name } }
    fn on<E: 'static>(self, f: impl Fn(&E) + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).on(f) }
    fn node_ref(self, r: NodeRef) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).node_ref(r) }

    // auto implemented layout methods
    fn w(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).w(v) }
    fn h(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).h(v) }
    fn min_w(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).min_w(v) }
    fn min_h(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).min_h(v) }
    fn max_w(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).max_w(v) }
    fn max_h(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).max_h(v) }
    fn flex_grow(self, v: impl Fn() -> f32 + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).flex_grow(v) }
    fn flex_shrink(self, v: impl Fn() -> f32 + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).flex_shrink(v) }
    fn flex_basis(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).flex_basis(v) }
    fn align_self(self, v: impl Fn() -> AlignSelf + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).align_self(v) }
    fn justify_self(self, v: impl Fn() -> AlignSelf + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).justify_self(v) }
    fn p(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).p(v) }
    fn p_left(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).p_left(v) }
    fn p_right(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).p_right(v) }
    fn p_top(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).p_top(v) }
    fn p_bottom(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).p_bottom(v) }
    fn m(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).m(v) }
    fn m_left(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).m_left(v) }
    fn m_right(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).m_right(v) }
    fn m_top(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).m_top(v) }
    fn m_bottom(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).m_bottom(v) }
    fn border(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).border(v) }
    fn border_left(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).border_left(v) }
    fn border_right(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).border_right(v) }
    fn border_top(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).border_top(v) }
    fn border_bottom(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).border_bottom(v) }
    fn aspect_ratio(self, v: impl Fn() -> f32 + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).aspect_ratio(v) }
    fn direction(self, v: impl Fn() -> FlexDirection + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).direction(v) }
    fn gap(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).gap(v) }
    fn align_items(self, v: impl Fn() -> AlignItems + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).align_items(v) }
    fn justify_content(self, v: impl Fn() -> JustifyContent + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).justify_content(v) }
    fn align_content(self, v: impl Fn() -> AlignContent + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).align_content(v) }
    fn flex_wrap(self, v: impl Fn() -> FlexWrap + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).flex_wrap(v) }
    fn display(self, v: impl Fn() -> Display + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).display(v) }
    fn grid_row(self, v: impl Fn() -> Line<GridPlacement> + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).grid_row(v) }
    fn grid_column(self, v: impl Fn() -> Line<GridPlacement> + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).grid_column(v) }
    fn position(self, v: impl Fn() -> Position + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).position(v) }
    fn inset_left(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).inset_left(v) }
    fn inset_right(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).inset_right(v) }
    fn inset_top(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).inset_top(v) }
    fn inset_bottom(self, v: impl Fn() -> Val + 'static) -> ViewConfig<Self> where Self: Sized { ViewConfig::new(self).inset_bottom(v) }
}

