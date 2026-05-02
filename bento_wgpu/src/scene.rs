use crate::nodes::*;
use slab::Slab;

pub struct SceneGraph {
    pub nodes: Slab<SceneNode>,
    pub root: SceneNodeId,
}

// A 2x3 affine transform matrix stored column-major as [a, b, c, d, tx, ty]:
//
//   | a  c  tx |
//   | b  d  ty |
//   | 0  0   1 |
//
// Applying to a point p:  p' = (a*x + c*y + tx,  b*x + d*y + ty)
// Identity: [1, 0, 0, 1, 0, 0]
pub type Mat2x3 = [f32; 6];

pub fn mat_identity() -> Mat2x3 {
    [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
}

// Multiply two affine matrices: result = parent * local
// (i.e. apply local first, then parent)
pub fn mat_mul(p: Mat2x3, l: Mat2x3) -> Mat2x3 {
    [
        p[0] * l[0] + p[2] * l[1],        // a
        p[1] * l[0] + p[3] * l[1],        // b
        p[0] * l[2] + p[2] * l[3],        // c
        p[1] * l[2] + p[3] * l[3],        // d
        p[0] * l[4] + p[2] * l[5] + p[4], // tx
        p[1] * l[4] + p[3] * l[5] + p[5], // ty
    ]
}

// Apply a 2x3 affine matrix to a 2D point
pub fn mat_apply(m: Mat2x3, x: f32, y: f32) -> (f32, f32) {
    (m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5])
}

// Build a local matrix for a TransformNode:
//   T(offset) * T(origin) * R * S * T(-origin)
//
// Decomposed:
//   - scale around origin
//   - rotate around origin
//   - translate to final position
pub fn mat_trs_pub(
    offset_x: f32,
    offset_y: f32,
    rotate: f32,
    scale_x: f32,
    scale_y: f32,
    origin_x: f32,
    origin_y: f32,
) -> Mat2x3 {
    let cos_r = rotate.cos();
    let sin_r = rotate.sin();
    // Combined R*S columns
    let a = cos_r * scale_x;
    let b = sin_r * scale_x;
    let c = -sin_r * scale_y;
    let d = cos_r * scale_y;
    // Translation: place the origin, then offset
    // T(offset + origin) * RS * T(-origin)
    // tx = offset_x + origin_x + a*(-origin_x) + c*(-origin_y)
    // ty = offset_y + origin_y + b*(-origin_x) + d*(-origin_y)
    let tx = offset_x + origin_x - a * origin_x - c * origin_y;
    let ty = offset_y + origin_y - b * origin_x - d * origin_y;
    [a, b, c, d, tx, ty]
}

// Build a local matrix from a NodeTransform given the node's own size.
// Used to resolve the auto-center origin (None -> w/2, h/2).
pub fn mat_from_node_transform(
    nt: &crate::nodes::NodeTransform,
    node_x: f32,
    node_y: f32,
    node_w: f32,
    node_h: f32,
) -> Mat2x3 {
    let (ox, oy) = nt.resolved_origin(node_w, node_h);
    mat_trs_pub(node_x, node_y, nt.rotate, nt.scale_x, nt.scale_y, ox, oy)
}

#[derive(Clone)]
pub struct TraversalState {
    // accumulated affine transform from root to this node
    pub transform: Mat2x3,
    pub opacity: f32,
    // axis-aligned clip rect in screen space [x1, y1, x2, y2]
    // stays axis-aligned even under rotation — same as CSS overflow:hidden
    pub clip: Option<[f32; 4]>,
}

impl TraversalState {
    pub fn new() -> Self {
        Self {
            transform: mat_identity(),
            opacity: 1.0,
            clip: None,
        }
    }

    // Compose a TransformNode's TRS into the current state
    pub fn apply_trs(
        &self,
        offset_x: f32,
        offset_y: f32,
        rotate: f32,
        scale_x: f32,
        scale_y: f32,
        origin_x: f32,
        origin_y: f32,
    ) -> Self {
        let local = mat_trs_pub(
            offset_x, offset_y, rotate, scale_x, scale_y, origin_x, origin_y,
        );
        Self {
            transform: mat_mul(self.transform, local),
            opacity: self.opacity,
            clip: self.clip,
        }
    }

    // Fast path for pure translation (no rotation or scale change)
    pub fn add_offset(&self, x: f32, y: f32) -> Self {
        self.apply_trs(x, y, 0.0, 1.0, 1.0, 0.0, 0.0)
    }

    pub fn multiply_opacity(&self, o: f32) -> Self {
        Self {
            opacity: self.opacity * o,
            ..self.clone()
        }
    }

    // Clip rects are always axis-aligned in screen space.
    // We apply the current transform's translation to the clip origin so that
    // a ClipNode placed at (10, 10) inside a TransformNode translated by (100, 0)
    // correctly clips at screen space (110, 10).
    pub fn intersect_clip(&self, x: f32, y: f32, w: f32, h: f32) -> Self {
        let (sx, sy) = mat_apply(self.transform, x, y);
        let new_clip = [
            sx,
            sy,
            sx + w * self.transform[0],
            sy + h * self.transform[3],
        ];
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

    // Screen-space position of a local point, applying the accumulated transform.
    // Used by the renderer to get the transformed origin for culling.
    pub fn transform_point(&self, x: f32, y: f32) -> (f32, f32) {
        mat_apply(self.transform, x, y)
    }

    // Build the full per-instance transform for a leaf node at local (x, y).
    // Composes the node's local position into the inherited transform.
    pub fn leaf_transform(&self, x: f32, y: f32) -> Mat2x3 {
        let local = [1.0_f32, 0.0, 0.0, 1.0, x, y];
        mat_mul(self.transform, local)
    }

    // Build the full transform for a leaf node that has its own NodeTransform
    // (rotate/scale/origin). Composes: parent_transform * TRS(node).
    // node_w/h are the node dimensions used to resolve the auto-center origin.
    pub fn leaf_transform_with_node(
        &self,
        node_x: f32,
        node_y: f32,
        node_w: f32,
        node_h: f32,
        nt: &crate::nodes::NodeTransform,
    ) -> Mat2x3 {
        let local = mat_from_node_transform(nt, node_x, node_y, node_w, node_h);
        mat_mul(self.transform, local)
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
    pub fn add_image(&mut self) -> ImageId {
        ImageId(self.nodes.insert(SceneNode::Image(ImageNode::new())))
    }
    pub fn add_blur(&mut self) -> BlurId {
        BlurId(self.nodes.insert(SceneNode::Blur(BlurNode::new())))
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
    pub fn image_mut(&mut self, id: ImageId) -> &mut ImageNode {
        match &mut self.nodes[id.0] {
            SceneNode::Image(n) => n,
            _ => panic!("not an image"),
        }
    }
    pub fn blur_mut(&mut self, id: BlurId) -> &mut BlurNode {
        match &mut self.nodes[id.0] {
            SceneNode::Blur(n) => n,
            _ => panic!("not a blur"),
        }
    }

    pub fn add_child(&mut self, parent: impl Into<SceneNodeId>, child: impl Into<SceneNodeId>) {
        let parent = parent.into();
        let child = child.into();
        match &mut self.nodes[parent.0] {
            SceneNode::Transform(n) => n.children.push(child),
            SceneNode::Clip(n) => n.children.push(child),
            SceneNode::Opacity(n) => n.children.push(child),
            _ => panic!("can only add children to group nodes"),
        }
    }

    pub fn remove_child(&mut self, parent: impl Into<SceneNodeId>, child: impl Into<SceneNodeId>) {
        let parent = parent.into();
        let child = child.into();
        match &mut self.nodes[parent.0] {
            SceneNode::Transform(n) => n.children.retain(|c| *c != child),
            SceneNode::Clip(n) => n.children.retain(|c| *c != child),
            SceneNode::Opacity(n) => n.children.retain(|c| *c != child),
            _ => {}
        }
    }

    pub fn remove_node(&mut self, id: impl Into<SceneNodeId>) {
        self.nodes.remove(id.into().0);
    }

    pub fn top_level_of(&self, ids: &[SceneNodeId]) -> Vec<SceneNodeId> {
        let id_set: std::collections::HashSet<usize> = ids.iter().map(|id| id.0).collect();
        let mut child_set: std::collections::HashSet<usize> = std::collections::HashSet::new();
        for &node_id in ids {
            match &self.nodes[node_id.0] {
                SceneNode::Transform(n) => {
                    for c in &n.children {
                        if id_set.contains(&c.0) {
                            child_set.insert(c.0);
                        }
                    }
                }
                SceneNode::Clip(n) => {
                    for c in &n.children {
                        if id_set.contains(&c.0) {
                            child_set.insert(c.0);
                        }
                    }
                }
                SceneNode::Opacity(n) => {
                    for c in &n.children {
                        if id_set.contains(&c.0) {
                            child_set.insert(c.0);
                        }
                    }
                }
                _ => {}
            }
        }
        ids.iter()
            .copied()
            .filter(|id| !child_set.contains(&id.0))
            .collect()
    }

    pub fn traverse<F>(&self, node_id: SceneNodeId, state: TraversalState, f: &mut F)
    where
        F: FnMut(&SceneNode, usize, &TraversalState),
    {
        let node = &self.nodes[node_id.0];
        match node {
            SceneNode::Rect(_)
            | SceneNode::Text(_)
            | SceneNode::Shadow(_)
            | SceneNode::Image(_)
            | SceneNode::Blur(_) => {
                f(node, node_id.0, &state);
            }
            SceneNode::Transform(n) => {
                let new_state = state.apply_trs(
                    n.offset_x, n.offset_y, n.rotate, n.scale_x, n.scale_y, n.origin_x, n.origin_y,
                );
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
