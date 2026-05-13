use std::any::Any;

use bento_shared::{RectNode, Scene, SceneNode, SceneNodeId};
use bento_shared::TextMeasurer;

use crate::{AsAny, Ui, Widget};

#[derive(Debug)]
pub struct Rect {
    pub dirty: bool,

    x: f32,
    y: f32,
    w: f32,
    h: f32,

    color: [f32; 4],
    radii: [f32; 4],
    border_color: [f32; 4],
    border_widths: [f32; 4],

    rotate: f32,
    scale_x: f32,
    scale_y: f32,
    z: i32,
    opacity: f32,
    clip: Option<[f32; 4]>,

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

            color: [1.0, 1.0, 1.0, 1.0],
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],

            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            z: 1,
            opacity: 1.0,
            clip: None,

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

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
        self.dirty = true;
    }

    pub fn set_radii(&mut self, radii: [f32; 4]) {
        self.radii = radii;
        self.dirty = true;
    }

    pub fn set_border_widths(&mut self, widths: [f32; 4]) {
        self.border_widths = widths;
        self.dirty = true;
    }

    pub fn set_border_color(&mut self, color: [f32; 4]) {
        self.border_color = color;
        self.dirty = true;
    }

    pub fn set_rotate(&mut self, angle: f32) {
        self.rotate = angle;
        self.dirty = true;
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.scale_x = x;
        self.scale_y = y;
        self.dirty = true;
    }

    pub fn set_z(&mut self, z: i32) {
        self.z = z;
        self.dirty = true;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
        self.dirty = true;
    }

    pub fn set_clip(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.clip = Some([x, y, w, h]);
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

    fn build(&mut self, ui: &mut Ui) {
        let mut node = RectNode::new(self.x, self.y, self.w, self.h);
        node.color = self.color;
        node.radii = self.radii;
        node.border_color = self.border_color;
        node.border_widths = self.border_widths;
        node.rotate = self.rotate;
        node.scale_x = self.scale_x;
        node.scale_y = self.scale_y;
        node.z = self.z;
        node.opacity = self.opacity;
        node.clip = self.clip;

        self.rect_id = Some(ui.scene_mut().add_rect(node));
    }

    fn update(&mut self, ui: &mut Ui, _measurer: &mut dyn TextMeasurer) {
        // Return if no SceneNodeId is set
        // If that's the case, build() was not called or something went wrong
        let Some(id) = self.rect_id else { return };

        // Look up SceneNode in Scene by id
        // Pattern match to get the inner RectNode as mutable
        // Return if SceneNode is not RectNode or missing
        let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(id) else {
            return;
        };

        // Write widget's values to RectNode
        r.x = self.x;
        r.y = self.y;
        r.w = self.w;
        r.h = self.h;
        r.color = self.color;
        r.radii = self.radii;
        r.border_color = self.border_color;
        r.border_widths = self.border_widths;
        r.rotate = self.rotate;
        r.scale_x = self.scale_x;
        r.scale_y = self.scale_y;
        r.z = self.z;
        r.opacity = self.opacity;
        r.clip = self.clip;
    }

    fn remove(&mut self, ui: &mut Ui) {
        let Some(id) = self.rect_id else { return };

        ui.scene_mut().remove(id);
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
