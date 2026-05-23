use crate::acc::Accumulated;
use crate::ui::Ui;
use crate::widget::primitive::{Rect, Text};
use crate::widget::{Widget, WidgetHandle};
use bento_shared::{TextAlign, TextMeasureRequest, TextMeasurer};
use bento_wgpu::{DrawList, RectDraw, TextDraw};

pub struct Button {
    id: usize,
    pub label_text: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub z: i32,
    pub padding: f32,
    dirty: bool,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            id: 0,
            label_text: text.to_string(),
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            color: [0.2, 0.2, 0.2, 1.0],
            z: 0,
            padding: 16.0,
            dirty: true,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        if self.label_text == text {
            return;
        }
        self.label_text = text.to_string();
        self.dirty = true;
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
    pub fn set_w(&mut self, w: f32) {
        if self.w == w {
            return;
        }
        self.w = w;
        self.dirty = true;
    }
    pub fn set_h(&mut self, h: f32) {
        if self.h == h {
            return;
        }
        self.h = h;
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

impl Widget for Button {
    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn name(&self) -> &str {
        "Button"
    }

    fn update(&mut self, ui: &mut Ui) {
        let result = ui.measurer.measure(TextMeasureRequest {
            text: &self.label_text,
            font_family: "",
            size: 14.0,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            max_width: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        self.w = result.width + self.padding * 2.0;
        self.h = result.height + self.padding * 2.0;
    }

    fn render(&self, draw_list: &mut DrawList, acc: &Accumulated) {
        let x = acc.offset_x;
        let y = acc.offset_y;
        let lw = self.w - self.padding * 2.0;
        let lh = self.h - self.padding * 2.0;

        draw_list.push_rect(RectDraw {
            x,
            y,
            w: self.w,
            h: self.h,
            color: self.color,
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            rotate: acc.rotate,
            scale_x: acc.scale_x,
            scale_y: acc.scale_y,
            opacity: acc.opacity,
            clip: acc.clip,
            z: self.z,
        });

        draw_list.push_text(TextDraw {
            x: x + (self.w - lw) / 2.0,
            y: y + (self.h - lh) / 2.0,
            w: lw,
            h: lh,
            text: self.label_text.clone(),
            size: 14.0,
            color: [1.0, 1.0, 1.0, 1.0],
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
            z: self.z + 1,
            color_ranges: vec![],
            background_ranges: vec![],
            underline_ranges: vec![],
            strikethrough_ranges: vec![],
            weight_ranges: vec![],
            italic_ranges: vec![],
            font_family_ranges: vec![],
        });
    }

    fn hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
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
