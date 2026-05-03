use crate::pipelines::rect::RectInstance;

pub enum Node {
    Rect(RectInstance),
}

pub struct Scene {
    nodes: Vec<Node>,
}

impl Scene {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_rect(&mut self, rect: RectInstance) {
        self.nodes.push(Node::Rect(rect));
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
}
