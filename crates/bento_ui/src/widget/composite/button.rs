use bento_shared::{TextMeasureRequest, TextMeasurer};

use crate::ui::Ui;
use crate::widget::primitive::{Rect, Text};
use crate::widget::{Widget, WidgetHandle};

pub struct Button {
    id: usize,
    bg: WidgetHandle<Rect>,
    label: WidgetHandle<Text>,

    pub label_text: String,

    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub padding: f32,

    dirty: bool,
}

impl Button {
    pub fn new() -> Self {
        Self {
            id: 0,
            bg: WidgetHandle::from_id(0),
            label: WidgetHandle::from_id(0),

            label_text: "Hello world".to_string(),

            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            color: [0.0, 0.0, 0.0, 1.0],
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

    fn build(&mut self, ui: &mut Ui) {
        self.bg = ui.add_child(self, Rect::new());
        self.label = ui.add_child(self, Text::new(&self.label_text));
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
        let lw = result.width;
        let lh = result.height;

        // auto size if width or height are not explicitly set
        let w = if self.w == 0.0 {
            lw + self.padding * 2.0
        } else {
            self.w
        };
        let h = if self.h == 0.0 {
            lh + self.padding * 2.0
        } else {
            self.h
        };

        if let Some(bg) = ui.get_mut(self.bg) {
            bg.set_x(self.x);
            bg.set_y(self.y);
            bg.set_w(w);
            bg.set_h(h);
            bg.set_color(self.color);
        }
        if let Some(label) = ui.get_mut(self.label) {
            label.set_content(&self.label_text);
            label.set_x(self.x + (w - lw) / 2.0);
            label.set_y(self.y + (h - lh) / 2.0);
        }
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
}
