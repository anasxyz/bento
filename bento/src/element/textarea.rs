use crate::color::Color;
use crate::element::base::Base;
use crate::element::element::{Element, EventResult};
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::element::scrollbar::Scrollbar;
use crate::fonts::Fonts;
use crate::input::{Key, Modifiers, MouseButton};
use crate::render::DrawCall;
use crate::ui::Ui;
use bento_derive::Element;

#[derive(Element)]
pub struct TextArea {
    base: Base,

    text: String,
    cursor: usize,
    cursor_x: f32,
    cursor_y: f32,

    pub bar: Scrollbar,

    pub font_size: f32,
    pub font_family: String,
    pub font_weight: u16,
    pub text_color: Color,
    pub placeholder: String,
    pub placeholder_color: Color,
    pub bg_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,
    pub border_widths: [f32; 4],
    pub padding: [f32; 4],
    pub readonly: bool,
}

impl TextArea {
    pub fn new() -> Self {
        let mut bar = Scrollbar::new();
        bar.vertical = true;
        bar.horizontal = false;
        Self {
            base: Base::new(),
            text: String::new(),
            cursor: 0,
            cursor_x: 0.0,
            cursor_y: 0.0,
            bar,
            font_size: 14.0,
            font_family: "sans-serif".to_string(),
            font_weight: 400,
            text_color: Color::WHITE,
            placeholder: String::new(),
            placeholder_color: Color::rgba(255, 255, 255, 80),
            bg_color: None,
            border_color: None,
            border_radius: None,
            border_widths: [0.0; 4],
            padding: [8.0, 8.0, 8.0, 8.0],
            readonly: false,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self.cursor = self.cursor.min(self.text.len());
        self.update_content_size();
        self.base.dirty = true;
        self
    }
    pub fn set_font_size(&mut self, v: f32) -> &mut Self {
        self.font_size = v;
        self.base.dirty = true;
        self
    }
    pub fn set_font_family(&mut self, v: &str) -> &mut Self {
        self.font_family = v.to_string();
        self.base.dirty = true;
        self
    }
    pub fn set_font_weight(&mut self, v: u16) -> &mut Self {
        self.font_weight = v;
        self.base.dirty = true;
        self
    }
    pub fn set_text_color(&mut self, v: Color) -> &mut Self {
        self.text_color = v;
        self.base.dirty = true;
        self
    }
    pub fn set_placeholder(&mut self, v: &str) -> &mut Self {
        self.placeholder = v.to_string();
        self.base.dirty = true;
        self
    }
    pub fn set_placeholder_color(&mut self, v: Color) -> &mut Self {
        self.placeholder_color = v;
        self.base.dirty = true;
        self
    }
    pub fn set_bg_color(&mut self, v: Option<Color>) -> &mut Self {
        self.bg_color = v;
        self.base.dirty = true;
        self
    }
    pub fn set_border_color(&mut self, v: Option<Color>) -> &mut Self {
        self.border_color = v;
        self.base.dirty = true;
        self
    }
    pub fn set_border_radius(&mut self, v: Option<f32>) -> &mut Self {
        self.border_radius = v;
        self.base.dirty = true;
        self
    }
    pub fn set_border(&mut self, v: [f32; 4]) -> &mut Self {
        self.border_widths = v;
        self.base.dirty = true;
        self
    }
    pub fn set_padding(&mut self, v: [f32; 4]) -> &mut Self {
        self.padding = v;
        self.base.dirty = true;
        self
    }
    pub fn set_readonly(&mut self, v: bool) -> &mut Self {
        self.readonly = v;
        self
    }

    fn line_height(&self) -> f32 {
        self.font_size * 1.4
    }
    fn lines(&self) -> Vec<&str> {
        self.text.split('\n').collect()
    }

    fn content_h(&self) -> f32 {
        let lines = self.lines().len().max(1);
        lines as f32 * self.line_height() + self.padding[0] + self.padding[2]
    }

    fn update_content_size(&mut self) {
        let l = &self.base.layout;
        self.bar.viewport_width = l.w;
        self.bar.viewport_height = l.h;
        self.bar.content_height = self.content_h();
        self.bar.content_width = l.w;
    }

    fn cursor_line_col(&self) -> (usize, usize) {
        let before = &self.text[..self.cursor];
        let line = before.chars().filter(|&c| c == '\n').count();
        let col = before
            .rfind('\n')
            .map(|i| self.cursor - i - 1)
            .unwrap_or(self.cursor);
        (line, col)
    }

    fn line_start(&self, line: usize) -> usize {
        self.text.split('\n').take(line).map(|l| l.len() + 1).sum()
    }

    fn clamp_cursor(&self, pos: usize) -> usize {
        let pos = pos.min(self.text.len());
        let mut p = pos;
        while p > 0 && !self.text.is_char_boundary(p) {
            p -= 1;
        }
        p
    }

    fn insert_char(&mut self, c: char) {
        self.text.insert(self.cursor, c);
        self.cursor += c.len_utf8();
        self.update_content_size();
        self.base.dirty = true;
    }

    fn delete_before(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let mut start = self.cursor - 1;
        while start > 0 && !self.text.is_char_boundary(start) {
            start -= 1;
        }
        self.text.remove(start);
        self.cursor = start;
        self.update_content_size();
        self.base.dirty = true;
    }

    fn delete_after(&mut self) {
        if self.cursor >= self.text.len() {
            return;
        }
        self.text.remove(self.cursor);
        self.update_content_size();
        self.base.dirty = true;
    }

    fn update_cursor_position(&mut self, ui: &mut Ui) {
        let l = &self.base.layout;
        let (cursor_line, cursor_col) = self.cursor_line_col();
        let lines = self.lines();
        let line_text = lines.get(cursor_line).copied().unwrap_or("");
        let text_before = &line_text[..cursor_col.min(line_text.len())];

        let mut fonts = ui.fonts.take().unwrap();
        let (w, _) = fonts.measure_sized(
            text_before,
            &self.font_family,
            self.font_size,
            self.font_weight,
            false,
            None,
        );
        ui.fonts = Some(fonts);

        self.cursor_x = l.x + self.padding[3] + w;
        self.cursor_y = l.y + self.padding[0] + cursor_line as f32 * self.line_height();
    }

    fn scroll_to_cursor(&mut self) {
        // cursor_y is absolute
        // convert to content relative
        let l = &self.base.layout;
        let content_y = self.cursor_y - l.y - self.padding[0];
        let viewport_h = l.h - self.padding[0] - self.padding[2];
        let cur = self.bar.scroll_y;

        let new_scroll = if content_y < cur {
            content_y
        } else if content_y + self.line_height() > cur + viewport_h {
            content_y + self.line_height() - viewport_h
        } else {
            cur
        };

        if new_scroll != cur {
            self.bar.set_scroll_y(new_scroll);
            self.base.dirty = true;
        }
    }

    fn rect(&self) -> (f32, f32, f32, f32) {
        let l = &self.base.layout;
        (l.x, l.y, l.w, l.h)
    }
}

impl Element for TextArea {
    fn draw_calls(&self, clip: Option<[f32; 4]>, z: i32, opacity: f32) -> Vec<DrawCall> {
        let l = &self.base.layout;
        let mut calls = Vec::new();

        // background
        if let Some(bg) = self.bg_color {
            let mut color = bg.to_array();
            color[3] *= opacity;
            let bc = self.border_color.unwrap_or(Color::BLACK).to_array();
            calls.push(DrawCall::Rect {
                x: l.x,
                y: l.y,
                w: l.w,
                h: l.h,
                color,
                radius: self.border_radius.unwrap_or(0.0),
                border_color: bc,
                border_widths: self.border_widths,
                clip,
                z_index: z,
            });
        }

        // clip to padding area
        let inner_clip = Some([
            l.x + self.padding[3],
            l.y + self.padding[0],
            l.x + l.w - self.padding[1],
            l.y + l.h - self.padding[2],
        ]);
        let text_clip = match (clip, inner_clip) {
            (Some([ax, ay, ax2, ay2]), Some([bx, by, bx2, by2])) => {
                Some([ax.max(bx), ay.max(by), ax2.min(bx2), ay2.min(by2)])
            }
            (None, c) | (c, None) => c,
        };

        let text_x = l.x + self.padding[3];
        let text_y = l.y + self.padding[0] - self.bar.scroll_y;

        let show_placeholder = self.text.is_empty() && !self.base.focused;
        let display_text = if show_placeholder {
            &self.placeholder
        } else {
            &self.text
        };
        let mut color = if show_placeholder {
            self.placeholder_color
        } else {
            self.text_color
        }
        .to_array();
        color[3] *= opacity;

        if !display_text.is_empty() {
            calls.push(DrawCall::Text {
                x: text_x,
                y: text_y,
                content: display_text.to_string(),
                family: self.font_family.clone(),
                size: self.font_size,
                weight: self.font_weight,
                italic: false,
                color,
                width: l.w - self.padding[1] - self.padding[3],
                clip: text_clip,
                z_index: z + 1,
            });
        }

        // cursor
        if self.base.focused {
            let cursor_draw_y = self.cursor_y - self.bar.scroll_y;
            calls.push(DrawCall::Rect {
                x: self.cursor_x,
                y: cursor_draw_y,
                w: 2.0,
                h: self.line_height(),
                color: [1.0, 1.0, 1.0, opacity],
                radius: 0.0,
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                clip: text_clip,
                z_index: z + 2,
            });
        }

        // scrollbar
        // drawn over content, unclipped
        calls.extend(self.bar.draw_calls(self.rect(), clip, z + 3, opacity));

        calls
    }

    fn on_focus_gained(&mut self, ui: &mut Ui, handle: Handle<()>) {
        self.base.focused = true;
        self.update_content_size();
        self.update_cursor_position(ui);
        self.base.dirty = true;
    }

    fn on_focus_lost(&mut self, _ui: &mut Ui, _handle: Handle<()>) {
        self.base.focused = false;
        self.base.dirty = true;
    }

    fn on_mouse_press(
        &mut self,
        _ui: &mut Ui,
        _handle: Handle<()>,
        x: f32,
        y: f32,
        button: MouseButton,
    ) -> EventResult {
        if button != MouseButton::Left {
            return EventResult::Propagate;
        }
        if self.bar.on_mouse_press(self.rect(), x, y) {
            self.base.dirty = true;
            EventResult::Handled
        } else {
            EventResult::Propagate
        }
    }

    fn on_mouse_release(
        &mut self,
        _ui: &mut Ui,
        _handle: Handle<()>,
        _x: f32,
        _y: f32,
        _button: MouseButton,
    ) -> EventResult {
        if self.bar.on_mouse_release() {
            self.base.dirty = true;
            EventResult::Handled
        } else {
            EventResult::Propagate
        }
    }

    fn on_mouse_move(&mut self, _ui: &mut Ui, _handle: Handle<()>, x: f32, y: f32) -> EventResult {
        if self.bar.on_mouse_move(self.rect(), x, y) {
            self.base.dirty = true;
            EventResult::Handled
        } else {
            EventResult::Propagate
        }
    }

    fn on_mouse_scroll(
        &mut self,
        _ui: &mut Ui,
        _handle: Handle<()>,
        delta_x: f32,
        delta_y: f32,
    ) -> EventResult {
        if self.bar.on_scroll(delta_x, delta_y) {
            self.base.dirty = true;
            EventResult::Handled
        } else {
            EventResult::Propagate
        }
    }

    fn on_key_press(
        &mut self,
        ui: &mut Ui,
        handle: Handle<()>,
        key: Key,
        mods: Modifiers,
        text: Option<char>,
    ) -> EventResult {
        if self.readonly {
            return EventResult::Propagate;
        }

        match key {
            Key::Backspace => {
                self.delete_before();
            }
            Key::Delete => {
                self.delete_after();
            }
            Key::Enter => {
                self.insert_char('\n');
            }
            Key::Left => {
                if self.cursor > 0 {
                    let mut pos = self.cursor - 1;
                    while pos > 0 && !self.text.is_char_boundary(pos) {
                        pos -= 1;
                    }
                    self.cursor = pos;
                    self.base.dirty = true;
                }
            }
            Key::Right => {
                if self.cursor < self.text.len() {
                    let mut pos = self.cursor + 1;
                    while pos < self.text.len() && !self.text.is_char_boundary(pos) {
                        pos += 1;
                    }
                    self.cursor = pos;
                    self.base.dirty = true;
                }
            }
            Key::Up => {
                let (line, col) = self.cursor_line_col();
                if line > 0 {
                    let prev_start = self.line_start(line - 1);
                    let prev_len = self.lines()[line - 1].len();
                    self.cursor = self.clamp_cursor(prev_start + col.min(prev_len));
                    self.base.dirty = true;
                }
            }
            Key::Down => {
                let (line, col) = self.cursor_line_col();
                let lines = self.lines();
                if line + 1 < lines.len() {
                    let next_start = self.line_start(line + 1);
                    let next_len = lines[line + 1].len();
                    self.cursor = self.clamp_cursor(next_start + col.min(next_len));
                    self.base.dirty = true;
                }
            }
            Key::Home => {
                let (line, _) = self.cursor_line_col();
                self.cursor = self.line_start(line);
                self.base.dirty = true;
            }
            Key::End => {
                let (line, _) = self.cursor_line_col();
                let line_len = self.lines().get(line).map(|l| l.len()).unwrap_or(0);
                self.cursor = self.clamp_cursor(self.line_start(line) + line_len);
                self.base.dirty = true;
            }
            _ => {
                if let Some(c) = text {
                    if !mods.ctrl && !mods.super_key && !c.is_control() {
                        self.insert_char(c);
                    } else {
                        return EventResult::Propagate;
                    }
                } else {
                    return EventResult::Propagate;
                }
            }
        }

        self.update_cursor_position(ui);
        self.scroll_to_cursor();
        EventResult::Handled
    }
}
