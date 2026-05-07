use bento_shared::scene::{Node, RectNode, Scene, SceneNodeId};
use crate::widget::Widget;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub radii: [f32; 4],
    pub opacity: f32,
    id: Option<SceneNodeId>,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            x, y, w, h,
            color: [1.0, 1.0, 1.0, 1.0],
            radii: [0.0; 4],
            opacity: 1.0,
            id: None,
        }
    }
}

impl Widget for Rect {
    fn build(&mut self, scene: &mut Scene) {
        let mut node = RectNode::new(self.x, self.y, self.w, self.h);
        node.color = self.color;
        node.radii = self.radii;
        node.opacity = self.opacity;
        self.id = Some(scene.add_rect(node));
    }

    fn update(&mut self, scene: &mut Scene) {
        let Some(id) = self.id else { return };
        let Some(Node::Rect(r)) = scene.get_mut(id) else { return };
        r.x = self.x;
        r.y = self.y;
        r.w = self.w;
        r.h = self.h;
        r.color = self.color;
        r.radii = self.radii;
        r.opacity = self.opacity;
    }
}
