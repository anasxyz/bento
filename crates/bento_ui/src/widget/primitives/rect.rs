use bento_macros::Widget;
use bento_shared::{
    TextMeasurer,
    scene::{Node, RectNode, Scene, SceneNodeId},
};

use crate::widget::{Base, HasBase, Widget};

#[derive(Widget)]
pub struct Rect {
    pub base: Base,
    pub color: [f32; 4],
    pub radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub opacity: f32,
    id: Option<SceneNodeId>,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            color: [1.0, 1.0, 1.0, 1.0],
            radii: [0.0; 4],
            opacity: 1.0,
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            id: None,
        }
    }
}

impl Widget for Rect {
    fn build(&mut self, scene: &mut Scene) {
        let l = &self.base.layout;
        let mut node = RectNode::new(l.x, l.y, l.w, l.h);
        node.color = self.color;
        node.radii = self.radii;
        node.opacity = self.opacity;
        node.border_color = self.border_color;
        node.border_widths = self.border_widths;
        self.id = Some(scene.add_rect(node));
    }

    fn update(&mut self, scene: &mut Scene, _measurer: &mut dyn TextMeasurer) {
        let Some(id) = self.id else { return };
        let Some(Node::Rect(r)) = scene.get_mut(id) else {
            return;
        };
        r.color = self.color;
        r.radii = self.radii;
        r.opacity = self.opacity;
        r.border_color = self.border_color;
        r.border_widths = self.border_widths;
    }
}
