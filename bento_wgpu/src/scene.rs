// scene.rs
//
// SceneGraph owns all nodes.
// Pure CPU data — no wgpu types, no GPU state anywhere in here.
//
// The caller (your framework) creates nodes, gets stable IDs back,
// and mutates nodes directly via rect_mut() / text_mut() / shadow_mut().
// The Renderer reads the scene graph each frame and handles all GPU work.
//
// Adding a new primitive type:
//   1. Add the struct + Id type to nodes.rs
//   2. Add a Slab<NewNode> field here
//   3. Add add_/remove_/get/get_mut methods following the same pattern

use crate::nodes::{RectId, RectNode, ShadowId, ShadowNode, TextId, TextNode};
use slab::Slab;

pub struct SceneGraph {
    pub(crate) rects: Slab<RectNode>,
    pub(crate) texts: Slab<TextNode>,
    pub(crate) shadows: Slab<ShadowNode>,

    dirty_rects: Vec<RectId>, // only nodes that changed this frame
    dirty_texts: Vec<TextId>,
    dirty_shadows: Vec<ShadowId>,
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            rects: Slab::new(),
            texts: Slab::new(),
            shadows: Slab::new(),

            dirty_rects: Vec::new(),
            dirty_texts: Vec::new(),
            dirty_shadows: Vec::new(),
        }
    }

    // ── rect ──────────────────────────────────────────────────────────────────

    /// Allocate a new invisible rect node. Returns a stable ID.
    pub fn add_rect(&mut self) -> RectId {
        let id = RectId(self.rects.insert(RectNode::new()));
        self.dirty_rects.push(id); // new nodes start dirty
        id
    }

    /// Remove and reclaim the rect node. The ID becomes invalid.
    pub fn remove_rect(&mut self, id: RectId) {
        self.rects.remove(id.0);
        self.dirty_rects.retain(|r| *r != id);
    }

    pub fn rect(&self, id: RectId) -> &RectNode {
        &self.rects[id.0]
    }

    pub fn rect_mut(&mut self, id: RectId) -> &mut RectNode {
        let node = &mut self.rects[id.0];
        if !node.dirty {
            node.dirty = true;
            self.dirty_rects.push(id); // only added once even if called multiple times
        }
        node
    }

    // ── text ──────────────────────────────────────────────────────────────────

    pub fn add_text(&mut self) -> TextId {
        let id = TextId(self.texts.insert(TextNode::new()));
        self.dirty_texts.push(id); // new nodes start dirty
        id
    }

    pub fn remove_text(&mut self, id: TextId) {
        self.texts.remove(id.0);
        self.dirty_texts.retain(|t| *t != id);
    }

    pub fn text(&self, id: TextId) -> &TextNode {
        &self.texts[id.0]
    }

    pub fn text_mut(&mut self, id: TextId) -> &mut TextNode {
        let node = &mut self.texts[id.0];
        if !node.dirty {
            node.dirty = true;
            self.dirty_texts.push(id);
        }
        node
    }

    // ── shadow ────────────────────────────────────────────────────────────────

    pub fn add_shadow(&mut self) -> ShadowId {
        let id = ShadowId(self.shadows.insert(ShadowNode::new()));
        self.dirty_shadows.push(id); // new nodes start dirty
        id
    }

    pub fn remove_shadow(&mut self, id: ShadowId) {
        self.shadows.remove(id.0);
        self.dirty_shadows.retain(|s| *s != id);
    }

    pub fn shadow(&self, id: ShadowId) -> &ShadowNode {
        &self.shadows[id.0]
    }

    pub fn shadow_mut(&mut self, id: ShadowId) -> &mut ShadowNode {
        let node = &mut self.shadows[id.0];
        if !node.dirty {
            node.dirty = true;
            self.dirty_shadows.push(id);
        }
        node
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}
