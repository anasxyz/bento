use std::any::Any;

use bento_shared::{RectNode, Scene, SceneNode, SceneNodeId};
use bento_shared::TextMeasurer;

use crate::widget::{AsAny, Widget};
use crate::ui::Ui;

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
