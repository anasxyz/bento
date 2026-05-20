use bento_shared::{RectNode, SceneNodeId};

use crate::{ui::Ui, widget::Widget};

pub struct Rect {
    id: Option<SceneNodeId>,
}

impl Rect {
    pub fn new() -> Self {
        Self { id: None }
    }
}

impl Widget for Rect {
    fn build(&mut self, ui: &mut Ui) {
        self.id = Some(
            ui.scene_mut()
                .add_rect(RectNode::new(0.0, 0.0, 100.0, 100.0)),
        );
    }
    fn update(&mut self, ui: &mut Ui) {}
}
