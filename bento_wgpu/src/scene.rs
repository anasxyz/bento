use crate::nodes::*;
use slab::Slab;

pub struct SceneGraph {
    pub nodes: Slab<SceneNode>,
    pub root: SceneNodeId,
}

#[derive(Clone)]
pub struct TraversalState {
    pub offset_x: f32,
    pub offset_y: f32,
    pub opacity: f32,
    pub clip: Option<[f32; 4]>,
}

impl TraversalState {
    pub fn new() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            opacity: 1.0,
            clip: None,
        }
    }

    pub fn add_offset(&self, x: f32, y: f32) -> Self {
        Self {
            offset_x: self.offset_x + x,
            offset_y: self.offset_y + y,
            ..self.clone()
        }
    }

    pub fn multiply_opacity(&self, o: f32) -> Self {
        Self {
            opacity: self.opacity * o,
            ..self.clone()
        }
    }

    pub fn intersect_clip(&self, x: f32, y: f32, w: f32, h: f32) -> Self {
        let new_clip = [x, y, x + w, y + h];
        let clip = match self.clip {
            None => new_clip,
            Some([cx, cy, cx2, cy2]) => [
                cx.max(new_clip[0]),
                cy.max(new_clip[1]),
                cx2.min(new_clip[2]),
                cy2.min(new_clip[3]),
            ],
        };
        Self {
            clip: Some(clip),
            ..self.clone()
        }
    }
}

impl SceneGraph {
    pub fn new() -> Self {
        let mut nodes = Slab::new();
        let root_id = nodes.insert(SceneNode::Transform(TransformNode::new()));
        Self {
            nodes,
            root: SceneNodeId(root_id),
        }
    }

    /// call f, return all SceneNodeIds created during f
    pub fn track_build<F: FnMut(&mut Self)>(&mut self, mut f: F) -> Vec<SceneNodeId> {
        let before: std::collections::HashSet<usize> = self.nodes.iter().map(|(i, _)| i).collect();
        f(self);
        self.nodes
            .iter()
            .map(|(i, _)| SceneNodeId(i))
            .filter(|id| !before.contains(&id.0))
            .collect()
    }

    pub fn add_rect(&mut self) -> RectId {
        RectId(self.nodes.insert(SceneNode::Rect(RectNode::new())))
    }

    pub fn add_text(&mut self) -> TextId {
        TextId(self.nodes.insert(SceneNode::Text(TextNode::new())))
    }

    pub fn add_shadow(&mut self) -> ShadowId {
        ShadowId(self.nodes.insert(SceneNode::Shadow(ShadowNode::new())))
    }

    pub fn add_clip(&mut self) -> ClipId {
        ClipId(self.nodes.insert(SceneNode::Clip(ClipNode::new())))
    }

    pub fn add_transform(&mut self) -> TransformId {
        TransformId(
            self.nodes
                .insert(SceneNode::Transform(TransformNode::new())),
        )
    }

    pub fn add_opacity(&mut self) -> OpacityId {
        OpacityId(self.nodes.insert(SceneNode::Opacity(OpacityNode::new())))
    }

    pub fn rect(&self, id: RectId) -> &RectNode {
        match &self.nodes[id.0] {
            SceneNode::Rect(n) => n,
            _ => panic!("not a rect"),
        }
    }
    pub fn rect_mut(&mut self, id: RectId) -> &mut RectNode {
        match &mut self.nodes[id.0] {
            SceneNode::Rect(n) => n,
            _ => panic!("not a rect"),
        }
    }

    pub fn text(&self, id: TextId) -> &TextNode {
        match &self.nodes[id.0] {
            SceneNode::Text(n) => n,
            _ => panic!("not a text"),
        }
    }
    pub fn text_mut(&mut self, id: TextId) -> &mut TextNode {
        match &mut self.nodes[id.0] {
            SceneNode::Text(n) => n,
            _ => panic!("not a text"),
        }
    }

    pub fn shadow(&self, id: ShadowId) -> &ShadowNode {
        match &self.nodes[id.0] {
            SceneNode::Shadow(n) => n,
            _ => panic!("not a shadow"),
        }
    }
    pub fn shadow_mut(&mut self, id: ShadowId) -> &mut ShadowNode {
        match &mut self.nodes[id.0] {
            SceneNode::Shadow(n) => n,
            _ => panic!("not a shadow"),
        }
    }

    pub fn clip_mut(&mut self, id: ClipId) -> &mut ClipNode {
        match &mut self.nodes[id.0] {
            SceneNode::Clip(n) => n,
            _ => panic!("not a clip"),
        }
    }

    pub fn transform_mut(&mut self, id: TransformId) -> &mut TransformNode {
        match &mut self.nodes[id.0] {
            SceneNode::Transform(n) => n,
            _ => panic!("not a transform"),
        }
    }

    pub fn opacity_mut(&mut self, id: OpacityId) -> &mut OpacityNode {
        match &mut self.nodes[id.0] {
            SceneNode::Opacity(n) => n,
            _ => panic!("not an opacity"),
        }
    }

    pub fn add_child(&mut self, parent: SceneNodeId, child: SceneNodeId) {
        match &mut self.nodes[parent.0] {
            SceneNode::Transform(n) => n.children.push(child),
            SceneNode::Clip(n) => n.children.push(child),
            SceneNode::Opacity(n) => n.children.push(child),
            _ => panic!("can only add children to group nodes"),
        }
    }

    pub fn remove_child(&mut self, parent: SceneNodeId, child: SceneNodeId) {
        match &mut self.nodes[parent.0] {
            SceneNode::Transform(n) => n.children.retain(|c| *c != child),
            SceneNode::Clip(n) => n.children.retain(|c| *c != child),
            SceneNode::Opacity(n) => n.children.retain(|c| *c != child),
            _ => {}
        }
    }

    pub fn remove_node(&mut self, id: SceneNodeId) {
        self.nodes.remove(id.0);
    }

    /// traverse the scene tree, calling f for each leaf node
    /// f receives the node, its slab index, and the accumulated traversal state
    pub fn traverse<F>(&self, node_id: SceneNodeId, state: TraversalState, f: &mut F)
    where
        F: FnMut(&SceneNode, usize, &TraversalState),
    {
        let node = &self.nodes[node_id.0];
        match node {
            SceneNode::Rect(_) | SceneNode::Text(_) | SceneNode::Shadow(_) => {
                f(node, node_id.0, &state);
            }
            SceneNode::Transform(n) => {
                let new_state = state.add_offset(n.offset_x, n.offset_y);
                for &child in &n.children {
                    self.traverse(child, new_state.clone(), f);
                }
            }
            SceneNode::Clip(n) => {
                let new_state = state.intersect_clip(n.x, n.y, n.w, n.h);
                for &child in &n.children {
                    self.traverse(child, new_state.clone(), f);
                }
            }
            SceneNode::Opacity(n) => {
                let new_state = state.multiply_opacity(n.opacity);
                for &child in &n.children {
                    self.traverse(child, new_state.clone(), f);
                }
            }
        }
    }
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}
