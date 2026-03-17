use crate::color::Color;
use crate::element::base::Base;
use crate::element::element::Element;
use crate::element::layout::Layout;
use crate::element::values::FlexDirection;
use crate::input::{Key, Modifiers, MouseButton};
use crate::render::DrawCall;
use bento_derive::Element;

#[derive(Element)]
pub struct Container {
    base: Base,
    bg_color: Option<Color>,
    border_radius: Option<f32>,
    border_widths: [f32; 4],
    border_color: Option<Color>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            bg_color: None,
            border_radius: None,
            border_widths: [0.0; 4],
            border_color: None,
        }
    }

    pub fn bg_color(&self) -> Option<Color> {
        self.bg_color
    }
    pub fn border_radius(&self) -> Option<f32> {
        self.border_radius
    }
    pub fn border_widths(&self) -> [f32; 4] {
        self.border_widths
    }
    pub fn border_color(&self) -> Option<Color> {
        self.border_color
    }

    pub fn set_bg_color(&mut self, color: Option<Color>) -> &mut Self {
        self.bg_color = color;
        self.base.dirty = true;
        self
    }
    pub fn set_border_radius(&mut self, radius: Option<f32>) -> &mut Self {
        self.border_radius = radius;
        self.base.dirty = true;
        self
    }
    pub fn set_border_color(&mut self, color: Option<Color>) -> &mut Self {
        self.border_color = color;
        self.base.dirty = true;
        self
    }
    pub fn set_border(&mut self, widths: [f32; 4]) -> &mut Self {
        self.border_widths = widths;
        self.base.dirty = true;
        self
    }
}

impl Element for Container {
    fn draw_calls(&self, clip: Option<[f32; 4]>, z: i32, opacity: f32) -> Vec<DrawCall> {
        let Some(bg) = self.bg_color else {
            return vec![];
        };
        let l = &self.base.layout;
        let mut color = bg.to_array();
        color[3] *= opacity;
        let mut border_color = self.border_color.unwrap_or(Color::BLACK).to_array();
        border_color[3] *= opacity;
        vec![DrawCall::Rect {
            x: l.x,
            y: l.y,
            w: l.w,
            h: l.h,
            color,
            radius: self.border_radius.unwrap_or(0.0),
            border_color,
            border_widths: self.border_widths,
            clip,
            z_index: z,
        }]
    }
}

pub struct Row;
impl Row {
    pub fn new() -> Container {
        let mut c = Container::new();
        c.base.layout.flex_direction = FlexDirection::Row;
        c
    }
}

pub struct Column;
impl Column {
    pub fn new() -> Container {
        let mut c = Container::new();
        c.base.layout.flex_direction = FlexDirection::Col;
        c
    }
}
