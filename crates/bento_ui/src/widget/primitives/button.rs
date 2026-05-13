use std::any::Any;

use bento_shared::{
    RectNode, Scene, SceneNode, SceneNodeId, TextAlign, TextMeasureRequest, TextMeasurer, TextNode,
};

use crate::{AsAny, Click, HoverEnter, HoverLeave, Ui, Widget, WidgetHandle};

pub struct Button {
    handle: WidgetHandle<Button>,

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

    group_id: Option<SceneNodeId>,
    rect_id: Option<SceneNodeId>,
    text_id: Option<SceneNodeId>,
}

impl Button {
    pub fn new(label: &str, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            handle: WidgetHandle::default(),
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
            group_id: None,
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

    fn set_handle(&mut self, id: u32, generation: u32) {
        self.handle = WidgetHandle::new(id, generation);
    }

    fn build(&mut self, ui: &mut Ui) {
        let scene = ui.scene_mut();
        self.group_id = Some(scene.add_group(|g, s| {
            let mut rect_node = RectNode::new(self.x, self.y, self.w, self.h);
            rect_node.radius(7.0);
            let mut text_node = TextNode::new(&self.label, self.x, self.y, 16.0);

            self.rect_id = Some(s.add_rect(rect_node));
            self.text_id = Some(s.add_text(text_node));
        }));

        let handle = self.handle;

        ui.listen(handle, move |e: &HoverEnter, ui| {
            if let Some(b) = ui.get_mut(handle) {
                b.set_color([0.2, 0.2, 0.7, 1.0]);
            }
        });

        ui.listen(handle, move |e: &HoverLeave, ui| {
            if let Some(b) = ui.get_mut(handle) {
                b.set_color([0.2, 0.2, 0.2, 1.0]);
            }
        });
    }

    fn scene_root(&self) -> Option<SceneNodeId> {
        self.group_id
    }

    fn update(&mut self, ui: &mut Ui, measurer: &mut dyn TextMeasurer) {
        let padding = 10.0;
        let text_max_width = self.w - padding * 2.0;

        let result = measurer.measure(TextMeasureRequest {
            text: &self.label,
            font_family: "",
            size: 16.0,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            max_width: Some(text_max_width),
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });

        let actual_h = result.height + padding * 2.0;
        self.set_h(self.h.max(actual_h));
        let text_x = self.x + (self.w - result.width) / 2.0;
        let text_y = self.y + (self.h - result.height) / 2.0;

        if let Some(id) = self.rect_id {
            if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(id) {
                r.x = self.x;
                r.y = self.y;
                r.w = self.w;
                r.h = self.h;
                r.color = self.color;
            }
        }

        if let Some(id) = self.text_id {
            if let Some(SceneNode::Text(t)) = ui.scene_mut().get_mut(id) {
                t.text = self.label.clone();
                t.x = text_x;
                t.y = text_y;
                t.w = result.width;
                t.h = result.height;
                t.color = self.text_color;
                t.max_width = Some(text_max_width);
            }
        }
    }

    fn remove(&mut self, ui: &mut Ui) {
        if let Some(id) = self.group_id {
            // removes group and all children
            ui.scene_mut().remove(id);
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
