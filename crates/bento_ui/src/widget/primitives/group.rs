use bento_macros::Widget;
use bento_shared::{
    TextMeasurer,
    scene::{GroupNode, Node, RectNode, Scene, SceneNodeId},
};

use crate::layout::Overflow;
use crate::widget::{Base, HasBase, Widget};

#[derive(Widget)]
pub struct Group {
    pub base: Base,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub opacity: Option<f32>,
    pub color: Option<[f32; 4]>,
    pub radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub(crate) id: Option<SceneNodeId>,
    bg_id: Option<SceneNodeId>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            scroll_x: 0.0,
            scroll_y: 0.0,
            opacity: None,
            color: None,
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            id: None,
            bg_id: None,
        }
    }
}

impl Widget for Group {
    fn build(&mut self, scene: &mut Scene) {
        let mut node = GroupNode::new();
        node.opacity = self.opacity;
        let id = scene.add_group(node);
        self.id = Some(id);

        let mut bg = RectNode::new(0.0, 0.0, 0.0, 0.0);
        bg.color = self.color.unwrap_or([0.0, 0.0, 0.0, 0.0]);
        bg.radii = self.radii;
        bg.border_color = self.border_color;
        bg.border_widths = self.border_widths;
        let bg_id = scene.add_rect(bg);
        scene.reparent(bg_id, id); // attach under this group
        self.bg_id = Some(bg_id);
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

        // sync background rect
        if let Some(bg_id) = self.bg_id {
            if let Some(Node::Rect(r)) = scene.get_mut(bg_id) {
                r.x = l.x;
                r.y = l.y;
                r.w = l.w;
                r.h = l.h;
                r.color = self.color.unwrap_or([0.0, 0.0, 0.0, 0.0]);
                r.radii = self.radii;
                r.border_color = self.border_color;
                r.border_widths = self.border_widths;
            } 
        }
    }
}
