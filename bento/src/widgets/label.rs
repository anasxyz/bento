use crate::color::Color;
use crate::fonts::{FontAttrs, Fonts};
use crate::widget::{AsAny, Base, HasBase, Widget};
use bento_derive::Widget;
use bento_wgpu::{SceneGraph, TextId};

#[derive(Widget)]
pub struct Label {
    pub base: Base,
    text: String,
    family: String,
    size: f32,
    weight: u16,
    italic: bool,
    color: Color,
    text_id: Option<TextId>,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            base: Base::new(),
            text: text.to_string(),
            family: "sans-serif".to_string(),
            size: 14.0,
            weight: 400,
            italic: false,
            color: Color::WHITE,
            text_id: None,
        }
    }

    pub fn set_text(&mut self, s: &str) -> &mut Self {
        self.text = s.to_string();
        self
    }
    pub fn set_font_family(&mut self, s: &str) -> &mut Self {
        self.family = s.to_string();
        self
    }
    pub fn set_size(&mut self, v: f32) -> &mut Self {
        self.size = v;
        self
    }
    pub fn set_weight(&mut self, v: u16) -> &mut Self {
        self.weight = v;
        self
    }
    pub fn set_italic(&mut self, v: bool) -> &mut Self {
        self.italic = v;
        self
    }
    pub fn set_color(&mut self, c: Color) -> &mut Self {
        self.color = c;
        self
    }
}

impl Widget for Label {
    fn build(&mut self, scene: &mut SceneGraph) {
        self.text_id = Some(scene.add_text());
    }

    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, _h: f32) {
        if let Some(id) = self.text_id {
            let node = scene.text_mut(id);
            node.set_pos(x, y);
            node.set_content(&self.text);
            node.set_family(&self.family);
            node.set_size(self.size);
            node.set_weight(self.weight);
            node.set_italic(self.italic);
            node.set_color(self.color.to_array());
            node.set_width(w);
            node.set_visible(true);
        }
    }

    fn measure(&self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)> {
        let attrs = FontAttrs {
            family: self.family.clone(),
            size: self.size,
            weight: self.weight,
            italic: self.italic,
            line_height: None,
        };
        Some(fonts.measure(&self.text, &attrs, max_width))
    }

    fn has_measure(&self) -> bool {
        true
    }
}
