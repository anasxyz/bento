use bento_wgpu::{DrawList, TextMeasurer};
use slab::Slab;
use std::cell::RefCell;

use crate::{
    node::{EventHandler, Node, NodeType},
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

pub(crate) fn add_node(node: Node) -> ViewId {
    TREE.with(|t| {
        let id = t.borrow_mut().nodes.insert(node);
        ViewId(id)
    })
}

pub(crate) fn render(id: ViewId, draw_list: &mut DrawList) {
    let children = TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        node.view.render(node.x, node.y, node.w, node.h, draw_list);
        node.children.clone()
    });

    for child_id in children {
        render(child_id, draw_list);
    }
}

pub(crate) fn layout(id: ViewId, x: f32, y: f32, measurer: &mut TextMeasurer) {
    let children = TREE.with(|t| t.borrow().nodes[id.0].children.clone());

    let mut child_y = y;
    for child_id in &children {
        layout(*child_id, x, child_y, measurer);
        TREE.with(|t| {
            child_y += t.borrow().nodes[child_id.0].h;
        });
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
    });
}

pub(crate) fn add_handler<E: 'static>(id: ViewId, f: impl Fn(&E) + 'static) {
    TREE.with(|t| {
        t.borrow_mut().nodes[id.0].handlers.push(EventHandler {
            type_id: std::any::TypeId::of::<E>(),
            handler: Box::new(move |any| {
                if let Some(event) = any.downcast_ref::<E>() {
                    f(event);
                }
            }),
        });
    });
}

pub(crate) fn dispatch<E: 'static>(id: ViewId, event: &E) {
    let type_id = std::any::TypeId::of::<E>();
    TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        for handler in &node.handlers {
            if handler.type_id == type_id {
                (handler.handler)(event);
            }
        }
    });
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
