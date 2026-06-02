use bento_wgpu::{DrawList, TextMeasurer};
use slab::Slab;
use std::cell::RefCell;

use crate::{
    node::{Node, NodeType},
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

pub fn render(id: ViewId, draw_list: &mut DrawList) {
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

pub fn layout(id: ViewId, x: f32, y: f32, measurer: &mut TextMeasurer) {
    TREE.with(|t| {
        let mut t = t.borrow_mut();
        let node = &mut t.nodes[id.0];
        let (w, h) = node.view.measure(measurer);
        node.x = x;
        node.y = y;
        node.w = w;
        node.h = h;
        let children = node.children.clone();
        drop(t);

        let mut child_y = y;
        for child_id in children {
            layout(child_id, x, child_y, measurer);
            // advance y for next child
            TREE.with(|t| {
                child_y += t.borrow().nodes[child_id.0].h;
            });
        }
    });
}

pub fn print_tree(id: ViewId, depth: usize) {
    TREE.with(|t| {
        let t = t.borrow();
        let node = &t.nodes[id.0];
        let indent = "  ".repeat(depth);
        println!("{}{} (id: {}) x: {} y: {} w: {} h: {}", indent, node.view.name(), id.0, node.x, node.y, node.w, node.h);
        let children = node.children.clone();
        drop(t);
        for child_id in children {
            print_tree(child_id, depth + 1);
        }
    });
}
