use bento_shared::{
    ColorRange, DecorationRange, FontFamilyRange, ItalicRange, TextAlign, WeightRange,
};

pub struct RectDraw {
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
    pub opacity: f32,
    pub clip: Option<[f32; 4]>,
    pub z: i32,
}

pub struct TextDraw {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub text: String,
    pub size: f32,
    pub color: [f32; 4],
    pub weight: u16,
    pub italic: bool,
    pub font_family: String,
    pub max_width: Option<f32>,
    pub line_height: Option<f32>,
    pub letter_spacing: f32,
    pub align: TextAlign,
    pub opacity: f32,
    pub clip: Option<[f32; 4]>,
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub z: i32,
    pub color_ranges: Vec<ColorRange>,
    pub background_ranges: Vec<DecorationRange>,
    pub underline_ranges: Vec<DecorationRange>,
    pub strikethrough_ranges: Vec<DecorationRange>,
    pub weight_ranges: Vec<WeightRange>,
    pub italic_ranges: Vec<ItalicRange>,
    pub font_family_ranges: Vec<FontFamilyRange>,
}

pub struct ImageDraw {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub image_id: u64,
    pub radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub opacity: f32,
    pub clip: Option<[f32; 4]>,
    pub z: i32,
}

pub enum DrawCommand {
    Rect(RectDraw),
    Text(TextDraw),
    Image(ImageDraw),
}

impl DrawCommand {
    pub fn z(&self) -> i32 {
        match self {
            DrawCommand::Rect(r) => r.z,
            DrawCommand::Text(t) => t.z,
            DrawCommand::Image(i) => i.z,
        }
    }
}

pub struct DrawList {
    pub commands: Vec<DrawCommand>,
}

impl DrawList {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn push_rect(&mut self, rect: RectDraw) {
        self.commands.push(DrawCommand::Rect(rect));
    }
    pub fn push_text(&mut self, text: TextDraw) {
        self.commands.push(DrawCommand::Text(text));
    }
    pub fn push_image(&mut self, image: ImageDraw) {
        self.commands.push(DrawCommand::Image(image));
    }

    pub fn sort_by_z(&mut self) {
        self.commands.sort_by_key(|c| c.z());
    }
}
