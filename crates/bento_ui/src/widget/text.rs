use crate::Ui;
use bento_shared::{SceneNodeId, TextNode};

pub struct Text {
    pub id: SceneNodeId,
}

impl Text {
    pub fn new(ui: &mut Ui, text: &str, x: f32, y: f32, size: f32) -> Self {
        let id = ui.scene_mut().add_text(TextNode::new(text, x, y, size));
        Self { id }
    }

    pub fn set_text(&self, ui: &mut Ui, text: &str) {
        if let Some(bento_shared::SceneNode::Text(t)) = ui.scene_mut().get_mut(self.id) {
            t.text = text.to_string();
            ui.needs_redraw = true;
        }
    }

    pub fn set_pos(&self, ui: &mut Ui, x: f32, y: f32) {
        if let Some(bento_shared::SceneNode::Text(t)) = ui.scene_mut().get_mut(self.id) {
            t.x = x;
            t.y = y;
            ui.needs_redraw = true;
        }
    }

    pub fn set_color(&self, ui: &mut Ui, color: [f32; 4]) {
        if let Some(bento_shared::SceneNode::Text(t)) = ui.scene_mut().get_mut(self.id) {
            t.color = color;
            ui.needs_redraw = true;
        }
    }

    pub fn set_size(&self, ui: &mut Ui, size: f32) {
        if let Some(bento_shared::SceneNode::Text(t)) = ui.scene_mut().get_mut(self.id) {
            t.size = size;
            ui.needs_redraw = true;
        }
    }
}
