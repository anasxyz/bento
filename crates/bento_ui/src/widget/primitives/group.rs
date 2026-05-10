use crate::layout::Overflow;
use crate::widget::Widget;
use crate::widget::{Base, HasBase};
use bento_macros::Widget;
use bento_shared::{
    TextMeasurer,
    scene::{GroupNode, Node, Scene, SceneNodeId},
};

#[derive(Widget)]
pub struct Group {
    pub base: Base,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub opacity: Option<f32>,
    pub clip: Option<[f32; 4]>,
    pub(crate) id: Option<SceneNodeId>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            scroll_x: 0.0,
            scroll_y: 0.0,
            opacity: None,
            clip: None,
            id: None,
        }
    }
}

impl Widget for Group {
    fn build(&mut self, scene: &mut Scene) {
        let mut node = GroupNode::new();
        node.opacity = self.opacity;
        node.clip = self.clip;
        let id = scene.add_group(node);
        self.id = Some(id);
    }

    fn update(&mut self, scene: &mut Scene, _measurer: &mut dyn TextMeasurer) {
        let Some(id) = self.id else { return };
        let Some(Node::Group(g)) = scene.get_mut(id) else {
            return;
        };
        let l = &self.base.layout;

        g.offset_x = -self.scroll_x;
        g.offset_y = -self.scroll_y;
        g.opacity = self.opacity;
        g.clip = match (l.overflow_x, l.overflow_y) {
            (Overflow::Visible, Overflow::Visible) => None,
            _ => Some([l.x, l.y, l.w, l.h]),
        };
    }
}
