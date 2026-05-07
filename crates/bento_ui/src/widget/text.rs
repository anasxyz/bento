use crate::widget::Widget;
use bento_shared::{
    TextMeasurer,
    scene::{Node, Scene, SceneNodeId, TextAlign, TextNode},
};

pub struct Text {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub size: f32,
    pub color: [f32; 4],
    pub opacity: f32,
    pub max_width: Option<f32>,
    pub align: TextAlign,
    pub font_family: String,
    pub weight: u16,
    pub italic: bool,
    pub letter_spacing: f32,

    id: Option<SceneNodeId>,
}

impl Text {
    pub fn new(text: &str, x: f32, y: f32, size: f32) -> Self {
        Self {
            x,
            y,
            text: text.to_string(),
            size,
            color: [1.0, 1.0, 1.0, 1.0],
            opacity: 1.0,
            max_width: None,
            align: TextAlign::Left,
            font_family: String::new(),
            weight: 400,
            italic: false,
            letter_spacing: 0.0,

            id: None,
        }
    }
}

impl Widget for Text {
    fn build(&mut self, scene: &mut Scene) {
        let mut node = TextNode::new(&self.text, self.x, self.y, self.size);
        node.color = self.color;
        node.opacity = self.opacity;
        node.max_width = self.max_width;
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

        t.x = self.x;
        t.y = self.y;
        t.text = self.text.clone();
        t.size = self.size;
        t.color = self.color;
        t.opacity = self.opacity;
        t.max_width = self.max_width;
        t.align = self.align.clone();
        t.font_family = self.font_family.clone();
        t.weight = self.weight;
        t.italic = self.italic;
        t.letter_spacing = self.letter_spacing;
    }
}
