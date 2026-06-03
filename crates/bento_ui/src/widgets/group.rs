use bento_wgpu::DrawCommand;

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

    fn render(&self, _x: f32, _y: f32, _w: f32, _h: f32) -> Vec<DrawCommand> {
        Vec::new()
    }

    fn build(self: Box<Self>) -> ViewId {
        let child_ids: Vec<ViewId> = self.children.into_iter().map(|c| c.build()).collect();

        let id = tree::add_node(Node {
            view: Box::new(Group {
                children: Vec::new(),
            }),
            parent: None,
            children: Vec::new(), 
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            handlers: Vec::new(),
            owner: None,
            paint_dirty: true,
            cache: Vec::new(),
            paint_subscriber: None,
            layout_dirty: true,
        });

        for child_id in child_ids {
            tree::append_child(id, child_id);
        }

        id
    }
}

pub fn group() -> Group {
    Group {
        children: Vec::new(),
    }
}
