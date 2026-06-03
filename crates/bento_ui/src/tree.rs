use bento_wgpu::{DrawList, TextMeasurer};
use slab::Slab;
use std::{cell::RefCell, rc::Rc};

use crate::{
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
}

pub fn append_child(parent: ViewId, child: ViewId) {
    eprintln!("[append_child] parent:{} child:{}", parent.0, child.0);
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        t.nodes[parent.0].children.push(child);
        t.nodes[child.0].parent = Some(parent);
    });
    mark_layout_dirty(child);
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

    eprintln!(
        "[render] node {} dirty:{} x:{} y:{} w:{} h:{}",
        id.0, paint_dirty, x, y, w, h
    );
}

pub fn layout(id: ViewId, x: f32, y: f32, measurer: &mut TextMeasurer) {
    let layout_dirty = TREE.with(|t| t.borrow().nodes[id.0].layout_dirty);
    if !layout_dirty {
        return;
    }

    let children = TREE.with(|t| t.borrow().nodes[id.0].children.clone());

    let mut child_y = y;
    for child_id in &children {
        let old_x = TREE.with(|t| t.borrow().nodes[child_id.0].x);
        let old_y = TREE.with(|t| t.borrow().nodes[child_id.0].y);

        TREE.with(|t| t.borrow_mut().nodes[child_id.0].layout_dirty = true);
        layout(*child_id, x, child_y, measurer);

        let (new_x, new_y) = TREE.with(|t| {
            let t = t.borrow();
            (t.nodes[child_id.0].x, t.nodes[child_id.0].y)
        });

        if new_x != old_x || new_y != old_y {
            TREE.with(|t| t.borrow_mut().nodes[child_id.0].paint_dirty = true);
        }

        child_y += TREE.with(|t| t.borrow().nodes[child_id.0].h);
    }

    TREE.with(|t| {
        let mut t = t.borrow_mut();
        let (w, h) = if children.is_empty() {
            t.nodes[id.0].view.measure(measurer)
        } else {
            children.iter().fold((0.0f32, 0.0f32), |acc, child_id| {
                let child = &t.nodes[child_id.0];
                (acc.0.max(child.w), acc.1 + child.h)
            })
        };
        let node = &mut t.nodes[id.0];
        node.x = x;
        node.y = y;
        node.w = w;
        node.h = h;
        node.layout_dirty = false;

        eprintln!("[layout] node {} x:{} y:{} w:{} h:{}", id.0, x, y, w, h);
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
