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

/// A color applied to a character range. `[start, end, r, g, b, a]`
/// where start/end are char indices (not byte indices).
pub type ColorRange = [f32; 6]; // start, end as f32 for uniformity
pub type DecorationRange = [f32; 6]; // start, end, r, g, b, a

/// A weight applied to a character range.
pub struct WeightRange {
    pub start: usize,
    pub end: usize,
    pub weight: u16,
}

/// An italic range.
pub struct ItalicRange {
    pub start: usize,
    pub end: usize,
}

/// A font family range.
pub struct FontFamilyRange {
    pub start: usize,
    pub end: usize,
    pub font_family: String,
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
    pub max_width: Option<f32>,

    // visual-only ranges (no reshape needed)
    pub color_ranges: Vec<ColorRange>, // [start, end, r, g, b, a]
    pub background_ranges: Vec<DecorationRange>, // [start, end, r, g, b, a]
    pub underline_ranges: Vec<DecorationRange>, // [start, end, r, g, b, a]
    pub strikethrough_ranges: Vec<DecorationRange>, // [start, end, r, g, b, a]

    // shaping-relevant ranges (reshape needed)
    pub weight_ranges: Vec<WeightRange>,
    pub italic_ranges: Vec<ItalicRange>,
    pub font_family_ranges: Vec<FontFamilyRange>,

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
            max_width: None,

            color_ranges: Vec::new(),
            background_ranges: Vec::new(),
            underline_ranges: Vec::new(),
            strikethrough_ranges: Vec::new(),
            weight_ranges: Vec::new(),
            italic_ranges: Vec::new(),
            font_family_ranges: Vec::new(),
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
