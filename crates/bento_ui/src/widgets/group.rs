use crate::{
    node::Node,
    tree,
    view::{View, ViewId},
};

pub struct Group {
    children: Vec<Box<dyn View>>,
}

impl Group {
    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl View for Group {
    fn name(&self) -> &'static str {
        "Group"
    }

    fn build(self: Box<Self>) -> ViewId {
        let child_ids: Vec<ViewId> = self.children.into_iter().map(|c| c.build()).collect();
        tree::add_node(Node {
            view: Box::new(Group {
                children: Vec::new(),
            }),
            children: child_ids,
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
        })
    }
}

pub fn group() -> Group {
    Group {
        children: Vec::new(),
    }
}
