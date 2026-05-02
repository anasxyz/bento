use slab::Slab;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SceneNodeId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RectId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShadowId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClipId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransformId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpacityId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlurId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageKey(pub u64);

impl From<RectId> for SceneNodeId {
    fn from(id: RectId) -> Self {
        SceneNodeId(id.0)
    }
}
impl From<TextId> for SceneNodeId {
    fn from(id: TextId) -> Self {
        SceneNodeId(id.0)
    }
}
impl From<ShadowId> for SceneNodeId {
    fn from(id: ShadowId) -> Self {
        SceneNodeId(id.0)
    }
}
impl From<ClipId> for SceneNodeId {
    fn from(id: ClipId) -> Self {
        SceneNodeId(id.0)
    }
}
impl From<TransformId> for SceneNodeId {
    fn from(id: TransformId) -> Self {
        SceneNodeId(id.0)
    }
}
impl From<OpacityId> for SceneNodeId {
    fn from(id: OpacityId) -> Self {
        SceneNodeId(id.0)
    }
}
impl From<ImageId> for SceneNodeId {
    fn from(id: ImageId) -> Self {
        SceneNodeId(id.0)
    }
}
impl From<BlurId> for SceneNodeId {
    fn from(id: BlurId) -> Self {
        SceneNodeId(id.0)
    }
}

// ── Per-node transform ────────────────────────────────────────────────────────
// Present on every leaf node. Origin=None auto-centers to (w/2, h/2) at render
// time for rects/images/shadows; text defaults to (0,0) top-left.
#[derive(Clone)]
pub struct NodeTransform {
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub origin: Option<(f32, f32)>,
}

impl NodeTransform {
    pub fn identity() -> Self {
        Self {
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            origin: None,
        }
    }
    pub fn is_identity(&self) -> bool {
        self.rotate == 0.0 && self.scale_x == 1.0 && self.scale_y == 1.0
    }
    pub fn resolved_origin(&self, w: f32, h: f32) -> (f32, f32) {
        self.origin.unwrap_or((w * 0.5, h * 0.5))
    }
}

// ── Gradient ──────────────────────────────────────────────────────────────────
// Linear gradient for rect fills. When stops is empty the solid color is used.
// Angle is in radians, measured from the positive X axis.
#[derive(Clone)]
pub struct GradientStop {
    pub position: f32, // 0.0 – 1.0 along the gradient axis
    pub color: [f32; 4],
}

#[derive(Clone)]
pub struct Gradient {
    pub angle: f32,               // radians
    pub stops: Vec<GradientStop>, // must have >= 2 stops to be active
}

impl Gradient {
    pub fn linear(angle_radians: f32, stops: Vec<GradientStop>) -> Self {
        Self {
            angle: angle_radians,
            stops,
        }
    }
    pub fn is_active(&self) -> bool {
        self.stops.len() >= 2
    }
}

// ── RectNode ──────────────────────────────────────────────────────────────────
pub struct RectNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub gradient: Option<Gradient>,
    pub radius: f32,
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub opacity: f32,
    pub z: i32,
    pub visible: bool,
    pub transform: NodeTransform,
    pub(crate) slot: u32,
}

impl RectNode {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            color: [0.0; 4],
            gradient: None,
            radius: 0.0,
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            opacity: 1.0,
            z: 0,
            visible: false,
            transform: NodeTransform::identity(),
            slot: u32::MAX,
        }
    }
    pub fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    pub fn set_size(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
    }
    pub fn set_color(&mut self, c: [f32; 4]) {
        self.color = c;
    }
    pub fn set_gradient(&mut self, g: Gradient) {
        self.gradient = Some(g);
    }
    pub fn clear_gradient(&mut self) {
        self.gradient = None;
    }
    pub fn set_radius(&mut self, r: f32) {
        self.radius = r;
    }
    pub fn set_border_color(&mut self, c: [f32; 4]) {
        self.border_color = c;
    }
    pub fn set_border_widths(&mut self, w: [f32; 4]) {
        self.border_widths = w;
    }
    pub fn set_opacity(&mut self, v: f32) {
        self.opacity = v.clamp(0.0, 1.0);
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }
    pub fn set_rotate(&mut self, r: f32) {
        self.transform.rotate = r;
    }
    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.transform.scale_x = x;
        self.transform.scale_y = y;
    }
    pub fn set_scale_uniform(&mut self, s: f32) {
        self.transform.scale_x = s;
        self.transform.scale_y = s;
    }
    pub fn set_transform_origin(&mut self, x: f32, y: f32) {
        self.transform.origin = Some((x, y));
    }
    pub fn clear_transform_origin(&mut self) {
        self.transform.origin = None;
    }
}

// ── TextDecoration ────────────────────────────────────────────────────────────
#[derive(Clone)]
pub struct TextDecoration {
    pub start: usize,
    pub end: usize,
    pub color: [f32; 4],
    pub thickness: f32,
}

// ── TextNode ──────────────────────────────────────────────────────────────────
pub struct TextNode {
    pub x: f32,
    pub y: f32,
    pub content: String,
    pub family: String,
    pub size: f32,
    pub weight: u16,
    pub italic: bool,
    pub color: [f32; 4],
    pub width: f32,
    pub opacity: f32,
    pub z: i32,
    pub visible: bool,
    pub transform: NodeTransform,
    pub selection_start: Option<usize>,
    pub selection_end: Option<usize>,
    pub selection_color: [f32; 4],
    pub underlines: Vec<TextDecoration>,
    pub strikethroughs: Vec<TextDecoration>,
}

impl TextNode {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            content: String::new(),
            family: String::new(),
            size: 14.0,
            weight: 400,
            italic: false,
            color: [1.0, 1.0, 1.0, 1.0],
            width: f32::MAX,
            opacity: 1.0,
            z: 0,
            visible: false,
            transform: NodeTransform::identity(),
            selection_start: None,
            selection_end: None,
            selection_color: [0.267, 0.596, 0.890, 0.314],
            underlines: Vec::new(),
            strikethroughs: Vec::new(),
        }
    }
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    pub fn set_content(&mut self, s: &str) {
        self.content.clear();
        self.content.push_str(s);
    }
    pub fn set_family(&mut self, s: &str) {
        self.family.clear();
        self.family.push_str(s);
    }
    pub fn set_size(&mut self, v: f32) {
        self.size = v;
    }
    pub fn set_weight(&mut self, v: u16) {
        self.weight = v;
    }
    pub fn set_italic(&mut self, v: bool) {
        self.italic = v;
    }
    pub fn set_color(&mut self, c: [f32; 4]) {
        self.color = c;
    }
    pub fn set_width(&mut self, w: f32) {
        self.width = w;
    }
    pub fn set_opacity(&mut self, v: f32) {
        self.opacity = v.clamp(0.0, 1.0);
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }
    pub fn set_rotate(&mut self, r: f32) {
        self.transform.rotate = r;
    }
    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.transform.scale_x = x;
        self.transform.scale_y = y;
    }
    pub fn set_scale_uniform(&mut self, s: f32) {
        self.transform.scale_x = s;
        self.transform.scale_y = s;
    }
    pub fn set_transform_origin(&mut self, x: f32, y: f32) {
        self.transform.origin = Some((x, y));
    }
    pub fn clear_transform_origin(&mut self) {
        self.transform.origin = None;
    }
    pub fn set_selection(&mut self, start: usize, end: usize) {
        self.selection_start = Some(start);
        self.selection_end = Some(end);
    }
    pub fn set_selection_color(&mut self, c: [f32; 4]) {
        self.selection_color = c;
    }
    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }
    pub fn has_selection(&self) -> bool {
        matches!((self.selection_start, self.selection_end), (Some(s), Some(e)) if s < e)
    }
    pub fn add_underline(&mut self, start: usize, end: usize, color: [f32; 4], thickness: f32) {
        self.underlines.push(TextDecoration {
            start,
            end,
            color,
            thickness,
        });
    }
    pub fn add_strikethrough(&mut self, start: usize, end: usize, color: [f32; 4], thickness: f32) {
        self.strikethroughs.push(TextDecoration {
            start,
            end,
            color,
            thickness,
        });
    }
    pub fn clear_underlines(&mut self) {
        self.underlines.clear();
    }
    pub fn clear_strikethroughs(&mut self) {
        self.strikethroughs.clear();
    }
}

// ── ShadowNode ────────────────────────────────────────────────────────────────
pub struct ShadowNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub blur: f32,
    pub radius: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub opacity: f32,
    pub visible: bool,
    pub z: i32,
    pub transform: NodeTransform,
    pub(crate) slot: u32,
}

impl ShadowNode {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            color: [0.0, 0.0, 0.0, 0.8],
            blur: 8.0,
            radius: 0.0,
            offset_x: 0.0,
            offset_y: 2.0,
            opacity: 1.0,
            visible: false,
            z: 0,
            transform: NodeTransform::identity(),
            slot: u32::MAX,
        }
    }
    pub fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
    pub fn set_color(&mut self, c: [f32; 4]) {
        self.color = c;
    }
    pub fn set_blur(&mut self, b: f32) {
        self.blur = b;
    }
    pub fn set_radius(&mut self, r: f32) {
        self.radius = r;
    }
    pub fn set_offset(&mut self, x: f32, y: f32) {
        self.offset_x = x;
        self.offset_y = y;
    }
    pub fn set_opacity(&mut self, v: f32) {
        self.opacity = v.clamp(0.0, 1.0);
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
    pub fn set_rotate(&mut self, r: f32) {
        self.transform.rotate = r;
    }
    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.transform.scale_x = x;
        self.transform.scale_y = y;
    }
    pub fn set_scale_uniform(&mut self, s: f32) {
        self.transform.scale_x = s;
        self.transform.scale_y = s;
    }
    pub fn set_transform_origin(&mut self, x: f32, y: f32) {
        self.transform.origin = Some((x, y));
    }
    pub fn clear_transform_origin(&mut self) {
        self.transform.origin = None;
    }
}

// ── ClipNode ──────────────────────────────────────────────────────────────────
// When the accumulated parent transform has rotation/scale the clip uses
// the stencil path. Otherwise it uses the fast axis-aligned scissor path.
pub struct ClipNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub radius: f32,
    pub children: Vec<SceneNodeId>,
}

impl ClipNode {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            radius: 0.0,
            children: Vec::new(),
        }
    }
    pub fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
    pub fn set_radius(&mut self, r: f32) {
        self.radius = r;
    }
}

// ── TransformNode ─────────────────────────────────────────────────────────────
pub struct TransformNode {
    pub offset_x: f32,
    pub offset_y: f32,
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub origin_x: f32,
    pub origin_y: f32,
    pub children: Vec<SceneNodeId>,
}

impl TransformNode {
    pub fn new() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
            children: Vec::new(),
        }
    }
    pub fn set_offset(&mut self, x: f32, y: f32) {
        self.offset_x = x;
        self.offset_y = y;
    }
    pub fn set_rotate(&mut self, r: f32) {
        self.rotate = r;
    }
    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.scale_x = x;
        self.scale_y = y;
    }
    pub fn set_scale_uniform(&mut self, s: f32) {
        self.scale_x = s;
        self.scale_y = s;
    }
    pub fn set_origin(&mut self, x: f32, y: f32) {
        self.origin_x = x;
        self.origin_y = y;
    }
}

// ── OpacityNode ───────────────────────────────────────────────────────────────
pub struct OpacityNode {
    pub opacity: f32,
    pub children: Vec<SceneNodeId>,
}

impl OpacityNode {
    pub fn new() -> Self {
        Self {
            opacity: 1.0,
            children: Vec::new(),
        }
    }
    pub fn set_opacity(&mut self, v: f32) {
        self.opacity = v.clamp(0.0, 1.0);
    }
}

// ── ImageNode ─────────────────────────────────────────────────────────────────
pub struct ImageNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub image_key: ImageKey,
    pub uv: [f32; 4],
    pub tint: [f32; 4],
    pub radius: f32,
    // Per-side border (top, right, bottom, left in logical px)
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub opacity: f32,
    pub z: i32,
    pub visible: bool,
    pub transform: NodeTransform,
    pub(crate) slot: u32,
}

impl ImageNode {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            image_key: ImageKey(0),
            uv: [0.0, 0.0, 1.0, 1.0],
            tint: [1.0, 1.0, 1.0, 1.0],
            radius: 0.0,
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            opacity: 1.0,
            z: 0,
            visible: false,
            transform: NodeTransform::identity(),
            slot: u32::MAX,
        }
    }
    pub fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
    pub fn set_image_key(&mut self, key: ImageKey) {
        self.image_key = key;
    }
    pub fn set_uv(&mut self, uv: [f32; 4]) {
        self.uv = uv;
    }
    pub fn set_tint(&mut self, t: [f32; 4]) {
        self.tint = t;
    }
    pub fn set_radius(&mut self, r: f32) {
        self.radius = r;
    }
    pub fn set_border_color(&mut self, c: [f32; 4]) {
        self.border_color = c;
    }
    pub fn set_border_widths(&mut self, w: [f32; 4]) {
        self.border_widths = w;
    }
    pub fn set_opacity(&mut self, v: f32) {
        self.opacity = v.clamp(0.0, 1.0);
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }
    pub fn set_rotate(&mut self, r: f32) {
        self.transform.rotate = r;
    }
    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.transform.scale_x = x;
        self.transform.scale_y = y;
    }
    pub fn set_scale_uniform(&mut self, s: f32) {
        self.transform.scale_x = s;
        self.transform.scale_y = s;
    }
    pub fn set_transform_origin(&mut self, x: f32, y: f32) {
        self.transform.origin = Some((x, y));
    }
    pub fn clear_transform_origin(&mut self) {
        self.transform.origin = None;
    }
}

// ── BlurNode (backdrop blur) ──────────────────────────────────────────────────
// Renders a frosted-glass blur over whatever is behind it.
// The blur region is a rounded rect at (x, y, w, h).
pub struct BlurNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub radius: f32,    // corner radius
    pub sigma: f32,     // blur radius in logical pixels (Gaussian sigma)
    pub tint: [f32; 4], // optional colour overlay on top of the blur
    pub opacity: f32,
    pub z: i32,
    pub visible: bool,
}

impl BlurNode {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            radius: 0.0,
            sigma: 8.0,
            tint: [1.0, 1.0, 1.0, 0.0],
            opacity: 1.0,
            z: 0,
            visible: false,
        }
    }
    pub fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
    pub fn set_radius(&mut self, r: f32) {
        self.radius = r;
    }
    pub fn set_sigma(&mut self, s: f32) {
        self.sigma = s;
    }
    pub fn set_tint(&mut self, t: [f32; 4]) {
        self.tint = t;
    }
    pub fn set_opacity(&mut self, v: f32) {
        self.opacity = v.clamp(0.0, 1.0);
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }
}

// ── SceneNode enum ────────────────────────────────────────────────────────────
pub enum SceneNode {
    Rect(RectNode),
    Text(TextNode),
    Shadow(ShadowNode),
    Clip(ClipNode),
    Transform(TransformNode),
    Opacity(OpacityNode),
    Image(ImageNode),
    Blur(BlurNode),
}

impl RectId {
    pub fn to_scene(self) -> SceneNodeId {
        self.into()
    }
}
impl TextId {
    pub fn to_scene(self) -> SceneNodeId {
        self.into()
    }
}
impl ShadowId {
    pub fn to_scene(self) -> SceneNodeId {
        self.into()
    }
}
impl ClipId {
    pub fn to_scene(self) -> SceneNodeId {
        self.into()
    }
}
impl TransformId {
    pub fn to_scene(self) -> SceneNodeId {
        self.into()
    }
}
impl OpacityId {
    pub fn to_scene(self) -> SceneNodeId {
        self.into()
    }
}
impl ImageId {
    pub fn to_scene(self) -> SceneNodeId {
        self.into()
    }
}
impl BlurId {
    pub fn to_scene(self) -> SceneNodeId {
        self.into()
    }
}
