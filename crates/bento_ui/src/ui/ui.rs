use std::any::{Any, TypeId};
use std::collections::HashMap;

use bento_shared::{GroupNode, ImageNode, RectNode, Scene, SceneNode, SceneNodeId, TextNode};

use crate::input::InputState;
use crate::ui::asyncs::AsyncEventQueue;

pub struct Ui {
    pub scene: Scene,
    pub input: InputState,
    pub asyncs: AsyncEventQueue,
    pub needs_redraw: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            input: InputState::new(),
            asyncs: AsyncEventQueue::new(),
            needs_redraw: false,
        }
    }

    pub fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    pub fn process_input(&mut self) {}

    pub fn add_rect(&mut self, x: f32, y: f32, w: f32, h: f32) -> SceneNodeId {
        self.scene.add_rect(RectNode::new(x, y, w, h))
    }

    pub fn add_text(&mut self, text: &str, x: f32, y: f32, size: f32) -> SceneNodeId {
        self.scene.add_text(TextNode::new(text, x, y, size))
    }

    pub fn add_image(&mut self, x: f32, y: f32, w: f32, h: f32, image_id: u64) -> SceneNodeId {
        self.scene.add_image(ImageNode::new(x, y, w, h, image_id))
    }

    pub fn add_group(&mut self, x: f32, y: f32, w: f32, h: f32) -> SceneNodeId {
        let mut g = GroupNode::new();
        g.x = x;
        g.y = y;
        g.w = w;
        g.h = h;
        self.scene.add_group(g)
    }

    pub fn rect(&mut self, id: SceneNodeId) -> &mut RectNode {
        if let Some(SceneNode::Rect(r)) = self.scene.get_mut(id) {
            r
        } else {
            panic!("No rect at {:?}", id)
        }
    }

    pub fn text(&mut self, id: SceneNodeId) -> &mut TextNode {
        if let Some(SceneNode::Text(t)) = self.scene.get_mut(id) {
            t
        } else {
            panic!("No text at {:?}", id)
        }
    }

    pub fn image(&mut self, id: SceneNodeId) -> &mut ImageNode {
        if let Some(SceneNode::Image(i)) = self.scene.get_mut(id) {
            i
        } else {
            panic!("No image at {:?}", id)
        }
    }

    pub fn group(&mut self, id: SceneNodeId) -> &mut GroupNode {
        if let Some(SceneNode::Group(g)) = self.scene.get_mut(id) {
            g
        } else {
            panic!("No group at {:?}", id)
        }
    }

    pub fn remove(&mut self, id: SceneNodeId) {
        self.scene.remove(id);
        self.needs_redraw = true;
    }

    pub fn append(&mut self, parent: SceneNodeId, child: SceneNodeId) {
        self.scene.append(parent, child);
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}
