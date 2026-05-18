use std::any::Any;

use bento_shared::scene::{ColorRange, DecorationRange, FontFamilyRange, ItalicRange, WeightRange};
use bento_shared::{Scene, SceneNode, SceneNodeId, TextAlign, TextNode};
use bento_shared::{TextMeasureRequest, TextMeasurer};

use crate::widget::{AsAny, Widget};
use crate::ui::Ui;

pub struct Text {
    pub dirty: bool,

    text: String,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    size: f32,
    color: [f32; 4],
    z: i32,
    rotate: f32,
    scale_x: f32,
    scale_y: f32,
    weight: u16,
    italic: bool,
    font_family: String,
    max_width: Option<f32>,
    line_height: Option<f32>,
    align: TextAlign,
    letter_spacing: f32,
    opacity: f32,
    clip: Option<[f32; 4]>,
    color_ranges: Vec<ColorRange>,
    background_ranges: Vec<DecorationRange>,
    underline_ranges: Vec<DecorationRange>,
    strikethrough_ranges: Vec<DecorationRange>,
    weight_ranges: Vec<WeightRange>,
    italic_ranges: Vec<ItalicRange>,
    font_family_ranges: Vec<FontFamilyRange>,

    focusable: bool,
    focused: bool,
    hoverable: bool,
    hovered: bool,

    text_id: Option<SceneNodeId>,
}

impl Text {
    pub fn new(text: &str, x: f32, y: f32, size: f32) -> Self {
        Self {
            dirty: true,
            text: text.to_string(),
            x,
            y,
            w: 0.0,
            h: 0.0,
            size,
            color: [1.0, 1.0, 1.0, 1.0],
            z: 1,
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            weight: 400,
            italic: false,
            font_family: String::new(),
            max_width: None,
            line_height: None,
            align: TextAlign::Left,
            letter_spacing: 0.0,
            opacity: 1.0,
            clip: None,
            color_ranges: Vec::new(),
            background_ranges: Vec::new(),
            underline_ranges: Vec::new(),
            strikethrough_ranges: Vec::new(),
            weight_ranges: Vec::new(),
            italic_ranges: Vec::new(),
            font_family_ranges: Vec::new(),
            focusable: true,
            focused: false,
            hoverable: true,
            hovered: false,
            text_id: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.dirty = true;
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
        self.dirty = true;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
        self.dirty = true;
    }

    pub fn set_size(&mut self, size: f32) {
        self.size = size;
        self.dirty = true;
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
        self.dirty = true;
    }

    pub fn set_z(&mut self, z: i32) {
        self.z = z;
        self.dirty = true;
    }

    pub fn set_rotate(&mut self, angle: f32) {
        self.rotate = angle;
        self.dirty = true;
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.scale_x = x;
        self.scale_y = y;
        self.dirty = true;
    }

    pub fn set_weight(&mut self, weight: u16) {
        self.weight = weight;
        self.dirty = true;
    }

    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
        self.dirty = true;
    }

    pub fn set_font_family(&mut self, family: &str) {
        self.font_family = family.to_string();
        self.dirty = true;
    }

    pub fn set_max_width(&mut self, width: f32) {
        self.max_width = Some(width);
        self.dirty = true;
    }

    pub fn set_line_height(&mut self, height: f32) {
        self.line_height = Some(height);
        self.dirty = true;
    }

    pub fn set_align(&mut self, align: TextAlign) {
        self.align = align;
        self.dirty = true;
    }

    pub fn set_letter_spacing(&mut self, spacing: f32) {
        self.letter_spacing = spacing;
        self.dirty = true;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
        self.dirty = true;
    }

    pub fn set_clip(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.clip = Some([x, y, w, h]);
        self.dirty = true;
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

    pub fn set_focusable(&mut self, focusable: bool) {
        self.focusable = focusable;
    }

    pub fn set_hoverable(&mut self, hoverable: bool) {
        self.hoverable = hoverable;
    }
}

impl Widget for Text {
}

impl AsAny for Text {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
