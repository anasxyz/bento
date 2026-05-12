use std::any::Any;

use bento_shared::{RectNode, Scene, SceneNode, SceneNodeId};
use bento_shared::TextMeasurer;

use crate::{AsAny, Widget};

#[derive(Debug)]
pub struct Rect {
    pub dirty: bool,

    x: f32,
    y: f32,
    w: f32,
    h: f32,

    focusable: bool,
    focused: bool,
    hoverable: bool,
    hovered: bool,

    rect_id: Option<SceneNodeId>,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            dirty: true,

            x,
            y,
            w,
            h,

            focusable: true,
            focused: false,
            hoverable: true,
            hovered: false,

            rect_id: None,
        }
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

    pub fn set_focusable(&mut self, focusable: bool) {
        self.focusable = focusable;
    }

    pub fn set_hoverable(&mut self, hoverable: bool) {
        self.hoverable = hoverable;
    }
}

impl Widget for Rect {
    fn name(&self) -> &str {
        "Rect"
    }

    fn build(&mut self, scene: &mut Scene) {
        let mut node = RectNode::new(self.x, self.y, self.w, self.h);
        self.rect_id = Some(scene.add_rect(node));
    }

    fn update(&mut self, scene: &mut Scene, _measurer: &mut dyn TextMeasurer) {
        println!("rect update x={} y={}", self.x, self.y);
        // Return if no SceneNodeId is set
        // If that's the case, build() was not called or something went wrong
        let Some(id) = self.rect_id else { return };

        // Look up SceneNode in Scene by id
        // Pattern match to get the inner RectNode as mutable
        // Return if SceneNode is not RectNode or missing
        let Some(SceneNode::Rect(r)) = scene.get_mut(id) else {
            return;
        };

        // Write widget's values to RectNode
        r.x = self.x;
        r.y = self.y;
        r.w = self.w;
        r.h = self.h;
    }

    fn remove(&mut self, scene: &mut Scene) {
        let Some(id) = self.rect_id else { return };

        scene.remove(id);
    }

    // TODO: add to proc macro
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    // TODO: add to proc macro
    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    // TODO: add to proc macro
    fn focusable(&self) -> bool {
        self.focusable
    }
    // TODO: add to proc macro
    fn is_focused(&self) -> bool {
        self.focused
    }
    // TODO: add to proc macro
    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    // TODO: add to proc macro
    fn hoverable(&self) -> bool {
        self.hoverable
    }
    // TODO: add to proc macro
    fn is_hovered(&self) -> bool {
        self.hovered
    }
    // TODO: add to proc macro
    fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }
}

// TODO: add to proc macro
impl AsAny for Rect {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
