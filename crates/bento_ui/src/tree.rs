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
    eprintln!("[remove_node] removing node {}", id.0);
    let (children, owner, parent) = TREE.with(|t| {
        let mut t = t.borrow_mut();
        let node = &mut t.nodes[id.0];
        let children = node.children.clone();
        let owner = node.owner.take();
        let parent = node.parent;
        (children, owner, parent)
    });

    drop(owner);

    for child_id in children {
        remove_node(child_id);
    }

    TREE.with(|t| t.borrow_mut().nodes.remove(id.0));

    // mark parent layout dirty
    if let Some(parent_id) = parent {
        mark_layout_dirty(parent_id);
    }

    ui::request_redraw();
}

pub fn append_child(parent: ViewId, child: ViewId) {
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        t.nodes[parent.0].children.push(child);
        t.nodes[child.0].parent = Some(parent);
    });
    mark_layout_dirty(child);

    ui::request_redraw();
}

pub fn reorder_children(parent: ViewId, order: Vec<ViewId>) {
    TREE.with(|t| {
        t.borrow_mut().nodes[parent.0].children = order;
    });
}

pub fn force_layout_dirty(id: ViewId) {
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        t.nodes[id.0].layout_dirty = true;
        t.nodes[id.0].paint_dirty = true;
    });
    let children = TREE.with(|t| t.borrow().nodes[id.0].children.clone());
    for child_id in children {
        force_layout_dirty(child_id);
    }
}

pub fn mark_layout_dirty(id: ViewId) {
    eprintln!("[mark_layout_dirty] node {}", id.0);
    let mut current = Some(id);
    while let Some(node_id) = current {
        let already_dirty = TREE.with(|t| t.borrow().nodes[node_id.0].layout_dirty);
        if already_dirty {
            break;
        }
        TREE.with(|t| {
            let mut t = t.borrow_mut();
            t.nodes[node_id.0].layout_dirty = true;
            t.nodes[node_id.0].paint_dirty = true; // mark paint dirty too
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
    let layout_dirty = TREE.with(|t| t.borrow().nodes[id.0].layout_dirty);
    if !layout_dirty {
        return;
    }

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
    // pass 0: recurse auto-height children first to measure them
    for child_id in children {
        let h_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].height);
        if h_sizing.is_auto() {
            TREE.with(|t| t.borrow_mut().nodes[child_id.0].layout_dirty = true);
            layout(
                *child_id,
                x + padding,
                y + padding,
                inner_w,
                inner_h,
                measurer,
            );
        }
    }

    // pass 1: count fixed and fill children
    let mut fixed_h: f32 = 0.0;
    let mut fill_count: u32 = 0;

    for child_id in children {
        let h_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].height);
        if h_sizing.is_fill() {
            fill_count += 1;
        } else {
            let ch = if h_sizing.is_auto() {
                // already laid out in pass 0, use actual height
                TREE.with(|t| t.borrow().nodes[child_id.0].h)
            } else {
                let (_, ch) = TREE.with(|t| t.borrow().nodes[child_id.0].view.measure(measurer));
                h_sizing.resolve(inner_h, ch)
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

    // compute total for main axis justification
    let total_h: f32 = {
        let mut h = 0.0;
        for child_id in children {
            let h_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].height);
            h += if h_sizing.is_fill() {
                fill_h
            } else if h_sizing.is_auto() {
                TREE.with(|t| t.borrow().nodes[child_id.0].h)
            } else {
                let (_, ch) = TREE.with(|t| t.borrow().nodes[child_id.0].view.measure(measurer));
                h_sizing.resolve(inner_h, ch)
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

    for (i, child_id) in children.iter().enumerate() {
        let (cw_sizing, ch_sizing) = TREE.with(|t| {
            let t = t.borrow();
            let node = &t.nodes[child_id.0];
            (node.width, node.height)
        });

        let (cw_natural, _) = TREE.with(|t| t.borrow().nodes[child_id.0].view.measure(measurer));

        let child_h = if ch_sizing.is_fill() {
            fill_h
        } else if ch_sizing.is_auto() {
            TREE.with(|t| t.borrow().nodes[child_id.0].h)
        } else {
            let (_, ch) = TREE.with(|t| t.borrow().nodes[child_id.0].view.measure(measurer));
            ch_sizing.resolve(inner_h, ch)
        };

        let child_w = match cross_axis {
            CrossAxis::Stretch => inner_w,
            _ => {
                if cw_sizing.is_auto() {
                    TREE.with(|t| t.borrow().nodes[child_id.0].w)
                } else {
                    cw_sizing.resolve(inner_w, cw_natural)
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
            node.layout_dirty = true;
        });

        layout(*child_id, child_x, cursor_y, child_w, child_h, measurer);

        // restore original sizing
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
    // pass 0: recurse auto-width children first to measure them
    for child_id in children {
        let w_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].width);
        let h_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].height);
        if w_sizing.is_auto() || h_sizing.is_auto() {
            TREE.with(|t| t.borrow_mut().nodes[child_id.0].layout_dirty = true);
            layout(
                *child_id,
                x + padding,
                y + padding,
                inner_w,
                inner_h,
                measurer,
            );
        }
    }

    // pass 1: count fixed and fill children
    let mut fixed_w: f32 = 0.0;
    let mut fill_count: u32 = 0;

    for child_id in children {
        let w_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].width);
        if w_sizing.is_fill() {
            fill_count += 1;
        } else {
            let cw = if w_sizing.is_auto() {
                // already laid out in pass 0, use actual width
                TREE.with(|t| t.borrow().nodes[child_id.0].w)
            } else {
                let (cw, _) = TREE.with(|t| t.borrow().nodes[child_id.0].view.measure(measurer));
                w_sizing.resolve(inner_w, cw)
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

    // compute total for main axis justification
    let total_w: f32 = {
        let mut w = 0.0;
        for child_id in children {
            let w_sizing = TREE.with(|t| t.borrow().nodes[child_id.0].width);
            w += if w_sizing.is_fill() {
                fill_w
            } else if w_sizing.is_auto() {
                TREE.with(|t| t.borrow().nodes[child_id.0].w)
            } else {
                let (cw, _) = TREE.with(|t| t.borrow().nodes[child_id.0].view.measure(measurer));
                w_sizing.resolve(inner_w, cw)
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

    for (i, child_id) in children.iter().enumerate() {
        let (cw_sizing, ch_sizing) = TREE.with(|t| {
            let t = t.borrow();
            let node = &t.nodes[child_id.0];
            (node.width, node.height)
        });

        let (_, ch_natural) = TREE.with(|t| t.borrow().nodes[child_id.0].view.measure(measurer));

        let child_w = if cw_sizing.is_fill() {
            fill_w
        } else if cw_sizing.is_auto() {
            TREE.with(|t| t.borrow().nodes[child_id.0].w)
        } else {
            let (cw, _) = TREE.with(|t| t.borrow().nodes[child_id.0].view.measure(measurer));
            cw_sizing.resolve(inner_w, cw)
        };

        let child_h = match cross_axis {
            CrossAxis::Stretch => inner_h,
            _ => {
                if ch_sizing.is_auto() {
                    TREE.with(|t| t.borrow().nodes[child_id.0].h)
                } else {
                    ch_sizing.resolve(inner_h, ch_natural)
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
            node.layout_dirty = true;
        });

        layout(*child_id, cursor_x, child_y, child_w, child_h, measurer);

        // restore original sizing
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
