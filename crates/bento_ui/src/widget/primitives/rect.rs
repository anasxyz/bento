use std::any::Any;

use bento_shared::{SceneNode, RectNode, Scene, SceneNodeId};

use crate::{AsAny, Widget};

#[derive(Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

    rect_id: Option<SceneNodeId>,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            x,
            y,
            w,
            h,

            rect_id: None,
        }
    }
}

impl Widget for Rect {
    fn name(&self) -> &str {
        "Rect"
    }

    fn build(&mut self, scene: &mut Scene) {
        let mut node = RectNode::new(self.x, self.y, self.w, self.h);
        self.rect_id = Some(scene.add_rect(node));
    }

    fn update(&mut self, scene: &mut Scene) {
        // Return if no SceneNodeId is set
        // If that's the case, build() was not called or something went wrong
        let Some(id) = self.rect_id else { return };

        // Look up SceneNode in Scene by id
        // Pattern match to get the inner RectNode as mutable
        // Return if SceneNode is not RectNode or missing
        let Some(SceneNode::Rect(r)) = scene.get_mut(id) else {
            return;
        };

        // Write widget's values to RectNode
        r.x = self.x;
        r.y = self.y;
        r.w = self.w;
        r.h = self.h;
    }
}

// TODO: add to proc macro
impl AsAny for Rect {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
