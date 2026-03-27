use crate::color::Color;
use crate::fonts::Fonts;
use crate::widget::{AsAny, Base, HasBase, Widget};
use bento_derive::Widget;
use bento_wgpu::{RectId, SceneGraph};

#[derive(Widget)]
pub struct Rect {
    pub base: Base,
    color: Color,
    radius: f32,
    border_color: Color,
    border_widths: [f32; 4],
    rect_id: Option<RectId>,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            color: Color::TRANSPARENT,
            radius: 0.0,
            border_color: Color::TRANSPARENT,
            border_widths: [0.0; 4],
            rect_id: None,
        }
    }

    pub fn set_color(&mut self, c: Color) -> &mut Self {
        self.color = c;
        self
    }
    pub fn set_radius(&mut self, r: f32) -> &mut Self {
        self.radius = r;
        self
    }
    pub fn set_border_color(&mut self, c: Color) -> &mut Self {
        self.border_color = c;
        self
    }
    pub fn set_border_widths(&mut self, w: [f32; 4]) -> &mut Self {
        self.border_widths = w;
        self
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Rect {
    fn build(&mut self, scene: &mut SceneGraph) {
        self.rect_id = Some(scene.add_rect());
    }

    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, h: f32) {
        if let Some(id) = self.rect_id {
            let node = scene.rect_mut(id);
            node.set_rect(x, y, w, h);
            node.set_color(self.color.to_array());
            node.set_radius(self.radius);
            node.set_border_color(self.border_color.to_array());
            node.set_border_widths(self.border_widths);
            node.set_visible(true);
        }
    }
}
