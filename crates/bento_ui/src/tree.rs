use bento_wgpu::DrawList;
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
        match &node.ntype {
            NodeType::Text(text) => text.render(draw_list),
        }
        node.children.clone()
    });

    for child_id in children {
        render(child_id, draw_list);
    }
}
