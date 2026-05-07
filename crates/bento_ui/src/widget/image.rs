use bento_shared::scene::{ImageNode, Node, Scene, SceneNodeId};
use crate::widget::Widget;

pub struct Image {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub image_id: u64,
    pub opacity: f32,
    pub radii: [f32; 4],
    id: Option<SceneNodeId>,
}

impl Image {
    pub fn new(x: f32, y: f32, w: f32, h: f32, image_id: u64) -> Self {
        Self {
            x, y, w, h,
            image_id,
            opacity: 1.0,
            radii: [0.0; 4],
            id: None,
        }
    }
}

impl Widget for Image {
    fn build(&mut self, scene: &mut Scene) {
        let mut node = ImageNode::new(self.x, self.y, self.w, self.h, self.image_id);
        node.opacity = self.opacity;
        node.radii = self.radii;
        self.id = Some(scene.add_image(node));
    }

    fn update(&mut self, scene: &mut Scene) {
        let Some(id) = self.id else { return };
        let Some(Node::Image(img)) = scene.get_mut(id) else { return };
        img.x = self.x;
        img.y = self.y;
        img.w = self.w;
        img.h = self.h;
        img.image_id = self.image_id;
        img.opacity = self.opacity;
        img.radii = self.radii;
    }
}
