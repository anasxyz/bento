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

use slab::Slab;
use crate::nodes::{RectNode, RectId, TextNode, TextId, ShadowNode, ShadowId};

pub struct SceneGraph {
    pub(crate) rects:   Slab<RectNode>,
    pub(crate) texts:   Slab<TextNode>,
    pub(crate) shadows: Slab<ShadowNode>,
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            rects:   Slab::new(),
            texts:   Slab::new(),
            shadows: Slab::new(),
        }
    }

    // ── rect ──────────────────────────────────────────────────────────────────

    /// Allocate a new invisible rect node. Returns a stable ID.
    pub fn add_rect(&mut self) -> RectId {
        RectId(self.rects.insert(RectNode::new()))
    }

    /// Remove and reclaim the rect node. The ID becomes invalid.
    pub fn remove_rect(&mut self, id: RectId) {
        self.rects.remove(id.0);
    }

    pub fn rect(&self, id: RectId) -> &RectNode {
        &self.rects[id.0]
    }

    pub fn rect_mut(&mut self, id: RectId) -> &mut RectNode {
        &mut self.rects[id.0]
    }

    // ── text ──────────────────────────────────────────────────────────────────

    pub fn add_text(&mut self) -> TextId {
        TextId(self.texts.insert(TextNode::new()))
    }

    pub fn remove_text(&mut self, id: TextId) {
        self.texts.remove(id.0);
    }

    pub fn text(&self, id: TextId) -> &TextNode {
        &self.texts[id.0]
    }

    pub fn text_mut(&mut self, id: TextId) -> &mut TextNode {
        &mut self.texts[id.0]
    }

    // ── shadow ────────────────────────────────────────────────────────────────

    pub fn add_shadow(&mut self) -> ShadowId {
        ShadowId(self.shadows.insert(ShadowNode::new()))
    }

    pub fn remove_shadow(&mut self, id: ShadowId) {
        self.shadows.remove(id.0);
    }

    pub fn shadow(&self, id: ShadowId) -> &ShadowNode {
        &self.shadows[id.0]
    }

    pub fn shadow_mut(&mut self, id: ShadowId) -> &mut ShadowNode {
        &mut self.shadows[id.0]
    }
}

impl Default for SceneGraph {
    fn default() -> Self { Self::new() }
}
