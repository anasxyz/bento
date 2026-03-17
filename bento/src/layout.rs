use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::element::values::{
    AlignItems, AlignSelf, FlexDirection, FlexWrap, JustifyContent, Overflow, Position, Size,
};
use crate::ui::Ui;
use std::collections::HashMap;
use taffy::prelude::{
    AvailableSpace, Dimension, Display, LengthPercentage, LengthPercentageAuto, NodeId, Style,
    TaffyTree,
};

fn to_dimension(size: &Size) -> Dimension {
    match size {
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

fn to_lpa_size(size: &Size) -> LengthPercentageAuto {
    match size {
        Size::Fixed(v) => LengthPercentageAuto::length(*v),
        Size::Percent(p) => LengthPercentageAuto::percent(*p / 100.0),
        Size::Auto => LengthPercentageAuto::auto(),
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

fn map_flex_wrap(w: &FlexWrap) -> taffy::FlexWrap {
    match w {
        FlexWrap::NoWrap => taffy::FlexWrap::NoWrap,
        FlexWrap::Wrap => taffy::FlexWrap::Wrap,
        FlexWrap::WrapReverse => taffy::FlexWrap::WrapReverse,
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

fn map_overflow(o: &Overflow) -> taffy::Overflow {
    match o {
        Overflow::Visible => taffy::Overflow::Visible,
        Overflow::Hidden => taffy::Overflow::Hidden,
        Overflow::Scroll => taffy::Overflow::Scroll,
    }
}

fn map_position(p: &Position) -> taffy::Position {
    match p {
        Position::Relative => taffy::Position::Relative,
        Position::Absolute => taffy::Position::Absolute,
    }
}

fn build_style(layout: &Layout) -> Style {
    Style {
        display: Display::Flex,
        position: map_position(&layout.position),
        flex_direction: map_flex_direction(&layout.flex_direction),
        flex_wrap: map_flex_wrap(&layout.flex_wrap),
        align_items: map_align_items(&layout.align_items),
        align_self: map_align_self(&layout.align_self),
        justify_content: map_justify(&layout.justify_content),
        flex_grow: layout.flex_grow,
        flex_shrink: layout.flex_shrink,
        flex_basis: to_dimension(&layout.flex_basis),
        size: taffy::geometry::Size {
            width: to_dimension(&layout.width),
            height: to_dimension(&layout.height),
        },
        min_size: taffy::geometry::Size {
            width: to_dimension(&layout.min_w),
            height: to_dimension(&layout.min_h),
        },
        max_size: taffy::geometry::Size {
            width: to_dimension(&layout.max_w),
            height: to_dimension(&layout.max_h),
        },
        aspect_ratio: layout.aspect_ratio,
        padding: taffy::geometry::Rect {
            top: to_lp(layout.padding[0]),
            right: to_lp(layout.padding[1]),
            bottom: to_lp(layout.padding[2]),
            left: to_lp(layout.padding[3]),
        },
        margin: taffy::geometry::Rect {
            top: to_lpa_auto(layout.margin[0]),
            right: to_lpa_auto(layout.margin[1]),
            bottom: to_lpa_auto(layout.margin[2]),
            left: to_lpa_auto(layout.margin[3]),
        },
        inset: taffy::geometry::Rect {
            top: to_lpa_size(&layout.inset[0]),
            right: to_lpa_size(&layout.inset[1]),
            bottom: to_lpa_size(&layout.inset[2]),
            left: to_lpa_size(&layout.inset[3]),
        },
        gap: taffy::geometry::Size {
            width: to_lp(layout.col_gap),
            height: to_lp(layout.row_gap),
        },
        overflow: taffy::geometry::Point {
            x: map_overflow(&layout.overflow_x),
            y: map_overflow(&layout.overflow_y),
        },
        ..Style::DEFAULT
    }
}

fn add_node(
    ui: &Ui,
    handle: Handle<()>,
    taffy: &mut TaffyTree<Handle<()>>,
    nodes: &mut HashMap<Handle<()>, NodeId>,
) -> NodeId {
    let el = match ui.get_any(handle) {
        Some(e) => e,
        None => {
            let node = taffy.new_leaf(Style::DEFAULT).unwrap();
            nodes.insert(handle, node);
            return node;
        }
    };

    let style = build_style(el.layout());
    let has_measure = el.has_measure();

    let node = if has_measure {
        taffy.new_leaf_with_context(style, handle).unwrap()
    } else {
        let children = ui.children(handle).to_vec();
        let ids: Vec<NodeId> = children
            .iter()
            .map(|&c| add_node(ui, c, taffy, nodes))
            .collect();
        taffy.new_with_children(style, &ids).unwrap()
    };

    nodes.insert(handle, node);
    node
}

fn write_back(
    ui: &mut Ui,
    handle: Handle<()>,
    taffy: &TaffyTree<Handle<()>>,
    node: NodeId,
    parent_x: f32,
    parent_y: f32,
) {
    let (x, y, w, h) = {
        let layout = taffy.layout(node).unwrap();
        (
            parent_x + layout.location.x,
            parent_y + layout.location.y,
            layout.size.width,
            layout.size.height,
        )
    };

    if let Some(el) = ui.get_any_mut(handle) {
        let l = el.layout_mut_internal();
        // save previous position before overwriting
        l.prev_x = l.x;
        l.prev_y = l.y;
        l.prev_w = l.w;
        l.prev_h = l.h;
        l.x = x;
        l.y = y;
        l.w = w;
        l.h = h;
    }

    let children = ui.children(handle).to_vec();
    let child_ids = taffy.children(node).unwrap().to_vec();
    for (child_handle, child_node) in children.iter().zip(child_ids.iter()) {
        write_back(ui, *child_handle, taffy, *child_node, x, y);
    }
}

pub fn layout_tree(ui: &mut Ui) {
    let root = match ui.root() {
        Some(r) => r,
        None => return,
    };

    let window_w = ui.window_width as f32;
    let window_h = ui.window_height as f32;

    // take taffy out of ui to avoid borrow conflicts
    let mut taffy = ui.taffy.take().unwrap();
    let mut taffy_nodes = std::mem::take(&mut ui.taffy_nodes);
    let mut taffy_root = ui.taffy_root;

    if taffy_root.is_none() {
        // first frame, build the entire tree from scratch
        let root_node = add_node(ui, root, &mut taffy, &mut taffy_nodes);
        taffy_root = Some(root_node);
    } else {
        // subsequent frames, only update styles for dirty elements
        let dirty_handles: Vec<Handle<()>> = ui
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                let s = s.as_ref()?;
                if s.element.is_dirty() {
                    Some(Handle::new(i as u32, s.generation))
                } else {
                    None
                }
            })
            .collect();

        for handle in dirty_handles {
            if let Some(&node) = taffy_nodes.get(&handle) {
                if let Some(el) = ui.get_any(handle) {
                    let style = build_style(el.layout());
                    taffy.set_style(node, style).unwrap();
                    taffy.mark_dirty(node).unwrap();
                }
            }
        }
    }

    let root_node = taffy_root.unwrap();
    let mut fonts = ui.fonts.take().unwrap();

    taffy
        .compute_layout_with_measure(
            root_node,
            taffy::geometry::Size {
                width: AvailableSpace::Definite(window_w),
                height: AvailableSpace::Definite(window_h),
            },
            |known_dimensions, available_space, _node_id, ctx, _style| {
                let Some(handle) = ctx else {
                    return taffy::geometry::Size::ZERO;
                };
                let el = match ui.get_any(*handle) {
                    Some(e) => e,
                    None => return taffy::geometry::Size::ZERO,
                };
                let max_width = known_dimensions
                    .width
                    .or_else(|| match available_space.width {
                        AvailableSpace::Definite(w) => Some(w),
                        _ => None,
                    });
                let (w, h) = match el.measure(&mut fonts, max_width) {
                    Some(size) => size,
                    None => return taffy::geometry::Size::ZERO,
                };
                taffy::geometry::Size {
                    width: known_dimensions.width.unwrap_or(w + 1.0),
                    height: known_dimensions.height.unwrap_or(h),
                }
            },
        )
        .unwrap();

    ui.fonts = Some(fonts);

    // write computed positions back to elements
    write_back(ui, root, &taffy, root_node, 0.0, 0.0);

    // put taffy back
    ui.taffy = Some(taffy);
    ui.taffy_nodes = taffy_nodes;
    ui.taffy_root = taffy_root;
}

pub fn invalidate_layout(ui: &mut Ui) {
    ui.taffy = Some(TaffyTree::new());
    ui.taffy_nodes.clear();
    ui.taffy_root = None;
}
