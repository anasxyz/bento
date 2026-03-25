// plain data structs for every drawable primitive
// just cpu side data
//
// there ar e a few rules:
//   - all setters compare before assigning. only mark dirty if the value
//     actually changed. this means sync_layout style callers can call setters
//     every frame cheaply without causing unnecessary gpu uploads
//   - `dirty` and `slot` are pub(crate). tThe SceneGraph and Renderer manage
//     them, callers never touch them directly
//   - new primitive types follow the same pattern: add a struct here,
//     add a Slab in SceneGraph, add a pipeline in pipelines/

// ids 

/// stable identifier for a RectNode. returned by SceneGraph::add_rect()
/// remains valid until remove_rect() is called with it
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RectId(pub(crate) usize);

/// stable identifier for a TextNode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextId(pub(crate) usize);

/// stable identifier for a ShadowNode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShadowId(pub(crate) usize);

// RectNode 

/// a rounded rectangle with optional border and clip region
/// covers buttons, panels, inputs, highlights, most ui chrome
pub struct RectNode {
    // position and size in logical pixels
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

    // visuals
    pub color: [f32; 4],           // rgba, premultiplied alpha expected
    pub radius: f32,               // corner radius, logical pixels
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],   // [top, right, bottom, left]
    pub clip: Option<[f32; 4]>,    // scissor rect [x, y, x2, y2]
    pub z: i32,                    // draw order, lower draws first

    pub visible: bool,

    // renderer managed
    // DO NOT write from outside this crate
    pub(crate) dirty: bool,
    pub(crate) slot: u32,  // gpu instance buffer index, u32::MAX = unassigned
}

impl RectNode {
    pub(crate) fn new() -> Self {
        Self {
            x: 0.0, y: 0.0, w: 0.0, h: 0.0,
            color: [0.0; 4],
            radius: 0.0,
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            clip: None,
            z: 0,
            visible: false,
            dirty: true,
            slot: u32::MAX,
        }
    }

    // setters, compare before assign 

    pub fn set_pos(&mut self, x: f32, y: f32) {
        if self.x != x || self.y != y {
            self.x = x; self.y = y;
            self.dirty = true;
        }
    }

    pub fn set_size(&mut self, w: f32, h: f32) {
        if self.w != w || self.h != h {
            self.w = w; self.h = h;
            self.dirty = true;
        }
    }

    pub fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        if self.x != x || self.y != y || self.w != w || self.h != h {
            self.x = x; self.y = y; self.w = w; self.h = h;
            self.dirty = true;
        }
    }

    pub fn set_color(&mut self, c: [f32; 4]) {
        if self.color != c { self.color = c; self.dirty = true; }
    }

    pub fn set_radius(&mut self, r: f32) {
        if self.radius != r { self.radius = r; self.dirty = true; }
    }

    pub fn set_border_color(&mut self, c: [f32; 4]) {
        if self.border_color != c { self.border_color = c; self.dirty = true; }
    }

    pub fn set_border_widths(&mut self, w: [f32; 4]) {
        if self.border_widths != w { self.border_widths = w; self.dirty = true; }
    }

    pub fn set_clip(&mut self, clip: Option<[f32; 4]>) {
        if self.clip != clip { self.clip = clip; self.dirty = true; }
    }

    pub fn set_z(&mut self, z: i32) {
        if self.z != z { self.z = z; self.dirty = true; }
    }

    pub fn set_visible(&mut self, v: bool) {
        if self.visible != v { self.visible = v; self.dirty = true; }
    }
}

// TextNode 

/// a text primitive rendered via glyphon/cosmic-text
/// supports multiline, wrapping, font family/weight/style
pub struct TextNode {
    pub x: f32,
    pub y: f32,
    pub content: String,
    pub family: String,
    pub size: f32,
    pub weight: u16,
    pub italic: bool,
    pub color: [f32; 4],
    pub width: f32,               // max layout width; f32::MAX = unconstrained
    pub clip: Option<[f32; 4]>,
    pub z: i32,
    pub visible: bool,

    pub(crate) dirty: bool,
    // text has no gpu slot, glyphon manages its own atlas and buffers
    // the text pipeline rebuilds its submission list each frame from all
    // visible dirty TextNodes
}

impl TextNode {
    pub(crate) fn new() -> Self {
        Self {
            x: 0.0, y: 0.0,
            content: String::new(),
            family: String::new(),
            size: 14.0,
            weight: 400,
            italic: false,
            color: [1.0, 1.0, 1.0, 1.0],
            width: f32::MAX,
            clip: None,
            z: 0,
            visible: false,
            dirty: true,
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        if self.x != x || self.y != y {
            self.x = x; self.y = y;
            self.dirty = true;
        }
    }

    pub fn set_content(&mut self, s: &str) {
        if self.content != s {
            self.content.clear();
            self.content.push_str(s);
            self.dirty = true;
        }
    }

    pub fn set_family(&mut self, s: &str) {
        if self.family != s {
            self.family.clear();
            self.family.push_str(s);
            self.dirty = true;
        }
    }

    pub fn set_size(&mut self, v: f32) {
        if self.size != v { self.size = v; self.dirty = true; }
    }

    pub fn set_weight(&mut self, v: u16) {
        if self.weight != v { self.weight = v; self.dirty = true; }
    }

    pub fn set_italic(&mut self, v: bool) {
        if self.italic != v { self.italic = v; self.dirty = true; }
    }

    pub fn set_color(&mut self, c: [f32; 4]) {
        if self.color != c { self.color = c; self.dirty = true; }
    }

    pub fn set_width(&mut self, w: f32) {
        if self.width != w { self.width = w; self.dirty = true; }
    }

    pub fn set_clip(&mut self, clip: Option<[f32; 4]>) {
        if self.clip != clip { self.clip = clip; self.dirty = true; }
    }

    pub fn set_z(&mut self, z: i32) {
        if self.z != z { self.z = z; self.dirty = true; }
    }

    pub fn set_visible(&mut self, v: bool) {
        if self.visible != v { self.visible = v; self.dirty = true; }
    }
}

// ShadowNode 

/// a soft box shadow drawn beneath a rectangle
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

    pub(crate) dirty: bool,
    pub(crate) slot: u32,
}

impl ShadowNode {
    pub(crate) fn new() -> Self {
        Self {
            x: 0.0, y: 0.0, w: 0.0, h: 0.0,
            color: [0.0, 0.0, 0.0, 0.8],
            blur: 8.0,
            radius: 0.0,
            offset_x: 0.0,
            offset_y: 2.0,
            visible: false,
            z: 0,
            dirty: true,
            slot: u32::MAX,
        }
    }

    pub fn set_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        if self.x != x || self.y != y || self.w != w || self.h != h {
            self.x = x; self.y = y; self.w = w; self.h = h;
            self.dirty = true;
        }
    }

    pub fn set_color(&mut self, c: [f32; 4]) {
        if self.color != c { self.color = c; self.dirty = true; }
    }

    pub fn set_blur(&mut self, b: f32) {
        if self.blur != b { self.blur = b; self.dirty = true; }
    }

    pub fn set_radius(&mut self, r: f32) {
        if self.radius != r { self.radius = r; self.dirty = true; }
    }

    pub fn set_offset(&mut self, x: f32, y: f32) {
        if self.offset_x != x || self.offset_y != y {
            self.offset_x = x; self.offset_y = y;
            self.dirty = true;
        }
    }

    pub fn set_visible(&mut self, v: bool) {
        if self.visible != v { self.visible = v; self.dirty = true; }
    }

    pub fn set_z(&mut self, z: i32) {
        if self.z != z { self.z = z; self.dirty = true; }
    }
}
