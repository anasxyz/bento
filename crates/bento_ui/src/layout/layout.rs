use super::tree::LayoutTree;
use crate::layout::Overflow;
use crate::layout::convert::{to_taffy_style, write_taffy_output};
use crate::widget::Widget;
use bento_shared::TextMeasurer;
use taffy::prelude::*;

pub fn run_layout(
    tree: &mut LayoutTree,
    widgets: &mut [Option<crate::ui::Slot>],
    measurer: &mut dyn TextMeasurer,
) {
    // sync widget layout properties into layout tree nodes
    for node in &mut tree.nodes {
        if node.slot == usize::MAX {
            continue;
        }
        if let Some(Some(slot)) = widgets.get(node.slot) {
            node.layout = slot.widget.base().layout.clone();
        }
    }

    let roots: Vec<usize> = tree
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.parent.is_none() && n.slot != usize::MAX)
        .map(|(i, _)| i)
        .collect();

    for root in roots {
        if tree.nodes[root].dirty {
            compute_subtree(tree, root, 0.0, 0.0, widgets, measurer);
        }
    }
}

fn compute_subtree(
    tree: &mut LayoutTree,
    node_index: usize,
    parent_x: f32,
    parent_y: f32,
    widgets: &mut [Option<crate::ui::Slot>],
    measurer: &mut dyn TextMeasurer,
) {
    println!("compute_subtree called for node {}", node_index);
    let children = tree.nodes[node_index].children.clone();

    let mut taffy: TaffyTree<usize> = TaffyTree::new();

    let child_taffy_nodes: Vec<NodeId> = children
        .iter()
        .map(|&ci| {
            let style = to_taffy_style(&tree.nodes[ci].layout);
            taffy.new_leaf_with_context(style, ci).unwrap()
        })
        .collect();

    let parent_w = tree.nodes[node_index].layout.w;
    let parent_h = tree.nodes[node_index].layout.h;
    let parent_abs_x = parent_x + tree.nodes[node_index].layout.x;
    let parent_abs_y = parent_y + tree.nodes[node_index].layout.y;
    let root_style = Style {
        size: Size {
            width: Dimension::length(parent_w),
            height: Dimension::length(parent_h),
        },
        ..to_taffy_style(&tree.nodes[node_index].layout)
    };

    let taffy_root = taffy
        .new_with_children(root_style, &child_taffy_nodes)
        .unwrap();

    let overflow_x = tree.nodes[node_index].layout.overflow_x;
    let overflow_y = tree.nodes[node_index].layout.overflow_y;

    taffy
        .compute_layout_with_measure(
            taffy_root,
            Size {
                width: match overflow_x {
                    Overflow::Scroll | Overflow::Visible => AvailableSpace::MaxContent,
                    Overflow::Hidden => AvailableSpace::Definite(parent_w),
                },
                height: match overflow_y {
                    Overflow::Scroll | Overflow::Visible => AvailableSpace::MaxContent,
                    Overflow::Hidden => AvailableSpace::Definite(parent_h),
                },
            },
            |known_dimensions, _available, _node_id, context, _style| {
                let ci = match context {
                    Some(i) => *i,
                    None => {
                        return Size {
                            width: 0.0,
                            height: 0.0,
                        };
                    }
                };
                let slot = tree.nodes[ci].slot;
                if let Some(Some(s)) = widgets.get_mut(slot) {
                    let (w, h) =
                        s.widget
                            .measure(known_dimensions.width, known_dimensions.height, measurer);
                    Size {
                        width: w,
                        height: h,
                    }
                } else {
                    Size {
                        width: 0.0,
                        height: 0.0,
                    }
                }
            },
        )
        .unwrap();

    for (i, &ci) in children.iter().enumerate() {
        let taffy_layout = taffy.layout(child_taffy_nodes[i]).unwrap();
        write_taffy_output(
            taffy_layout,
            parent_abs_x,
            parent_abs_y,
            &mut tree.nodes[ci].layout,
        );
        tree.nodes[ci].dirty = false;

        if !tree.nodes[ci].children.is_empty() {
            let child_x = tree.nodes[ci].layout.x;
            let child_y = tree.nodes[ci].layout.y;
            compute_subtree(tree, ci, child_x, child_y, widgets, measurer);
        }
    }

    tree.nodes[node_index].dirty = false;
}
