use std::collections::HashMap;
use taffy::prelude::{
    AvailableSpace, Dimension, Display, LengthPercentage, LengthPercentageAuto, NodeId, Style,
    TaffyTree,
};

use super::layout::Layout;
use super::values::*;
use crate::widget::Handle;

pub struct LayoutEngine {
    taffy: TaffyTree<Handle<()>>,
    nodes: HashMap<Handle<()>, NodeId>,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::<Handle<()>>::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn add(&mut self, handle: Handle<()>, layout: &Layout) {
        let style = build_style(layout);
        let node = self.taffy.new_leaf(style).unwrap();
        self.nodes.insert(handle, node);
    }

    pub fn add_with_measure(&mut self, handle: Handle<()>, layout: &Layout) {
        let style = build_style(layout);
        let node = self.taffy.new_leaf_with_context(style, handle).unwrap();
        self.nodes.insert(handle, node);
    }

    pub fn remove(&mut self, handle: Handle<()>) {
        if let Some(node) = self.nodes.remove(&handle) {
            self.taffy.remove(node).unwrap();
        }
    }

    pub fn set_layout(&mut self, handle: Handle<()>, layout: &Layout) {
        if let Some(&node) = self.nodes.get(&handle) {
            let style = build_style(layout);
            self.taffy.set_style(node, style).unwrap();
        }
    }

    pub fn set_children(&mut self, parent: Handle<()>, children: &[Handle<()>]) {
        let Some(&parent_node) = self.nodes.get(&parent) else {
            return;
        };
        let child_nodes: Vec<NodeId> = children
            .iter()
            .filter_map(|h| self.nodes.get(h).copied())
            .collect();
        self.taffy.set_children(parent_node, &child_nodes).unwrap();
    }

    pub fn compute(
        &mut self,
        root: Handle<()>,
        width: f32,
        height: f32,
        measure: impl Fn(Handle<()>, Option<f32>, Option<f32>) -> (f32, f32),
    ) {
        let Some(&root_node) = self.nodes.get(&root) else {
            return;
        };
        self.taffy
            .compute_layout_with_measure(
                root_node,
                taffy::geometry::Size {
                    width: AvailableSpace::Definite(width),
                    height: AvailableSpace::Definite(height),
                },
                |known, available, _node_id, ctx, _style| {
                    let Some(handle) = ctx else {
                        return taffy::geometry::Size::ZERO;
                    };
                    let max_w = known.width.or_else(|| match available.width {
                        AvailableSpace::Definite(w) => Some(w),
                        _ => None,
                    });
                    let max_h = known.height.or_else(|| match available.height {
                        AvailableSpace::Definite(h) => Some(h),
                        _ => None,
                    });
                    let (w, h) = measure(*handle, max_w, max_h);
                    taffy::geometry::Size {
                        width: known.width.unwrap_or(w),
                        height: known.height.unwrap_or(h),
                    }
                },
            )
            .unwrap();
    }

    pub fn get_rect(&self, handle: Handle<()>) -> Option<(f32, f32, f32, f32)> {
        let &node = self.nodes.get(&handle)?;
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        let mut current = node;
        loop {
            let layout = self.taffy.layout(current).unwrap();
            x += layout.location.x;
            y += layout.location.y;
            match self.taffy.parent(current) {
                Some(parent) => current = parent,
                None => break,
            }
        }
        let layout = self.taffy.layout(node).unwrap();
        Some((x, y, layout.size.width, layout.size.height))
    }

    pub fn invalidate(&mut self) {
        self.taffy = TaffyTree::<Handle<()>>::new();
        self.nodes.clear();
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn build_style(l: &Layout) -> Style {
    Style {
        display: Display::Flex,
        position: map_position(&l.position),
        flex_direction: map_flex_direction(&l.flex_direction),
        flex_wrap: map_flex_wrap(&l.flex_wrap),
        align_items: map_align_items(&l.align_items),
        align_self: map_align_self(&l.align_self),
        justify_content: map_justify(&l.justify_content),
        flex_grow: l.flex_grow,
        flex_shrink: l.flex_shrink,
        flex_basis: to_dimension(&l.flex_basis),
        size: taffy::geometry::Size {
            width: to_dimension(&l.width),
            height: to_dimension(&l.height),
        },
        min_size: taffy::geometry::Size {
            width: to_dimension(&l.min_w),
            height: to_dimension(&l.min_h),
        },
        max_size: taffy::geometry::Size {
            width: to_dimension(&l.max_w),
            height: to_dimension(&l.max_h),
        },
        aspect_ratio: l.aspect_ratio,
        padding: taffy::geometry::Rect {
            top: to_lp(l.padding[0]),
            right: to_lp(l.padding[1]),
            bottom: to_lp(l.padding[2]),
            left: to_lp(l.padding[3]),
        },
        margin: taffy::geometry::Rect {
            top: to_lpa_auto(l.margin[0]),
            right: to_lpa_auto(l.margin[1]),
            bottom: to_lpa_auto(l.margin[2]),
            left: to_lpa_auto(l.margin[3]),
        },
        inset: taffy::geometry::Rect {
            top: to_lpa_size(&l.inset[0]),
            right: to_lpa_size(&l.inset[1]),
            bottom: to_lpa_size(&l.inset[2]),
            left: to_lpa_size(&l.inset[3]),
        },
        gap: taffy::geometry::Size {
            width: to_lp(l.col_gap),
            height: to_lp(l.row_gap),
        },
        overflow: taffy::geometry::Point {
            x: map_overflow(&l.overflow),
            y: map_overflow(&l.overflow),
        },
        ..Style::DEFAULT
    }
}

fn to_dimension(s: &Size) -> Dimension {
    match s {
        Size::Fixed(v) => Dimension::length(*v),
        Size::Percent(p) => Dimension::percent(*p / 100.0),
        Size::Auto => Dimension::auto(),
    }
}

fn to_lp(v: f32) -> LengthPercentage {
    LengthPercentage::length(v)
}

fn to_lpa_auto(v: f32) -> LengthPercentageAuto {
    if v.is_nan() {
        LengthPercentageAuto::auto()
    } else {
        LengthPercentageAuto::length(v)
    }
}

fn to_lpa_size(s: &Size) -> LengthPercentageAuto {
    match s {
        Size::Fixed(v) => LengthPercentageAuto::length(*v),
        Size::Percent(p) => LengthPercentageAuto::percent(*p / 100.0),
        Size::Auto => LengthPercentageAuto::auto(),
    }
}

fn map_position(p: &Position) -> taffy::Position {
    match p {
        Position::Relative => taffy::Position::Relative,
        Position::Absolute => taffy::Position::Absolute,
    }
}

fn map_flex_direction(d: &FlexDirection) -> taffy::FlexDirection {
    match d {
        FlexDirection::Row => taffy::FlexDirection::Row,
        FlexDirection::Col => taffy::FlexDirection::Column,
        FlexDirection::RowReverse => taffy::FlexDirection::RowReverse,
        FlexDirection::ColReverse => taffy::FlexDirection::ColumnReverse,
    }
}

fn map_flex_wrap(w: &FlexWrap) -> taffy::FlexWrap {
    match w {
        FlexWrap::NoWrap => taffy::FlexWrap::NoWrap,
        FlexWrap::Wrap => taffy::FlexWrap::Wrap,
        FlexWrap::WrapReverse => taffy::FlexWrap::WrapReverse,
    }
}

fn map_align_items(a: &AlignItems) -> Option<taffy::AlignItems> {
    Some(match a {
        AlignItems::Start => taffy::AlignItems::Start,
        AlignItems::Center => taffy::AlignItems::Center,
        AlignItems::End => taffy::AlignItems::End,
        AlignItems::Stretch => taffy::AlignItems::Stretch,
        AlignItems::Baseline => taffy::AlignItems::Baseline,
    })
}

fn map_align_self(a: &AlignSelf) -> Option<taffy::AlignSelf> {
    match a {
        AlignSelf::Auto => None,
        AlignSelf::Start => Some(taffy::AlignSelf::Start),
        AlignSelf::Center => Some(taffy::AlignSelf::Center),
        AlignSelf::End => Some(taffy::AlignSelf::End),
        AlignSelf::Stretch => Some(taffy::AlignSelf::Stretch),
        AlignSelf::Baseline => Some(taffy::AlignSelf::Baseline),
    }
}

fn map_justify(j: &JustifyContent) -> Option<taffy::JustifyContent> {
    Some(match j {
        JustifyContent::Start => taffy::JustifyContent::Start,
        JustifyContent::Center => taffy::JustifyContent::Center,
        JustifyContent::End => taffy::JustifyContent::End,
        JustifyContent::SpaceBetween => taffy::JustifyContent::SpaceBetween,
        JustifyContent::SpaceAround => taffy::JustifyContent::SpaceAround,
        JustifyContent::SpaceEvenly => taffy::JustifyContent::SpaceEvenly,
    })
}

fn map_overflow(o: &Overflow) -> taffy::Overflow {
    match o {
        Overflow::Visible => taffy::Overflow::Visible,
        Overflow::Hidden => taffy::Overflow::Hidden,
        Overflow::Scroll => taffy::Overflow::Scroll,
    }
}
