pub struct RectNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub z: i32,
    pub(crate) slot: u32,
}

impl RectNode {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            x,
            y,
            w,
            h,
            color: [1.0, 1.0, 1.0, 1.0],
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            z: 1,
            slot: u32::MAX,
        }
    }
}

pub struct ColorSpan {
    pub start: usize,
    pub end: usize,
    pub color: [f32; 4],
}

pub struct TextNode {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: [f32; 4],
    pub z: i32,
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub weight: u16,
    pub italic: bool,
    pub font_family: String,
    pub color_spans: Vec<ColorSpan>,
    pub max_width: Option<f32>,  
    pub(crate) slot: usize,
}

impl TextNode {
    pub fn new(text: &str, x: f32, y: f32, size: f32) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
            size,
            color: [1.0, 1.0, 1.0, 1.0],
            z: 1,
            slot: usize::MAX,
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            weight: 400,
            italic: false,
            font_family: String::new(),
            color_spans: Vec::new(),
            max_width: None,
        }
    }
}

pub enum Node {
    Rect(RectNode),
    Text(TextNode),
}

pub struct Scene {
    pub nodes: Vec<Node>,
}

impl Scene {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_rect(&mut self, rect: RectNode) {
        self.nodes.push(Node::Rect(rect));
    }

    pub fn add_text(&mut self, text: TextNode) {
        self.nodes.push(Node::Text(text));
    }
}
