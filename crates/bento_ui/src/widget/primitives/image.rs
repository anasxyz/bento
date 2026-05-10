use crate::widget::Widget;
use crate::widget::{Base, HasBase};
use bento_macros::Widget;
use bento_shared::{
    TextMeasurer,
    scene::{ImageNode, Node, Scene, SceneNodeId},
};

#[derive(Widget)]
pub struct Image {
    pub base: Base,
    pub image_id: u64,
    pub opacity: f32,
    pub radii: [f32; 4],
    id: Option<SceneNodeId>,
}

impl Image {
    pub fn new(image_id: u64) -> Self {
        Self {
            base: Base::new(),
            image_id,
            opacity: 1.0,
            radii: [0.0; 4],
            id: None,
        }
    }
}

impl Widget for Image {
    fn build(&mut self, scene: &mut Scene) {
        let l = &self.base.layout;
        let mut node = ImageNode::new(l.x, l.y, l.w, l.h, self.image_id);
        node.opacity = self.opacity;
        node.radii = self.radii;
        self.id = Some(scene.add_image(node));
    }

    fn update(&mut self, scene: &mut Scene, _measurer: &mut dyn TextMeasurer) {
        let Some(id) = self.id else { return };
        let Some(Node::Image(img)) = scene.get_mut(id) else {
            return;
        };
        img.image_id = self.image_id;
        img.opacity = self.opacity;
        img.radii = self.radii;
    }
}
