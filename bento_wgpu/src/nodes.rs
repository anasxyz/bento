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
pub struct ImageKey(pub u64);

pub struct RectNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub radius: f32,
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub z: i32,
    pub visible: bool,
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
            radius: 0.0,
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            z: 0,
            visible: false,
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
    pub fn set_radius(&mut self, r: f32) {
        self.radius = r;
    }
    pub fn set_border_color(&mut self, c: [f32; 4]) {
        self.border_color = c;
    }
    pub fn set_border_widths(&mut self, w: [f32; 4]) {
        self.border_widths = w;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }
}

#[derive(Clone)]
pub struct TextDecoration {
    pub start: usize,
    pub end: usize,
    pub color: [f32; 4],
    pub thickness: f32,
}

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
    pub z: i32,
    pub visible: bool,
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
            z: 0,
            visible: false,
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
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
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
    pub visible: bool,
    pub z: i32,
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
            visible: false,
            z: 0,
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
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
}

pub struct ClipNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub children: Vec<SceneNodeId>,
}

impl ClipNode {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            children: Vec::new(),
        }
    }
    pub fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
}

pub struct TransformNode {
    pub offset_x: f32,
    pub offset_y: f32,
    pub children: Vec<SceneNodeId>,
}

impl TransformNode {
    pub fn new() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            children: Vec::new(),
        }
    }
    pub fn set_offset(&mut self, x: f32, y: f32) {
        self.offset_x = x;
        self.offset_y = y;
    }
}

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
        self.opacity = v;
    }
}

pub struct ImageNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub image_key: ImageKey,
    pub uv: [f32; 4],
    pub tint: [f32; 4],
    pub radius: f32,
    pub z: i32,
    pub visible: bool,
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
            z: 0,
            visible: false,
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
    pub fn set_tint(&mut self, tint: [f32; 4]) {
        self.tint = tint;
    }
    pub fn set_radius(&mut self, r: f32) {
        self.radius = r;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }
}

pub enum SceneNode {
    Rect(RectNode),
    Text(TextNode),
    Shadow(ShadowNode),
    Clip(ClipNode),
    Transform(TransformNode),
    Opacity(OpacityNode),
    Image(ImageNode),
}

impl RectId {
    pub fn to_scene(self) -> SceneNodeId {
        SceneNodeId(self.0)
    }
}
impl TextId {
    pub fn to_scene(self) -> SceneNodeId {
        SceneNodeId(self.0)
    }
}
impl ShadowId {
    pub fn to_scene(self) -> SceneNodeId {
        SceneNodeId(self.0)
    }
}
impl ClipId {
    pub fn to_scene(self) -> SceneNodeId {
        SceneNodeId(self.0)
    }
}
impl TransformId {
    pub fn to_scene(self) -> SceneNodeId {
        SceneNodeId(self.0)
    }
}
impl OpacityId {
    pub fn to_scene(self) -> SceneNodeId {
        SceneNodeId(self.0)
    }
}
impl ImageId {
    pub fn to_scene(self) -> SceneNodeId {
        SceneNodeId(self.0)
    }
}
