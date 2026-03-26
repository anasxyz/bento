#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RectId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShadowId(pub(crate) usize);

pub struct RectNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub radius: f32,
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub clip: Option<[f32; 4]>,
    pub z: i32,
    pub visible: bool,
    pub(crate) slot: u32,
}

impl RectNode {
    pub(crate) fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            color: [0.0; 4],
            radius: 0.0,
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            clip: None,
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
    pub fn set_clip(&mut self, clip: Option<[f32; 4]>) {
        self.clip = clip;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }
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
    pub clip: Option<[f32; 4]>,
    pub z: i32,
    pub visible: bool,
}

impl TextNode {
    pub(crate) fn new() -> Self {
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
            clip: None,
            z: 0,
            visible: false,
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
    pub fn set_clip(&mut self, clip: Option<[f32; 4]>) {
        self.clip = clip;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
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
    pub(crate) fn new() -> Self {
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
