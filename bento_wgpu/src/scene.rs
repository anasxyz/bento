pub struct RectNode {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
    pub(crate) color: [f32; 4],
    pub(crate) radii: [f32; 4],
    pub(crate) border_color: [f32; 4],
    pub(crate) border_widths: [f32; 4],
    pub(crate) rotate: f32,
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
    pub(crate) z: i32,
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

    pub fn x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }
    pub fn y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }
    pub fn w(&mut self, w: f32) -> &mut Self {
        self.w = w;
        self
    }
    pub fn h(&mut self, h: f32) -> &mut Self {
        self.h = h;
        self
    }

    pub fn pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn size(&mut self, w: f32, h: f32) -> &mut Self {
        self.w = w;
        self.h = h;
        self
    }

    pub fn color(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = color;
        self
    }
    pub fn radii(&mut self, radii: [f32; 4]) -> &mut Self {
        self.radii = radii;
        self
    }
    pub fn radius(&mut self, r: f32) -> &mut Self {
        self.radii = [r; 4];
        self
    }

    pub fn border(&mut self, color: [f32; 4], widths: [f32; 4]) -> &mut Self {
        self.border_color = color;
        self.border_widths = widths;
        self
    }
    pub fn border_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.border_color = color;
        self
    }
    pub fn border_widths(&mut self, widths: [f32; 4]) -> &mut Self {
        self.border_widths = widths;
        self
    }
    pub fn border_width(&mut self, w: f32) -> &mut Self {
        self.border_widths = [w; 4];
        self
    }

    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        self.rotate = angle;
        self
    }
    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.scale_x = x;
        self.scale_y = y;
        self
    }
    pub fn scale_x(&mut self, x: f32) -> &mut Self {
        self.scale_x = x;
        self
    }
    pub fn scale_y(&mut self, y: f32) -> &mut Self {
        self.scale_y = y;
        self
    }

    pub fn z(&mut self, z: i32) -> &mut Self {
        self.z = z;
        self
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct ColorRange {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) color: [f32; 4],
}

#[derive(Clone, PartialEq)]
pub(crate) struct DecorationRange {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) color: [f32; 4],
}

#[derive(Clone)]
pub(crate) struct WeightRange {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) weight: u16,
}

#[derive(Clone)]
pub(crate) struct ItalicRange {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Clone)]
pub(crate) struct FontFamilyRange {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) font_family: String,
}

pub struct TextNode {
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) size: f32,
    pub(crate) color: [f32; 4],
    pub(crate) z: i32,
    pub(crate) rotate: f32,
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
    pub(crate) weight: u16,
    pub(crate) italic: bool,
    pub(crate) font_family: String,
    pub(crate) max_width: Option<f32>,

    pub(crate) color_ranges: Vec<ColorRange>,
    pub(crate) background_ranges: Vec<DecorationRange>,
    pub(crate) underline_ranges: Vec<DecorationRange>,
    pub(crate) strikethrough_ranges: Vec<DecorationRange>,
    pub(crate) weight_ranges: Vec<WeightRange>,
    pub(crate) italic_ranges: Vec<ItalicRange>,
    pub(crate) font_family_ranges: Vec<FontFamilyRange>,

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

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self
    }
    pub fn x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }
    pub fn y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }
    pub fn pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn size(&mut self, size: f32) -> &mut Self {
        self.size = size;
        self
    }
    pub fn color(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = color;
        self
    }
    pub fn z(&mut self, z: i32) -> &mut Self {
        self.z = z;
        self
    }
    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        self.rotate = angle;
        self
    }
    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.scale_x = x;
        self.scale_y = y;
        self
    }
    pub fn scale_x(&mut self, x: f32) -> &mut Self {
        self.scale_x = x;
        self
    }
    pub fn scale_y(&mut self, y: f32) -> &mut Self {
        self.scale_y = y;
        self
    }
    pub fn weight(&mut self, weight: u16) -> &mut Self {
        self.weight = weight;
        self
    }
    pub fn italic(&mut self, italic: bool) -> &mut Self {
        self.italic = italic;
        self
    }
    pub fn font_family(&mut self, family: &str) -> &mut Self {
        self.font_family = family.to_string();
        self
    }
    pub fn max_width(&mut self, width: f32) -> &mut Self {
        self.max_width = Some(width);
        self
    }
    pub fn no_max_width(&mut self) -> &mut Self {
        self.max_width = None;
        self
    }

    pub fn add_color(&mut self, start: usize, end: usize, color: [f32; 4]) -> &mut Self {
        self.color_ranges.push(ColorRange { start, end, color });
        self
    }

    pub fn add_background(&mut self, start: usize, end: usize, color: [f32; 4]) -> &mut Self {
        self.background_ranges
            .push(DecorationRange { start, end, color });
        self
    }

    pub fn add_underline(&mut self, start: usize, end: usize, color: [f32; 4]) -> &mut Self {
        self.underline_ranges
            .push(DecorationRange { start, end, color });
        self
    }

    pub fn add_strikethrough(&mut self, start: usize, end: usize, color: [f32; 4]) -> &mut Self {
        self.strikethrough_ranges
            .push(DecorationRange { start, end, color });
        self
    }

    pub fn add_weight(&mut self, start: usize, end: usize, weight: u16) -> &mut Self {
        self.weight_ranges.push(WeightRange { start, end, weight });
        self
    }

    pub fn add_italic(&mut self, start: usize, end: usize) -> &mut Self {
        self.italic_ranges.push(ItalicRange { start, end });
        self
    }

    pub fn add_font_family(&mut self, start: usize, end: usize, family: &str) -> &mut Self {
        self.font_family_ranges.push(FontFamilyRange {
            start,
            end,
            font_family: family.to_string(),
        });
        self
    }

    pub fn clear_colors(&mut self) -> &mut Self {
        self.color_ranges.clear();
        self
    }
    pub fn clear_backgrounds(&mut self) -> &mut Self {
        self.background_ranges.clear();
        self
    }
    pub fn clear_underlines(&mut self) -> &mut Self {
        self.underline_ranges.clear();
        self
    }
    pub fn clear_strikethroughs(&mut self) -> &mut Self {
        self.strikethrough_ranges.clear();
        self
    }
    pub fn clear_weights(&mut self) -> &mut Self {
        self.weight_ranges.clear();
        self
    }
    pub fn clear_italics(&mut self) -> &mut Self {
        self.italic_ranges.clear();
        self
    }
    pub fn clear_font_families(&mut self) -> &mut Self {
        self.font_family_ranges.clear();
        self
    }

    pub fn clear_all_ranges(&mut self) -> &mut Self {
        self.color_ranges.clear();
        self.background_ranges.clear();
        self.underline_ranges.clear();
        self.strikethrough_ranges.clear();
        self.weight_ranges.clear();
        self.italic_ranges.clear();
        self.font_family_ranges.clear();
        self
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
