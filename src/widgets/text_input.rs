use std::any::Any;
use crate::{
    FontId, MouseState, Drawer,
    widgets::{Rect, Widget},
};

pub struct TextInputWidget {
    pub(crate) id: usize,
    pub(crate) bounds: Rect,
    pub(crate) font: Option<FontId>,
    pub(crate) bg_color: [f32; 4],
    pub(crate) focused_color: [f32; 4],
    pub(crate) outline_color: [f32; 4],
    pub(crate) focused_outline_color: [f32; 4],
    pub(crate) outline_thickness: f32,
    pub(crate) placeholder: String,
    pub(crate) placeholder_color: [f32; 4],
    pub(crate) text_color: [f32; 4],
    pub(crate) corner_radius: f32,

    pub value: String,
    pub focused: bool,
    pub just_focused: bool,
    pub just_unfocused: bool,
    pub just_changed: bool,
    pub cursor: usize,

    blink_timer: f32,
    cursor_visible: bool,

    dirty: bool,
}

impl TextInputWidget {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            bounds: Rect { x: 0.0, y: 0.0, w: 200.0, h: 32.0 },
            font: None,
            bg_color: [0.12, 0.13, 0.16, 1.0],
            focused_color: [0.14, 0.16, 0.20, 1.0],
            outline_color: [0.25, 0.28, 0.35, 1.0],
            focused_outline_color: [0.3, 0.55, 1.0, 1.0],
            outline_thickness: 1.5,
            placeholder: String::new(),
            placeholder_color: [0.4, 0.43, 0.52, 1.0],
            text_color: [0.88, 0.90, 0.95, 1.0],
            corner_radius: 5.0,
            value: String::new(),
            focused: false,
            just_focused: false,
            just_unfocused: false,
            just_changed: false,
            cursor: 0,
            blink_timer: 0.0,
            cursor_visible: true,
            dirty: true,
        }
    }

    pub fn position(&mut self, x: f32, y: f32) -> &mut Self {
        self.bounds.x = x;
        self.bounds.y = y;
        self
    }

    pub fn size(&mut self, w: f32, h: f32) -> &mut Self {
        self.bounds.w = w;
        self.bounds.h = h;
        self
    }

    pub fn font(&mut self, font_id: FontId) -> &mut Self {
        self.font = Some(font_id);
        self
    }

    pub fn placeholder(&mut self, text: impl Into<String>) -> &mut Self {
        self.placeholder = text.into();
        self
    }

    pub fn bg_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.bg_color = color;
        self
    }

    pub fn focused_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.focused_color = color;
        self
    }

    pub fn outline_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.outline_color = color;
        self
    }

    pub fn focused_outline_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.focused_outline_color = color;
        self
    }

    pub fn text_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.text_color = color;
        self
    }

    pub fn corner_radius(&mut self, r: f32) -> &mut Self {
        self.corner_radius = r;
        self
    }

    pub fn value(&mut self, text: impl Into<String>) -> &mut Self {
        self.value = text.into();
        let len = self.value.len();
        self.cursor = len;
        self.dirty = true;
        self
    }

    pub(crate) fn insert_char(&mut self, c: char) {
        self.value.insert(self.cursor, c);
        self.cursor += c.len_utf8();
        self.just_changed = true;
        self.dirty = true;
    }

    pub(crate) fn backspace(&mut self) {
        if self.cursor == 0 { return; }
        let mut new_cursor = self.cursor - 1;
        while !self.value.is_char_boundary(new_cursor) {
            new_cursor -= 1;
        }
        self.value.remove(new_cursor);
        self.cursor = new_cursor;
        self.just_changed = true;
        self.dirty = true;
    }

    pub(crate) fn delete_forward(&mut self) {
        if self.cursor >= self.value.len() { return; }
        self.value.remove(self.cursor);
        self.just_changed = true;
        self.dirty = true;
    }

    pub(crate) fn move_cursor_left(&mut self) {
        if self.cursor == 0 { return; }
        let mut new = self.cursor - 1;
        while !self.value.is_char_boundary(new) { new -= 1; }
        self.cursor = new;
        self.dirty = true;
    }

    pub(crate) fn move_cursor_right(&mut self) {
        if self.cursor >= self.value.len() { return; }
        let mut new = self.cursor + 1;
        while !self.value.is_char_boundary(new) { new += 1; }
        self.cursor = new;
        self.dirty = true;
    }

    pub(crate) fn move_cursor_home(&mut self) {
        self.cursor = 0;
        self.dirty = true;
    }

    pub(crate) fn move_cursor_end(&mut self) {
        self.cursor = self.value.len();
        self.dirty = true;
    }

    pub fn tick_blink(&mut self, dt: f32) -> bool {
        if !self.focused { return false; }
        self.blink_timer += dt;
        if self.blink_timer >= 0.53 {
            self.blink_timer = 0.0;
            self.cursor_visible = !self.cursor_visible;
            self.dirty = true;
            return true;
        }
        false
    }
}

impl Widget for TextInputWidget {
    fn id(&self) -> usize { self.id }
    fn bounds(&self) -> Rect { self.bounds }
    fn set_bounds(&mut self, bounds: Rect) {
        if (bounds.x - self.bounds.x).abs() > 0.001
            || (bounds.y - self.bounds.y).abs() > 0.001
            || (bounds.w - self.bounds.w).abs() > 0.001
            || (bounds.h - self.bounds.h).abs() > 0.001
        {
            self.bounds = bounds;
            self.dirty = true;
        }
    }

    fn update(&mut self, mouse: &MouseState) {
        self.just_focused   = false;
        self.just_unfocused = false;
        self.just_changed   = false;

        if mouse.left_just_pressed {
            let clicked_inside = self.bounds.contains(mouse.x, mouse.y);
            if clicked_inside && !self.focused {
                self.focused = true;
                self.just_focused = true;
                self.cursor_visible = true;
                self.blink_timer = 0.0;
                self.dirty = true;
            } else if !clicked_inside && self.focused {
                self.focused = false;
                self.just_unfocused = true;
                self.dirty = true;
            }
        }
    }

    fn is_dirty(&self) -> bool { self.dirty }
    fn clear_dirty(&mut self) { self.dirty = false; }

    fn render(&mut self, drawer: &mut Drawer) {
        let font_id = self.font.expect(
            "TextInputWidget has no font, call .font(font_id) before rendering"
        );

        let x = self.bounds.x;
        let y = self.bounds.y;
        let w = self.bounds.w;
        let h = self.bounds.h;
        let r = self.corner_radius;
        let padding = 8.0f32;

        let bg = if self.focused { self.focused_color } else { self.bg_color };
        let outline = if self.focused { self.focused_outline_color } else { self.outline_color };
        drawer.rounded_rect(x, y, w, h, r, bg, outline, self.outline_thickness);

        let text_y_center = {
            let (_, text_h) = drawer.fonts.measure("A", font_id);
            y + (h - text_h) / 2.0
        };

        if self.value.is_empty() && !self.placeholder.is_empty() {
            let color = self.placeholder_color;
            let _ = color; 
            drawer.text(&self.placeholder, font_id, x + padding, text_y_center);
        } else {
            drawer.text(&self.value, font_id, x + padding, text_y_center);
        }

        if self.focused && self.cursor_visible {
            let text_before_cursor = &self.value[..self.cursor];
            let (cursor_x_offset, text_h) = if text_before_cursor.is_empty() {
                let (_, h) = drawer.fonts.measure("A", font_id);
                (0.0f32, h)
            } else {
                drawer.fonts.measure(text_before_cursor, font_id)
            };

            let cursor_x = x + padding + cursor_x_offset;
            let cursor_top = y + (h - text_h) / 2.0;
            let cursor_h = text_h;
            let cursor_w = 1.5f32;

            drawer.rect(cursor_x, cursor_top, cursor_w, cursor_h, [1.0, 1.0, 1.0, 0.8], [0.0; 4], 0.0);
        }
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
