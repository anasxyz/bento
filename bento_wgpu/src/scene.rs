// scenegraph owns all nodes
// just data
//
// dirty tracking lives here, not on individual nodes
//
// *_mut() accessors push the id onto the dirty list the first time 
// a node is mutated each frame
//
// the renderer drains those lists meaning zero iterations when nothing changed
//
// Adding a new primitive type:
//   1. Add struct + Id to nodes.rs
//   2. Add Slab + dirty Vec + CRUD methods here following the same pattern
//   3. Add pipeline in pipelines/, wire into renderer.rs
//
// to add a new primitive type you just:
//   1. add a struct + Id to nodes.rs
//   2. add a Slab + dirty Vec + crud methods here following the same pattern
//   3. add a pipeline in pipelines/ dir then wire into renderer.rs

use crate::nodes::{RectId, RectNode, ShadowId, ShadowNode, TextId, TextNode};
use slab::Slab;

pub struct SceneGraph {
    pub(crate) rects: Slab<RectNode>,
    pub(crate) texts: Slab<TextNode>,
    pub(crate) shadows: Slab<ShadowNode>,

    // ids of nodes that changed since the last render() call
    // populated by *_mut() accessors, drained by the renderer
    pub(crate) dirty_rects: Vec<RectId>,
    pub(crate) dirty_texts: Vec<TextId>,
    pub(crate) dirty_shadows: Vec<ShadowId>,
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

    /// allocate a new rect node
    /// starts invisible and dirty so the renderer
    /// assigns it a gpu slot on the first frame
    pub fn add_rect(&mut self) -> RectId {
        let id = RectId(self.rects.insert(RectNode::new()));
        self.dirty_rects.push(id);
        id
    }

    /// remove the rect node
    /// its gpu slot is reclaimed separately via renderer::free_rect_slot() if 
    /// immediate reuse is wanted
    pub fn remove_rect(&mut self, id: RectId) {
        self.rects.remove(id.0);
        self.dirty_rects.retain(|r| *r != id);
    }

    pub fn rect(&self, id: RectId) -> &RectNode {
        &self.rects[id.0]
    }

    /// mutable access
    ///
    /// pushes id onto dirty_rects the first time each frame so the renderer 
    /// knows to reupload this node
    pub fn rect_mut(&mut self, id: RectId) -> &mut RectNode {
        let node = &mut self.rects[id.0];
        if !node.dirty {
            node.dirty = true;
            self.dirty_rects.push(id);
        }
        node
    }

    pub fn add_text(&mut self) -> TextId {
        let id = TextId(self.texts.insert(TextNode::new()));
        self.dirty_texts.push(id);
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

    pub fn add_shadow(&mut self) -> ShadowId {
        let id = ShadowId(self.shadows.insert(ShadowNode::new()));
        self.dirty_shadows.push(id);
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

    /// true if any node changed since the last render
    /// useful for deciding whether to request a redraw from the windowing system
    pub fn is_dirty(&self) -> bool {
        !self.dirty_rects.is_empty()
            || !self.dirty_texts.is_empty()
            || !self.dirty_shadows.is_empty()
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}
