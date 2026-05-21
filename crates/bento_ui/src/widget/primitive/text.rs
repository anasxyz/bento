use bento_shared::{SceneNode, SceneNodeId, TextMeasureRequest, TextMeasurer, TextNode};
use crate::{Ui, widget::Widget};

pub struct Text {
    id: usize,
    node: Option<SceneNodeId>,

    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub content: String,
    pub size: f32,
    pub color: [f32; 4],

    dirty: bool,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self {
            id: 0,
            node: None,

            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            content: content.to_string(),
            size: 14.0,
            color: [1.0, 1.0, 1.0, 1.0],

            dirty: true,
        }
    }

    pub fn set_x(&mut self, x: f32) {
        if self.x == x { return; }
        self.x = x;
        self.dirty = true;
    }
    pub fn set_y(&mut self, y: f32) {
        if self.y == y { return; }
        self.y = y;
        self.dirty = true;
    }
    pub fn set_content(&mut self, content: &str) {
        if self.content == content { return; }
        self.content = content.to_string();
        self.dirty = true;
    }
    pub fn set_size(&mut self, size: f32) {
        if self.size == size { return; }
        self.size = size;
        self.dirty = true;
    }
    pub fn set_color(&mut self, color: [f32; 4]) {
        if self.color == color { return; }
        self.color = color;
        self.dirty = true;
    }
}

impl Widget for Text {
    fn id(&self) -> usize { self.id }
    fn set_id(&mut self, id: usize) { self.id = id; }
    fn name(&self) -> &str { "Text" }

    fn build(&mut self, ui: &mut Ui) {
        // measure to get initial w/h
        let result = ui.measurer.measure(TextMeasureRequest {
            text: &self.content,
            font_family: "",
            size: self.size,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            max_width: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        self.w = result.width;
        self.h = result.height;

        let mut node = TextNode::new(&self.content, self.x, self.y, self.size);
        node.color = self.color;
        node.w = self.w;
        node.h = self.h;
        let node_id = ui.scene_mut().add_text(node);
        self.node = Some(node_id);
    }

    fn update(&mut self, ui: &mut Ui) {
        let result = ui.measurer.measure(TextMeasureRequest {
            text: &self.content,
            font_family: "",
            size: self.size,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            max_width: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        self.w = result.width;
        self.h = result.height;

        if let Some(node_id) = self.node {
            if let Some(SceneNode::Text(t)) = ui.scene_mut().get_mut(node_id) {
                t.text = self.content.clone();
                t.x = self.x;
                t.y = self.y;
                t.w = self.w;
                t.h = self.h;
                t.size = self.size;
                t.color = self.color;
            }
        }
    }

    fn remove(&mut self, ui: &mut Ui) {
        if let Some(node_id) = self.node {
            ui.scene_mut().remove(node_id);
        }
    }

    fn hitbox(&self) -> (f32, f32, f32, f32) { (self.x, self.y, self.w, self.h) }
    fn is_dirty(&self) -> bool { self.dirty }
    fn set_dirty(&mut self, dirty: bool) { self.dirty = dirty; }
}
