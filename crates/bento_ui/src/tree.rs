use bento_wgpu::{DrawList, TextMeasurer};
use slab::Slab;
use std::{cell::RefCell, rc::Rc};

use crate::{
    layout::{CrossAxis, Direction, MainAxis, Size},
    node::{EventHandler, Node, NodeType},
    reactive::{owner::Owner, runtime},
    ui,
    view::ViewId,
};

struct Tree {
    nodes: Slab<Node>,
}

impl Tree {
    fn new() -> Self {
        Self { nodes: Slab::new() }
    }
}

thread_local! {
    static TREE: RefCell<Tree> = RefCell::new(Tree::new());
}

pub fn add_node(node: Node) -> ViewId {
    let id = TREE.with(|t| ViewId(t.borrow_mut().nodes.insert(node)));

    // set up per-node paint subscriber
    let sub_id = runtime::create_subscriber(Rc::new(move || {
        TREE.with(|t| {
            if let Some(node) = t.borrow_mut().nodes.get_mut(id.0) {
                node.paint_dirty = true;
            }
        });
        mark_layout_dirty(id);
        ui::request_redraw();
    }));

    TREE.with(|t| {
        if let Some(node) = t.borrow_mut().nodes.get_mut(id.0) {
            node.paint_subscriber = Some(sub_id);
        }
    });

    // subscribe signals by running render once inside observer
    runtime::push_observer(sub_id);
    TREE.with(|t| t.borrow().nodes[id.0].view.render(0.0, 0.0, 0.0, 0.0));
    runtime::pop_observer();

    // leave paint_dirty = true so first real render pass computes correct positions
    id
}

pub fn remove_node(id: ViewId) {
    let parent = TREE.with(|t| t.borrow().nodes[id.0].parent);
    remove_node_inner(id);
    if let Some(parent_id) = parent {
        mark_layout_dirty(parent_id);
    }
    ui::request_redraw();
}

fn remove_node_inner(id: ViewId) {
    let (children, owner) = TREE.with(|t| {
        let mut t = t.borrow_mut();
        let node = &mut t.nodes[id.0];
        let children = node.children.clone();
        let owner = node.owner.take();
        (children, owner)
    });

    drop(owner);

    for child_id in children {
        remove_node_inner(child_id);
    }

    TREE.with(|t| t.borrow_mut().nodes.remove(id.0));
}

pub fn append_child(parent: ViewId, child: ViewId) {
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        t.nodes[parent.0].children.push(child);
        t.nodes[child.0].parent = Some(parent);
    });
    mark_layout_dirty(parent);
    ui::request_redraw();
}

pub fn reorder_children(parent: ViewId, order: Vec<ViewId>) {
    TREE.with(|t| {
        t.borrow_mut().nodes[parent.0].children = order;
    });
}

pub fn mark_layout_dirty(id: ViewId) {
    let mut current = Some(id);
    while let Some(node_id) = current {
        let already_dirty = TREE.with(|t| t.borrow().nodes[node_id.0].layout_dirty);
        if already_dirty {
            break;
        }
        TREE.with(|t| {
            t.borrow_mut().nodes[node_id.0].layout_dirty = true;
        });
        current = TREE.with(|t| t.borrow().nodes[node_id.0].parent);
    }
}

pub fn store_owner(id: ViewId, owner: Owner) {
    TREE.with(|t| {
        t.borrow_mut().nodes[id.0].owner = Some(owner);
    });
}

pub fn render(id: ViewId, draw_list: &mut DrawList) {
    let (paint_dirty, x, y, w, h, children) = TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        (
            node.paint_dirty,
            node.x,
            node.y,
            node.w,
            node.h,
            node.children.clone(),
        )
    });

    if paint_dirty {
        let sub_id = TREE.with(|t| t.borrow().nodes[id.0].paint_subscriber);

        let commands = if let Some(sub_id) = sub_id {
            runtime::push_observer(sub_id);
            let cmds = TREE.with(|t| t.borrow().nodes[id.0].view.render(x, y, w, h));
            runtime::pop_observer();
            cmds
        } else {
            TREE.with(|t| t.borrow().nodes[id.0].view.render(x, y, w, h))
        };

        for cmd in &commands {
            draw_list.push_command(cmd.clone());
        }

        TREE.with(|t| {
            let mut t = t.borrow_mut();
            let node = &mut t.nodes[id.0];
            node.cache = commands;
            node.paint_dirty = false;
        });
    } else {
        let cache = TREE.with(|t| t.borrow().nodes[id.0].cache.clone());
        for cmd in cache {
            draw_list.push_command(cmd);
        }
    }

    for child_id in children {
        render(child_id, draw_list);
    }
}

pub fn layout(
    id: ViewId,
    x: f32,
    y: f32,
    available_w: f32,
    available_h: f32,
    measurer: &mut TextMeasurer,
) {
    let (layout_dirty, last_w, last_h) = TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        (
            node.layout_dirty,
            node.last_available_w,
            node.last_available_h,
        )
    });
    if !layout_dirty && last_w == available_w && last_h == available_h {
        return;
    }
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        let node = &mut t.nodes[id.0];
        node.last_available_w = available_w;
        node.last_available_h = available_h;
    });
    layout_node(id, x, y, available_w, available_h, measurer);
}

fn layout_node(
    id: ViewId,
    x: f32,
    y: f32,
    available_w: f32,
    available_h: f32,
    measurer: &mut TextMeasurer,
) {
    let (width_sizing, height_sizing) = TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        (node.width, node.height)
    });

    let children = TREE.with(|t| t.borrow().nodes[id.0].children.clone());

    let container = TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        node.view.as_container().map(|c| {
            (
                c.direction(),
                c.gap(),
                c.padding(),
                c.main_axis(),
                c.cross_axis(),
            )
        })
    });

    let (direction, gap, padding, main_axis, cross_axis) = match container {
        Some(c) => c,
        None => {
            let content_size = TREE.with(|t| t.borrow().nodes[id.0].view.measure(measurer));
            let my_w = width_sizing.resolve(available_w, content_size.0);
            let my_h = height_sizing.resolve(available_h, content_size.1);
            TREE.with(|t| {
                let mut t = t.borrow_mut();
                let node = &mut t.nodes[id.0];
                node.x = x;
                node.y = y;
                node.w = my_w;
                node.h = my_h;
                node.layout_dirty = false;
                node.paint_dirty = true;
            });
            return;
        }
    };

    let content_size = if children.is_empty() {
        TREE.with(|t| t.borrow().nodes[id.0].view.measure(measurer))
    } else {
        (0.0, 0.0)
    };

    let my_w = width_sizing.resolve(available_w, content_size.0);
    let my_h = height_sizing.resolve(available_h, content_size.1);

    let inner_w = match width_sizing {
        Size::Auto => available_w - padding * 2.0,
        _ => my_w - padding * 2.0,
    };
    let inner_h = match height_sizing {
        Size::Auto => available_h - padding * 2.0,
        _ => my_h - padding * 2.0,
    };

    if !children.is_empty() {
        match direction {
            Direction::Column => layout_column(
                id,
                x,
                y,
                my_w,
                my_h,
                inner_w,
                inner_h,
                width_sizing,
                height_sizing,
                &children,
                gap,
                padding,
                main_axis,
                cross_axis,
                measurer,
            ),
            Direction::Row => layout_row(
                id,
                x,
                y,
                my_w,
                my_h,
                inner_w,
                inner_h,
                width_sizing,
                height_sizing,
                &children,
                gap,
                padding,
                main_axis,
                cross_axis,
                measurer,
            ),
        }
    } else {
        TREE.with(|t| {
            let mut t = t.borrow_mut();
            let node = &mut t.nodes[id.0];
            node.x = x;
            node.y = y;
            node.w = my_w;
            node.h = my_h;
            node.layout_dirty = false;
            node.paint_dirty = true;
        });
    }
}

fn layout_column(
    id: ViewId,
    x: f32,
    y: f32,
    my_w: f32,
    my_h: f32,
    inner_w: f32,
    inner_h: f32,
    width_sizing: Size,
    height_sizing: Size,
    children: &[ViewId],
    gap: f32,
    padding: f32,
    main_axis: MainAxis,
    cross_axis: CrossAxis,
    measurer: &mut TextMeasurer,
) {
    // cache measure results once for all children
    let measures: Vec<(f32, f32)> = children
        .iter()
        .map(|child_id| TREE.with(|t| t.borrow().nodes[child_id.0].view.measure(measurer)))
        .collect();

    // pass 0: measure all auto-height children
    for (i, child_id) in children.iter().enumerate() {
        let h_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].height);
        if h_sizing.is_auto() {
            layout_node(
                *child_id,
                x + padding,
                y + padding,
                inner_w,
                inner_h,
                measurer,
            );
        }
    }

    // pass 1: compute fill height
    let mut fixed_h: f32 = 0.0;
    let mut fill_count: u32 = 0;

    for (i, child_id) in children.iter().enumerate() {
        let h_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].height);
        if h_sizing.is_fill() {
            fill_count += 1;
        } else {
            let ch = if h_sizing.is_auto() {
                TREE.with(|t| t.borrow().nodes[child_id.0].h)
            } else {
                h_sizing.resolve(inner_h, measures[i].1)
            };
            fixed_h += ch;
        }
    }

    let gaps_total = gap * (children.len().saturating_sub(1)) as f32;
    let remaining = (inner_h - fixed_h - gaps_total).max(0.0);
    let fill_h = if fill_count > 0 {
        remaining / fill_count as f32
    } else {
        0.0
    };

    let total_h: f32 = {
        let mut h = 0.0;
        for (i, child_id) in children.iter().enumerate() {
            let h_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].height);
            h += if h_sizing.is_fill() {
                fill_h
            } else if h_sizing.is_auto() {
                TREE.with(|t| t.borrow().nodes[child_id.0].h)
            } else {
                h_sizing.resolve(inner_h, measures[i].1)
            };
        }
        h + gaps_total
    };

    let mut cursor_y = y + padding;
    cursor_y += match main_axis {
        MainAxis::Start => 0.0,
        MainAxis::Center => (inner_h - total_h) / 2.0,
        MainAxis::End => inner_h - total_h,
        MainAxis::SpaceBetween => 0.0,
    };

    let space_between = if main_axis == MainAxis::SpaceBetween && children.len() > 1 {
        (inner_h - total_h + gaps_total) / (children.len() - 1) as f32
    } else {
        gap
    };

    // pass 2: position all children
    for (i, child_id) in children.iter().enumerate() {
        let (cw_sizing, ch_sizing) = TREE.with(|t| {
            let t = t.borrow();
            let node = &t.nodes[child_id.0];
            (node.width, node.height)
        });

        let child_h = if ch_sizing.is_fill() {
            fill_h
        } else if ch_sizing.is_auto() {
            TREE.with(|t| t.borrow().nodes[child_id.0].h)
        } else {
            ch_sizing.resolve(inner_h, measures[i].1)
        };

        let child_w = match cross_axis {
            CrossAxis::Stretch if !width_sizing.is_auto() => inner_w,
            _ => {
                if cw_sizing.is_auto() {
                    TREE.with(|t| t.borrow().nodes[child_id.0].w)
                } else {
                    cw_sizing.resolve(inner_w, measures[i].0)
                }
            }
        };

        let child_x = match cross_axis {
            CrossAxis::Start | CrossAxis::Stretch => x + padding,
            CrossAxis::Center => x + padding + (inner_w - child_w) / 2.0,
            CrossAxis::End => x + padding + inner_w - child_w,
        };

        TREE.with(|t| {
            let mut t = t.borrow_mut();
            let node = &mut t.nodes[child_id.0];
            node.width = Size::Fixed(child_w);
            node.height = Size::Fixed(child_h);
        });

        layout_node(*child_id, child_x, cursor_y, child_w, child_h, measurer);

        TREE.with(|t| {
            let mut t = t.borrow_mut();
            let node = &mut t.nodes[child_id.0];
            node.width = cw_sizing;
            node.height = ch_sizing;
        });

        cursor_y += child_h;
        if i < children.len() - 1 {
            cursor_y += space_between;
        }
    }

    let content_w = TREE.with(|t| {
        let t = t.borrow();
        children
            .iter()
            .map(|cid| t.nodes[cid.0].w)
            .fold(0.0f32, f32::max)
    }) + padding * 2.0;

    let content_h = cursor_y - y;

    TREE.with(|t| {
        let mut t = t.borrow_mut();
        let node = &mut t.nodes[id.0];
        node.x = x;
        node.y = y;
        node.w = if width_sizing.is_auto() {
            content_w
        } else {
            my_w
        };
        node.h = if height_sizing.is_auto() {
            content_h
        } else {
            my_h
        };
        node.layout_dirty = false;
        node.paint_dirty = true;
    });
}

fn layout_row(
    id: ViewId,
    x: f32,
    y: f32,
    my_w: f32,
    my_h: f32,
    inner_w: f32,
    inner_h: f32,
    width_sizing: Size,
    height_sizing: Size,
    children: &[ViewId],
    gap: f32,
    padding: f32,
    main_axis: MainAxis,
    cross_axis: CrossAxis,
    measurer: &mut TextMeasurer,
) {
    // cache measure results once for all children
    let measures: Vec<(f32, f32)> = children
        .iter()
        .map(|child_id| TREE.with(|t| t.borrow().nodes[child_id.0].view.measure(measurer)))
        .collect();

    // pass 0: measure all auto children
    for (i, child_id) in children.iter().enumerate() {
        let w_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].width);
        let h_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].height);
        if w_sizing.is_auto() || h_sizing.is_auto() {
            layout_node(
                *child_id,
                x + padding,
                y + padding,
                inner_w,
                inner_h,
                measurer,
            );
        }
    }

    // pass 1: compute fill width
    let mut fixed_w: f32 = 0.0;
    let mut fill_count: u32 = 0;

    for (i, child_id) in children.iter().enumerate() {
        let w_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].width);
        if w_sizing.is_fill() {
            fill_count += 1;
        } else {
            let cw = if w_sizing.is_auto() {
                TREE.with(|t| t.borrow().nodes[child_id.0].w)
            } else {
                w_sizing.resolve(inner_w, measures[i].0)
            };
            fixed_w += cw;
        }
    }

    let gaps_total = gap * (children.len().saturating_sub(1)) as f32;
    let remaining = (inner_w - fixed_w - gaps_total).max(0.0);
    let fill_w = if fill_count > 0 {
        remaining / fill_count as f32
    } else {
        0.0
    };

    let total_w: f32 = {
        let mut w = 0.0;
        for (i, child_id) in children.iter().enumerate() {
            let w_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].width);
            w += if w_sizing.is_fill() {
                fill_w
            } else if w_sizing.is_auto() {
                TREE.with(|t| t.borrow().nodes[child_id.0].w)
            } else {
                w_sizing.resolve(inner_w, measures[i].0)
            };
        }
        w + gaps_total
    };

    let mut cursor_x = x + padding;
    cursor_x += match main_axis {
        MainAxis::Start => 0.0,
        MainAxis::Center => (inner_w - total_w) / 2.0,
        MainAxis::End => inner_w - total_w,
        MainAxis::SpaceBetween => 0.0,
    };

    let space_between = if main_axis == MainAxis::SpaceBetween && children.len() > 1 {
        (inner_w - total_w + gaps_total) / (children.len() - 1) as f32
    } else {
        gap
    };

    // pass 2: position all children
    for (i, child_id) in children.iter().enumerate() {
        let (cw_sizing, ch_sizing) = TREE.with(|t| {
            let t = t.borrow();
            let node = &t.nodes[child_id.0];
            (node.width, node.height)
        });

        let child_w = if cw_sizing.is_fill() {
            fill_w
        } else if cw_sizing.is_auto() {
            TREE.with(|t| t.borrow().nodes[child_id.0].w)
        } else {
            cw_sizing.resolve(inner_w, measures[i].0)
        };

        let child_h = match cross_axis {
            CrossAxis::Stretch if !height_sizing.is_auto() => inner_h,
            _ => {
                if ch_sizing.is_auto() {
                    TREE.with(|t| t.borrow().nodes[child_id.0].h)
                } else {
                    ch_sizing.resolve(inner_h, measures[i].1)
                }
            }
        };

        let child_y = match cross_axis {
            CrossAxis::Start | CrossAxis::Stretch => y + padding,
            CrossAxis::Center => y + padding + (inner_h - child_h) / 2.0,
            CrossAxis::End => y + padding + inner_h - child_h,
        };

        TREE.with(|t| {
            let mut t = t.borrow_mut();
            let node = &mut t.nodes[child_id.0];
            node.width = Size::Fixed(child_w);
            node.height = Size::Fixed(child_h);
        });

        layout_node(*child_id, cursor_x, child_y, child_w, child_h, measurer);

        TREE.with(|t| {
            let mut t = t.borrow_mut();
            let node = &mut t.nodes[child_id.0];
            node.width = cw_sizing;
            node.height = ch_sizing;
        });

        cursor_x += child_w;
        if i < children.len() - 1 {
            cursor_x += space_between;
        }
    }

    let content_h = TREE.with(|t| {
        let t = t.borrow();
        children
            .iter()
            .map(|cid| t.nodes[cid.0].h)
            .fold(0.0f32, f32::max)
    }) + padding * 2.0;

    let content_w = cursor_x - x;

    TREE.with(|t| {
        let mut t = t.borrow_mut();
        let node = &mut t.nodes[id.0];
        node.x = x;
        node.y = y;
        node.w = if width_sizing.is_auto() {
            content_w
        } else {
            my_w
        };
        node.h = if height_sizing.is_auto() {
            content_h
        } else {
            my_h
        };
        node.layout_dirty = false;
        node.paint_dirty = true;
    });
}

pub(crate) fn add_handler<E: 'static>(id: ViewId, f: impl Fn(&E) + 'static) {
    TREE.with(|t| {
        t.borrow_mut().nodes[id.0].handlers.push(EventHandler {
            type_id: std::any::TypeId::of::<E>(),
            handler: Rc::new(move |any| {
                if let Some(event) = any.downcast_ref::<E>() {
                    f(event);
                }
            }),
        });
    });
}

pub fn set_width(id: ViewId, size: Size) {
    TREE.with(|t| t.borrow_mut().nodes[id.0].width = size);
}

pub fn set_height(id: ViewId, size: Size) {
    TREE.with(|t| t.borrow_mut().nodes[id.0].height = size);
}

pub(crate) fn dispatch<E: 'static>(id: ViewId, event: &E) {
    let type_id = std::any::TypeId::of::<E>();
    let handlers: Vec<Rc<dyn Fn(&dyn std::any::Any)>> = TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        node.handlers
            .iter()
            .filter(|h| h.type_id == type_id)
            .map(|h| h.handler.clone())
            .collect()
    });

    for handler in handlers {
        handler(event as &dyn std::any::Any);
    }
}

pub(crate) fn hit_test(id: ViewId, x: f32, y: f32) -> Option<ViewId> {
    let (node_x, node_y, node_w, node_h, children) = TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        (node.x, node.y, node.w, node.h, node.children.clone())
    });

    // check children first (front to back)
    for child_id in children.iter().rev() {
        if let Some(hit) = hit_test(*child_id, x, y) {
            return Some(hit);
        }
    }

    // then check self
    if x >= node_x && x <= node_x + node_w && y >= node_y && y <= node_y + node_h {
        return Some(id);
    }

    None
}

pub(crate) fn print_tree(id: ViewId, depth: usize) {
    TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        let indent = "  ".repeat(depth);
        println!(
            "{}{} (id: {}) x: {} y: {} w: {} h: {}",
            indent,
            node.view.name(),
            id.0,
            node.x,
            node.y,
            node.w,
            node.h
        );
        let children = node.children.clone();
        drop(t);
        for child_id in children {
            print_tree(child_id, depth + 1);
        }
    });
}
