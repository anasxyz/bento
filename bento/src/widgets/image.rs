use crate::color::Color;
use crate::widget::{AsAny, Base, HasBase, Widget};
use bento_derive::Widget;
use bento_wgpu::{ImageId, ImageKey, SceneGraph, SceneNodeId};

#[derive(Widget)]
pub struct Image {
    pub base: Base,

    image_key: ImageKey,
    radius:    f32,
    tint:      Color,

    image_id: Option<ImageId>,
}

impl Image {
    pub fn new(key: ImageKey) -> Self {
        Self {
            base:     Base::new(),
            image_key: key,
            radius:   0.0,
            tint:     Color::WHITE,
            image_id: None,
        }
    }

    pub fn set_image_key(&mut self, key: ImageKey) -> &mut Self {
        self.image_key = key;
        self.base.render_dirty = true;
        self
    }
    pub fn set_radius(&mut self, r: f32) -> &mut Self {
        self.radius = r;
        self.base.render_dirty = true;
        self
    }
    pub fn set_tint(&mut self, c: Color) -> &mut Self {
        self.tint = c;
        self.base.render_dirty = true;
        self
    }
}

impl Widget for Image {
    fn build(&mut self, scene: &mut SceneGraph) {
        self.image_id = Some(scene.add_image());
    }

    fn sync(&mut self, scene: &mut SceneGraph) {
        let layer = self.base.layer();
        let x = self.base.x();
        let y = self.base.y();
        let w = self.base.w();
        let h = self.base.h();

        if let Some(id) = self.image_id {
            let n = scene.image_mut(id);
            n.set_rect(x, y, w, h);
            n.set_image_key(self.image_key);
            n.set_radius(self.radius);
            n.set_tint(self.tint.to_array());
            n.set_z(layer as i32);
            n.set_visible(self.base.visible);
        }
    }
}
