use bento_macros::Widget;
use bento_shared::{
    TextMeasureRequest, TextMeasurer,
    scene::{Node, Scene, SceneNodeId, TextAlign, TextNode},
};

use crate::layout::Size;
use crate::widget::{Base, HasBase, Widget};

#[derive(Widget)]
pub struct Text {
    pub base: Base,
    pub text: String,
    pub size: f32,
    pub color: [f32; 4],
    pub opacity: f32,
    pub align: TextAlign,
    pub font_family: String,
    pub weight: u16,
    pub italic: bool,
    pub letter_spacing: f32,
    id: Option<SceneNodeId>,
}

impl Text {
    pub fn new(text: &str, size: f32) -> Self {
        Self {
            base: Base::new(),
            text: text.to_string(),
            size,
            color: [1.0, 1.0, 1.0, 1.0],
            opacity: 1.0,
            align: TextAlign::Left,
            font_family: String::new(),
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            id: None,
        }
    }

    fn max_width_px(&self) -> Option<f32> {
        match &self.base.layout.max_width {
            Size::Px(v) => Some(*v),
            _ => None,
        }
    }
}

impl Widget for Text {
    fn build(&mut self, scene: &mut Scene) {
        let l = &self.base.layout;
        let mut node = TextNode::new(&self.text, l.x, l.y, self.size);
        node.color = self.color;
        node.opacity = self.opacity;
        node.max_width = self.max_width_px();
        node.align = self.align.clone();
        node.font_family = self.font_family.clone();
        node.weight = self.weight;
        node.italic = self.italic;
        node.letter_spacing = self.letter_spacing;
        self.id = Some(scene.add_text(node));
    }

    fn update(&mut self, scene: &mut Scene, _measurer: &mut dyn TextMeasurer) {
        let Some(id) = self.id else { return };
        let Some(Node::Text(t)) = scene.get_mut(id) else {
            return;
        };
        let l = &self.base.layout;
        t.x = l.x;
        t.y = l.y;
        t.text = self.text.clone();
        t.size = self.size;
        t.color = self.color;
        t.opacity = self.opacity;
        t.max_width = self.max_width_px();
        t.align = self.align.clone();
        t.font_family = self.font_family.clone();
        t.weight = self.weight;
        t.italic = self.italic;
        t.letter_spacing = self.letter_spacing;
    }

    fn measure(
        &self,
        _known_w: Option<f32>,
        _known_h: Option<f32>,
        measurer: &mut dyn TextMeasurer,
    ) -> (f32, f32) {
        let result = measurer.measure(TextMeasureRequest {
            text: &self.text,
            size: self.size,
            max_width: self.max_width_px(),
            font_family: &self.font_family,
            weight: self.weight,
            italic: self.italic,
            letter_spacing: self.letter_spacing,
            line_height: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        (result.width, result.height)
    }
}
