use super::tree::LayoutTree;
use crate::layout::convert::write_taffy_output;
use crate::layout::types::Overflow;
use bento_shared::TextMeasurer;
use taffy::prelude::*;

pub fn run_layout(
    tree: &mut LayoutTree,
    widgets: &mut [Option<crate::ui::Slot>],
    measurer: &mut dyn TextMeasurer,
) {
    // sync widget layouts into layout nodes
    for node in &mut tree.nodes {
        if node.slot == usize::MAX {
            continue;
        }
        if let Some(taffy_id) = node.taffy_id {
            let style = tree.taffy.style(taffy_id).unwrap();
            println!("node {} align_items: {:?}", node.slot, style.align_items);
        }
        if let Some(Some(slot)) = widgets.get(node.slot) {
            let new_layout = slot.widget.base().layout.clone();
            if !new_layout.inputs_equal(&node.layout) {
                node.layout = new_layout;
                node.dirty = true;
            }
        }
    }

    // collect dirty roots BEFORE sync clears dirty flags
    let roots: Vec<usize> = tree
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.parent.is_none() && n.slot != usize::MAX && n.dirty)
        .map(|(i, _)| i)
        .collect();

    if roots.is_empty() {
        return;
    }

    let dirty_count = tree
        .nodes
        .iter()
        .filter(|n| n.dirty && n.slot != usize::MAX)
        .count();

    // sync styles into taffy and mark nodes clean
    let t1 = std::time::Instant::now();
    let any = tree.sync_styles();
    if !any {
        return;
    }

    for root_index in roots {
        let Some(root_taffy_id) = tree.nodes[root_index].taffy_id else {
            continue;
        };
        let parent_w = tree.nodes[root_index].layout.w;
        let parent_h = tree.nodes[root_index].layout.h;
        let overflow_x = tree.nodes[root_index].layout.overflow_x;
        let overflow_y = tree.nodes[root_index].layout.overflow_y;
        let mut nodes_measured = 0u32;

        let t2 = std::time::Instant::now();
        tree.taffy
            .compute_layout_with_measure(
                root_taffy_id,
                Size {
                    width: match overflow_x {
                        Overflow::Scroll => AvailableSpace::MaxContent,
                        _ => AvailableSpace::Definite(parent_w),
                    },
                    height: match overflow_y {
                        Overflow::Scroll => AvailableSpace::MaxContent,
                        _ => AvailableSpace::Definite(parent_h),
                    },
                },
                |known_dimensions, _available, _node_id, context, _style| {
                    nodes_measured += 1;
                    let index = match context {
                        Some(i) => *i,
                        None => {
                            return Size {
                                width: 0.0,
                                height: 0.0,
                            };
                        }
                    };
                    let slot = tree.nodes[index].slot;
                    if let Some(Some(s)) = widgets.get_mut(slot) {
                        let (w, h) = s.widget.measure(
                            known_dimensions.width,
                            known_dimensions.height,
                            measurer,
                        );
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

        write_results(tree, root_index, 0.0, 0.0);
    }
}

fn write_results(tree: &mut LayoutTree, index: usize, parent_x: f32, parent_y: f32) {
    let Some(taffy_id) = tree.nodes[index].taffy_id else {
        return;
    };
    let taffy_layout = tree.taffy.layout(taffy_id).unwrap();
    write_taffy_output(
        taffy_layout,
        parent_x,
        parent_y,
        &mut tree.nodes[index].layout,
    );
    tree.nodes[index].dirty = false;

    let abs_x = tree.nodes[index].layout.x;
    let abs_y = tree.nodes[index].layout.y;
    let children = tree.nodes[index].children.clone();
    for ci in children {
        write_results(tree, ci, abs_x, abs_y);
    }
}
