use std::any::Any;

use bento_shared::{Scene, SceneNode, SceneNodeId, TextNode};
use bento_shared::{TextMeasureRequest, TextMeasurer};

use crate::{AsAny, Widget};

pub struct Text {
    pub dirty: bool,

    text: String,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    size: f32,
    color: [f32; 4],

    focusable: bool,
    focused: bool,
    hoverable: bool,
    hovered: bool,

    text_id: Option<SceneNodeId>,
}

impl Text {
    pub fn new(text: &str, x: f32, y: f32, size: f32) -> Self {
        Self {
            dirty: true,
            text: text.to_string(),
            x,
            y,
            w: 0.0,
            h: 0.0,
            size,
            color: [1.0, 1.0, 1.0, 1.0],
            focusable: true,
            focused: false,
            hoverable: true,
            hovered: false,
            text_id: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.dirty = true;
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
        self.dirty = true;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
        self.dirty = true;
    }

    pub fn set_size(&mut self, size: f32) {
        self.size = size;
        self.dirty = true;
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
        self.dirty = true;
    }

    pub fn set_focusable(&mut self, focusable: bool) {
        self.focusable = focusable;
    }

    pub fn set_hoverable(&mut self, hoverable: bool) {
        self.hoverable = hoverable;
    }
}

impl Widget for Text {
    fn name(&self) -> &str {
        "Text"
    }

    fn build(&mut self, scene: &mut Scene) {
        let mut node = TextNode::new(&self.text, self.x, self.y, self.size);
        node.color = self.color;
        self.text_id = Some(scene.add_text(node));
    }

    fn update(&mut self, scene: &mut Scene, measurer: &mut dyn TextMeasurer) {
        let result = measurer.measure(TextMeasureRequest {
            text: &self.text,
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

        let Some(id) = self.text_id else { return };
        let Some(SceneNode::Text(t)) = scene.get_mut(id) else {
            return;
        };

        t.text = self.text.clone();
        t.x = self.x;
        t.y = self.y;
        t.size = self.size;
        t.color = self.color;
    }

    fn remove(&mut self, scene: &mut Scene) {
        let Some(id) = self.text_id else { return };
        scene.remove(id);
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn focusable(&self) -> bool {
        self.focusable
    }
    fn is_focused(&self) -> bool {
        self.focused
    }
    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn hoverable(&self) -> bool {
        self.hoverable
    }
    fn is_hovered(&self) -> bool {
        self.hovered
    }
    fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }
}

impl AsAny for Text {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
