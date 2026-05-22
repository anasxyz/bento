use crate::{Ui, acc::Accumulated, widget::Widget};
use bento_shared::{SceneNode, SceneNodeId, TextAlign, TextMeasureRequest, TextMeasurer, TextNode};
use bento_wgpu::{DrawList, RectDraw, TextDraw};

pub struct Text {
    id: usize,

    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub content: String,
    pub size: f32,
    pub color: [f32; 4],
    pub z: i32,

    dirty: bool,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self {
            id: 0,

            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            content: content.to_string(),
            size: 14.0,
            color: [1.0, 1.0, 1.0, 1.0],
            z: 0,

            dirty: true,
        }
    }

    pub fn set_x(&mut self, x: f32) {
        if self.x == x {
            return;
        }
        self.x = x;
        self.dirty = true;
    }
    pub fn set_y(&mut self, y: f32) {
        if self.y == y {
            return;
        }
        self.y = y;
        self.dirty = true;
    }
    pub fn set_content(&mut self, content: &str) {
        if self.content == content {
            return;
        }
        self.content = content.to_string();
        self.dirty = true;
    }
    pub fn set_size(&mut self, size: f32) {
        if self.size == size {
            return;
        }
        self.size = size;
        self.dirty = true;
    }
    pub fn set_color(&mut self, color: [f32; 4]) {
        if self.color == color {
            return;
        }
        self.color = color;
        self.dirty = true;
    }
    pub fn set_z(&mut self, z: i32) {
        if self.z == z {
            return;
        }
        self.z = z;
        self.dirty = true;
    }
}

impl Widget for Text {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn name(&self) -> &str {
        "Text"
    }

    fn build(&mut self, ui: &mut Ui) {}

    fn update(&mut self, ui: &mut Ui) {}

    fn remove(&mut self, ui: &mut Ui) {}

    fn hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn render(&self, draw_list: &mut DrawList, acc: &Accumulated) {
        draw_list.push_text(
            self.id as u64,
            TextDraw {
                x: acc.offset_x,
                y: acc.offset_y,
                w: self.w,
                h: self.h,
                text: self.content.clone(),
                size: self.size,
                color: self.color,
                weight: 400,
                italic: false,
                font_family: String::new(),
                max_width: None,
                line_height: None,
                letter_spacing: 0.0,
                align: TextAlign::Left,
                opacity: acc.opacity,
                clip: acc.clip,
                rotate: acc.rotate,
                scale_x: acc.scale_x,
                scale_y: acc.scale_y,
                z: acc.z,
                color_ranges: Vec::new(),
                background_ranges: Vec::new(),
                underline_ranges: Vec::new(),
                strikethrough_ranges: Vec::new(),
                weight_ranges: Vec::new(),
                italic_ranges: Vec::new(),
                font_family_ranges: Vec::new(),
            },
        );
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.set_x(x);
        self.set_y(y);
    }

    fn render_offset(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn z(&self) -> i32 {
        self.z
    }
}
