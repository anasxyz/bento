use std::any::Any;

use bento_shared::{
    RectNode, Scene, SceneNode, SceneNodeId, TextMeasureRequest, TextMeasurer, TextNode,
};

use crate::{AsAny, Widget};

pub struct Button {
    pub dirty: bool,

    x: f32,
    y: f32,
    w: f32,
    h: f32,
    label: String,
    color: [f32; 4],
    text_color: [f32; 4],

    focusable: bool,
    focused: bool,
    hoverable: bool,
    hovered: bool,

    rect_id: Option<SceneNodeId>,
    text_id: Option<SceneNodeId>,
}

impl Button {
    pub fn new(label: &str, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            dirty: true,
            x,
            y,
            w,
            h,
            label: label.to_string(),
            color: [0.2, 0.2, 0.2, 1.0],
            text_color: [1.0, 1.0, 1.0, 1.0],
            focusable: true,
            focused: false,
            hoverable: true,
            hovered: false,
            rect_id: None,
            text_id: None,
        }
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
        self.dirty = true;
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
        self.dirty = true;
    }

    pub fn set_text_color(&mut self, color: [f32; 4]) {
        self.text_color = color;
        self.dirty = true;
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn w(&self) -> f32 {
        self.w
    }
    pub fn h(&self) -> f32 {
        self.h
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
        self.dirty = true;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
        self.dirty = true;
    }
    pub fn set_w(&mut self, w: f32) {
        self.w = w;
        self.dirty = true;
    }
    pub fn set_h(&mut self, h: f32) {
        self.h = h;
        self.dirty = true;
    }
}

impl Widget for Button {
    fn name(&self) -> &str {
        "Button"
    }

    fn build(&mut self, scene: &mut Scene) {
        self.rect_id = Some(scene.add_rect(RectNode::new(self.x, self.y, self.w, self.h)));
        self.text_id = Some(scene.add_text(TextNode::new(&self.label, self.x, self.y, 16.0)));
    }

    fn update(&mut self, scene: &mut Scene, measurer: &mut dyn TextMeasurer) {
        let result = measurer.measure(TextMeasureRequest {
            text: &self.label,
            font_family: "",
            size: 16.0,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            max_width: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });

        let text_x = self.x + (self.w - result.width) / 2.0;
        let text_y = self.y + (self.h - result.height) / 2.0;

        if let Some(id) = self.rect_id {
            if let Some(SceneNode::Rect(r)) = scene.get_mut(id) {
                r.x = self.x;
                r.y = self.y;
                r.w = self.w;
                r.h = self.h;
                r.color = self.color;
            }
        }

        if let Some(id) = self.text_id {
            if let Some(SceneNode::Text(t)) = scene.get_mut(id) {
                t.text = self.label.clone();
                t.x = text_x;
                t.y = text_y;
                t.color = self.text_color;
            }
        }
    }

    fn remove(&mut self, scene: &mut Scene) {
        if let Some(id) = self.rect_id {
            scene.remove(id);
        }
        if let Some(id) = self.text_id {
            scene.remove(id);
        }
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

impl AsAny for Button {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
