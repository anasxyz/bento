use crate::color::Color;
use crate::widget::{AsAny, Base, HasBase, Widget};
use bento_derive::Widget;
use bento_wgpu::{ClipId, RectId, SceneGraph, SceneNodeId, TransformId};

#[derive(Widget)]
pub struct ScrollContainer {
    pub base: Base,
    pub scroll_y: f32,
    pub scroll_x: f32,
    color: Color,
    height: f32,
    rect_id: Option<RectId>,
    clip_id: Option<ClipId>,
    transform_id: Option<TransformId>,
}

impl ScrollContainer {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            scroll_y: 0.0,
            scroll_x: 0.0,
            color: Color::TRANSPARENT,
            height: 0.0,
            rect_id: None,
            clip_id: None,
            transform_id: None,
        }
    }

    pub fn set_color(&mut self, c: Color) -> &mut Self {
        self.color = c;
        self
    }
}

impl Default for ScrollContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ScrollContainer {
    fn build(&mut self, scene: &mut SceneGraph) {
        self.rect_id = Some(scene.add_rect());
        self.clip_id = Some(scene.add_clip());
        self.transform_id = Some(scene.add_transform());
        let clip = self.clip_id.unwrap();
        let transform = self.transform_id.unwrap();
        scene.add_child(SceneNodeId(clip.0), SceneNodeId(transform.0));
    }

    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, h: f32) {
        self.height = h;

        let max_scroll = (self.base.content_height - h).max(0.0);
        self.scroll_y = self.scroll_y.clamp(0.0, max_scroll);

        if let Some(id) = self.rect_id {
            let node = scene.rect_mut(id);
            node.set_rect(x, y, w, h);
            node.set_color(self.color.to_array());
            node.set_visible(true);
        }
        if let Some(id) = self.clip_id {
            scene.clip_mut(id).set_rect(x, y, w, h);
        }
        if let Some(id) = self.transform_id {
            scene
                .transform_mut(id)
                .set_offset(-self.scroll_x, -self.scroll_y);
        }
    }

    fn children_attachment_node(&self) -> Option<SceneNodeId> {
        self.transform_id.map(|id| SceneNodeId(id.0))
    }

    fn on_mouse_scroll(&mut self, _dx: f32, dy: f32) {
        let max_scroll = (self.base.content_height - self.height).max(0.0);
        if max_scroll <= 0.0 {
            return;
        }
        self.scroll_y = (self.scroll_y + dy * 20.0).clamp(0.0, max_scroll);
    }
}
