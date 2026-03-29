use crate::color::Color;
use crate::fonts::{FontAttrs, Fonts};
use crate::input::MouseButton;
use crate::widget::{AsAny, Base, HasBase, Widget};
use bento_derive::Widget;
use bento_wgpu::{RectId, SceneGraph, SceneNodeId, TextId, TransformId};

#[derive(Widget)]
pub struct Button {
    pub base: Base,

    // style
    label: String,
    color: Color,
    hover_color: Color,
    pressed_color: Color,
    disabled_color: Color,
    text_color: Color,
    disabled_text_color: Color,
    radius: f32,
    border_color: Color,
    border_widths: [f32; 4],
    font_family: String,
    font_size: f32,
    font_weight: u16,

    // state
    hovered: bool,
    pressed: bool,
    disabled: bool,

    // scene nodes
    rect_id: Option<RectId>,
    transform_id: Option<TransformId>,
    text_id: Option<TextId>,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            base: Base::new(),
            label: label.to_string(),
            color: Color::rgb(60, 60, 60),
            hover_color: Color::rgb(80, 80, 80),
            pressed_color: Color::rgb(40, 40, 40),
            disabled_color: Color::rgb(40, 40, 40),
            text_color: Color::WHITE,
            disabled_text_color: Color::rgba(255, 255, 255, 100),
            radius: 6.0,
            border_color: Color::TRANSPARENT,
            border_widths: [0.0; 4],
            font_family: "sans-serif".to_string(),
            font_size: 14.0,
            font_weight: 500,
            hovered: false,
            pressed: false,
            disabled: false,
            rect_id: None,
            transform_id: None,
            text_id: None,
        }
    }

    pub fn set_label(&mut self, s: &str) -> &mut Self {
        self.label = s.to_string();
        self.base.render_dirty = true;
        self
    }
    pub fn set_color(&mut self, c: Color) -> &mut Self {
        self.color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_hover_color(&mut self, c: Color) -> &mut Self {
        self.hover_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_pressed_color(&mut self, c: Color) -> &mut Self {
        self.pressed_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_disabled_color(&mut self, c: Color) -> &mut Self {
        self.disabled_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_text_color(&mut self, c: Color) -> &mut Self {
        self.text_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_disabled_text_color(&mut self, c: Color) -> &mut Self {
        self.disabled_text_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_radius(&mut self, r: f32) -> &mut Self {
        self.radius = r;
        self.base.render_dirty = true;
        self
    }
    pub fn set_border_color(&mut self, c: Color) -> &mut Self {
        self.border_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_border_widths(&mut self, w: [f32; 4]) -> &mut Self {
        self.border_widths = w;
        self.base.render_dirty = true;
        self
    }
    pub fn set_font_family(&mut self, s: &str) -> &mut Self {
        self.font_family = s.to_string();
        self.base.render_dirty = true;
        self
    }
    pub fn set_font_size(&mut self, v: f32) -> &mut Self {
        self.font_size = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_font_weight(&mut self, v: u16) -> &mut Self {
        self.font_weight = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_disabled(&mut self, v: bool) -> &mut Self {
        self.disabled = v;
        self.base.render_dirty = true;
        self
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn current_color(&self) -> Color {
        if self.disabled {
            self.disabled_color
        } else if self.pressed {
            self.pressed_color
        } else if self.hovered {
            self.hover_color
        } else {
            self.color
        }
    }

    fn current_text_color(&self) -> Color {
        if self.disabled {
            self.disabled_text_color
        } else {
            self.text_color
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new("")
    }
}

impl Widget for Button {
    fn build(&mut self, scene: &mut SceneGraph) {
        self.transform_id = Some(scene.add_transform());
        self.rect_id = Some(scene.add_rect());
        self.text_id = Some(scene.add_text());

        let transform = self.transform_id.unwrap();
        let rect = self.rect_id.unwrap();
        let text = self.text_id.unwrap();

        scene.add_child(SceneNodeId(transform.0), SceneNodeId(rect.0));
        scene.add_child(SceneNodeId(transform.0), SceneNodeId(text.0));
    }

    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, h: f32) {
        if let Some(id) = self.rect_id {
            let n = scene.rect_mut(id);
            n.set_rect(x, y, w, h);
            n.set_color(self.current_color().to_array());
            n.set_radius(self.radius);
            n.set_border_color(self.border_color.to_array());
            n.set_border_widths(self.border_widths);
            n.set_visible(true);
        }

        if let Some(id) = self.text_id {
            let n = scene.text_mut(id);
            let text_x = x + w / 2.0;
            let text_y = y + (h - self.font_size) / 2.0 - self.font_size * 0.15;
            n.set_pos(text_x, text_y);
            n.set_content(&self.label);
            n.set_family(&self.font_family);
            n.set_size(self.font_size);
            n.set_weight(self.font_weight);
            n.set_color(self.current_text_color().to_array());
            n.set_width(w);
            n.set_visible(true);
        }
    }

    fn children_attachment_node(&self) -> Option<SceneNodeId> {
        self.transform_id.map(|id| SceneNodeId(id.0))
    }

    fn is_interactive(&self) -> bool {
        !self.disabled
    }

    fn measure(&self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)> {
        let attrs = FontAttrs {
            family: self.font_family.clone(),
            size: self.font_size,
            weight: self.font_weight,
            italic: false,
            line_height: None,
        };
        let (tw, _) = fonts.measure(&self.label, &attrs, max_width);
        Some((tw + 24.0, self.font_size * 1.6 + 8.0))
    }

    fn has_measure(&self) -> bool {
        true
    }

    fn on_mouse_enter(&mut self) {
        if self.disabled { return; }
        self.hovered = true;
        self.base.cursor = crate::input::Cursor::Pointer;
        self.base.render_dirty = true;
    }

    fn on_mouse_leave(&mut self) {
        self.hovered = false;
        self.pressed = false;
        self.base.cursor = crate::input::Cursor::Default;
        self.base.render_dirty = true;
    }

    fn on_mouse_press(&mut self, _x: f32, _y: f32, button: MouseButton) {
        if self.disabled { return; }
        if button == MouseButton::Left {
            self.pressed = true;
            self.base.render_dirty = true;
        }
    }

    fn on_mouse_release(&mut self, _x: f32, _y: f32, _button: MouseButton) {
        if self.pressed {
            self.pressed = false;
            self.base.render_dirty = true;
        }
    }
}
