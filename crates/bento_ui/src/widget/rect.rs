use bento_shared::{RectNode, SceneNodeId};
use crate::Ui;

pub struct Rect {
    pub id: SceneNodeId,
}

impl Rect {
    pub fn new(ui: &mut Ui, x: f32, y: f32, w: f32, h: f32) -> Self {
        let id = ui.scene_mut().add_rect(RectNode::new(x, y, w, h));
        Self { id }
    }

    pub fn set_color(&self, ui: &mut Ui, color: [f32; 4]) {
        if let Some(bento_shared::SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.id) {
            r.color = color;
            ui.needs_redraw = true;
        }
    }

    pub fn set_pos(&self, ui: &mut Ui, x: f32, y: f32) {
        if let Some(bento_shared::SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.id) {
            r.x = x;
            r.y = y;
            ui.needs_redraw = true;
        }
    }

    pub fn set_size(&self, ui: &mut Ui, w: f32, h: f32) {
        if let Some(bento_shared::SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.id) {
            r.w = w;
            r.h = h;
            ui.needs_redraw = true;
        }
    }
}
