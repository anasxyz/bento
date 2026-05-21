use bento_shared::{RectNode, SceneNode, SceneNodeId};

use crate::{Ui, widget::Widget};

pub struct Rect {
    id: usize,
    node: Option<SceneNodeId>,

    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
}

impl Rect {
    pub fn new() -> Self {
        Self {
            id: 0,
            node: None,

            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn set_w(&mut self, w: f32) {
        self.w = w;
    }
    pub fn set_h(&mut self, h: f32) {
        self.h = h;
    }
}

impl Widget for Rect {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn name(&self) -> &str {
        "Rect"
    }

    fn build(&mut self, ui: &mut Ui) {
        println!("building rect");
        let mut node = RectNode::new(self.x, self.y, self.w, self.h);
        node.color = self.color;
        let node_id = ui.scene_mut().add_rect(node);
        self.node = Some(node_id);
    }

    fn update(&mut self, ui: &mut Ui) {
        if let Some(node_id) = self.node {
            if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(node_id) {
                r.x = self.x;
                r.y = self.y;
                r.w = self.w;
                r.h = self.h;
                r.color = self.color;
            }
        }
    }

    fn hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }
}
