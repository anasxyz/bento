use crate::{
    Ui,
    widget::{Widget, WidgetHandle},
};
use bento_shared::{SceneNodeId, TextNode};

pub struct Text {
    id: Option<SceneNodeId>,
    dirty: bool,
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub size: f32,
    pub color: [f32; 4],
    pub opacity: f32,
    pub z: i32,
    pub weight: u16,
    pub italic: bool,
}

impl Text {
    pub fn new(text: &str, x: f32, y: f32, size: f32) -> Self {
        Self {
            id: None,
            dirty: false,
            x,
            y,
            text: text.to_string(),
            size,
            color: [1.0, 1.0, 1.0, 1.0],
            opacity: 1.0,
            z: 1,
            weight: 400,
            italic: false,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.dirty = true;
    }
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.dirty = true;
    }
    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
        self.dirty = true;
    }
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
        self.dirty = true;
    }
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
        self.dirty = true;
    }
    pub fn set_weight(&mut self, weight: u16) {
        self.weight = weight;
        self.dirty = true;
    }
    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
        self.dirty = true;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
        self.dirty = true;
    }
}

impl Widget for Text {
    fn root(&self) -> Option<SceneNodeId> {
        self.id
    }

    fn name(&self) -> &str {
        "Text"
    }

    fn build(&mut self, ui: &mut Ui, _handle: WidgetHandle<()>) {
        let mut node = TextNode::new(&self.text, self.x, self.y, self.size);
        node.color = self.color;
        node.opacity = self.opacity;
        node.z = self.z;
        node.weight = self.weight;
        node.italic = self.italic;
        self.id = Some(ui.scene_mut().add_text(node));
    }

    fn update(&mut self, ui: &mut Ui) {
        if let Some(bento_shared::SceneNode::Text(t)) = ui.scene_mut().get_mut(self.id.unwrap()) {
            t.text = self.text.clone();
            t.x = self.x;
            t.y = self.y;
            t.size = self.size;
            t.color = self.color;
            t.opacity = self.opacity;
            t.z = self.z;
            t.weight = self.weight;
            t.italic = self.italic;
            ui.needs_redraw = true;
        }
        self.dirty = false;
    }

    fn remove(&mut self, ui: &mut Ui) {
        if let Some(id) = self.id {
            ui.scene_mut().remove(id);
        }
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}
