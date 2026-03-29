use crate::Cursor;
use crate::color::Color;
use crate::fonts::{FontAttrs, Fonts};
use crate::input::{Key, Modifiers, MouseButton};
use crate::widget::{AsAny, Base, HasBase, Widget};
use bento_derive::Widget;
use bento_wgpu::{ClipId, RectId, SceneGraph, SceneNodeId, TextId, TransformId};

#[derive(Widget)]
pub struct TextInput {
    pub base: Base,
    pub value: String,
    pub value_dirty: bool,

    // style
    background_color: Color,
    text_color: Color,
    placeholder_color: Color,
    cursor_color: Color,
    selection_color: Color,
    border_color: Color,
    border_width: f32,
    radius: f32,
    padding_x: f32,
    font_family: String,
    font_size: f32,
    font_weight: u16,
    placeholder: String,

    // state
    pub cursor_pos: usize,
    pub cursor_offset_x: f32,
    scroll_x: f32,
    cursor_visible: bool,
    selecting: bool,
    selection_anchor: usize,
    selection_start: usize,
    selection_end: usize,
    click_count: u32,
    last_click_x: f32,

    // computed char x offsets for click to cursor (set by update.rs)
    pub char_offsets: Vec<(usize, f32)>, // (byte_index, x_offset)

    // computed in sync
    text_x: f32,
    text_y: f32,
    width: f32,
    height: f32,

    // scene nodes
    bg_id: Option<RectId>,
    selection_id: Option<RectId>,
    clip_id: Option<ClipId>,
    transform_id: Option<TransformId>,
    text_id: Option<TextId>,
    cursor_id: Option<RectId>,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            value: String::new(),
            value_dirty: true,
            background_color: Color::rgba(10, 10, 10, 0),
            text_color: Color::WHITE,
            placeholder_color: Color::rgba(255, 255, 255, 80),
            cursor_color: Color::WHITE,
            selection_color: Color::rgba(68, 152, 227, 80),
            border_color: Color::rgb(80, 80, 80),
            border_width: 1.0,
            radius: 4.0,
            padding_x: 6.0,
            font_family: "sans-serif".to_string(),
            font_size: 14.0,
            font_weight: 400,
            placeholder: String::new(),
            cursor_pos: 0,
            cursor_offset_x: 0.0,
            scroll_x: 0.0,
            cursor_visible: true,
            selecting: false,
            selection_anchor: 0,
            selection_start: 0,
            selection_end: 0,
            click_count: 0,
            last_click_x: 0.0,
            char_offsets: Vec::new(),
            text_x: 0.0,
            text_y: 0.0,
            width: 0.0,
            height: 0.0,
            bg_id: None,
            selection_id: None,
            clip_id: None,
            transform_id: None,
            text_id: None,
            cursor_id: None,
        }
    }

    pub fn set_value(&mut self, s: &str) -> &mut Self {
        self.value = s.to_string();
        self.cursor_pos = self.value.len();
        self.selection_start = 0;
        self.selection_end = 0;
        self.base.render_dirty = true;
        self.value_dirty = true;
        self
    }
    pub fn set_placeholder(&mut self, s: &str) -> &mut Self {
        self.placeholder = s.to_string();
        self.base.render_dirty = true;
        self
    }
    pub fn set_placeholder_color(&mut self, c: Color) -> &mut Self {
        self.placeholder_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_background_color(&mut self, c: Color) -> &mut Self {
        self.background_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_text_color(&mut self, c: Color) -> &mut Self {
        self.text_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_cursor_color(&mut self, c: Color) -> &mut Self {
        self.cursor_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_selection_color(&mut self, c: Color) -> &mut Self {
        self.selection_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_border_color(&mut self, c: Color) -> &mut Self {
        self.border_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_border_width(&mut self, v: f32) -> &mut Self {
        self.border_width = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_border_radius(&mut self, v: f32) -> &mut Self {
        self.radius = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_padding_x(&mut self, v: f32) -> &mut Self {
        self.padding_x = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_font_family(&mut self, s: &str) -> &mut Self {
        self.font_family = s.to_string();
        self.base.render_dirty = true;
        self
    }
    pub fn set_font_size(&mut self, v: f32) -> &mut Self {
        self.font_size = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_font_weight(&mut self, v: u16) -> &mut Self {
        self.font_weight = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_cursor(&mut self, c: Cursor) -> &mut Self {
        self.base.cursor = c;
        self.base.render_dirty = true;
        self
    }

    pub fn has_selection(&self) -> bool {
        self.selection_start < self.selection_end
    }

    fn clear_selection(&mut self) {
        self.selection_start = 0;
        self.selection_end = 0;
    }

    fn update_selection(&mut self) {
        self.selection_start = self.selection_anchor.min(self.cursor_pos);
        self.selection_end = self.selection_anchor.max(self.cursor_pos);
    }

    fn delete_selection(&mut self) {
        if self.has_selection() {
            self.value.drain(self.selection_start..self.selection_end);
            self.cursor_pos = self.selection_start;
            self.value_dirty = true;
            self.clear_selection();
        }
    }

    /// find the char byte index closest to a given x offset within the text
    fn x_to_cursor(&self, x_in_text: f32) -> usize {
        if self.char_offsets.is_empty() {
            return 0;
        }

        let mut best_idx = 0;
        let mut best_dist = f32::MAX;

        for &(byte_idx, offset) in &self.char_offsets {
            let dist = (offset - x_in_text).abs();
            if dist < best_dist {
                best_dist = dist;
                best_idx = byte_idx;
            }
        }
        best_idx
    }

    /// called by update.rs
    /// computes x offset for every char boundary
    pub fn update_cursor_offset(&mut self, fonts: &mut Fonts) {
        let attrs = FontAttrs {
            family: self.font_family.clone(),
            size: self.font_size,
            weight: self.font_weight,
            italic: false,
            line_height: None,
        };

        // build char_offsets: one entry per char boundary including end
        self.char_offsets.clear();
        let mut i = 0;
        while i <= self.value.len() {
            if self.value.is_char_boundary(i) {
                let offset = if i == 0 {
                    0.0
                } else {
                    fonts.measure(&self.value[..i], &attrs, None).0
                };
                self.char_offsets.push((i, offset));
            }
            i += 1;
        }

        // set cursor_offset_x from cursor_pos
        self.cursor_offset_x = self
            .char_offsets
            .iter()
            .find(|&&(idx, _)| idx == self.cursor_pos)
            .map(|&(_, x)| x)
            .unwrap_or(0.0);
    }

    pub fn toggle_blink(&mut self) {
        self.cursor_visible = !self.cursor_visible;
        self.base.render_dirty = true;
    }

    fn word_start(&self, pos: usize) -> usize {
        let mut p = pos.min(self.value.len());
        // skip non alphanumeric going left
        while p > 0 {
            let prev = self.prev_char_boundary(p);
            let c = self.value[prev..p].chars().next().unwrap_or(' ');
            if c.is_alphanumeric() || c == '_' {
                p = prev;
            } else {
                break;
            }
        }
        p
    }

    fn word_end(&self, pos: usize) -> usize {
        let mut p = pos;
        while p < self.value.len() {
            let next = self.next_char_boundary(p);
            let c = self.value[p..next].chars().next().unwrap_or(' ');
            if c.is_alphanumeric() || c == '_' {
                p = next;
            } else {
                break;
            }
        }
        p
    }

    fn select_word_at(&mut self, pos: usize) {
        let start = self.word_start(pos);
        let end = self.word_end(pos);
        // if no word found, select the char
        if start == end {
            self.selection_anchor = pos;
            self.cursor_pos = self.next_char_boundary(pos).min(self.value.len());
        } else {
            self.selection_anchor = start;
            self.cursor_pos = end;
        }
        self.update_selection();
    }

    fn prev_char_boundary(&self, pos: usize) -> usize {
        let mut p = pos;
        if p == 0 {
            return 0;
        }
        p -= 1;
        while p > 0 && !self.value.is_char_boundary(p) {
            p -= 1;
        }
        p
    }

    fn next_char_boundary(&self, pos: usize) -> usize {
        let mut p = pos + 1;
        while p < self.value.len() && !self.value.is_char_boundary(p) {
            p += 1;
        }
        p.min(self.value.len())
    }

    pub fn update_cursor_x(&mut self) {
        self.cursor_offset_x = self
            .char_offsets
            .iter()
            .find(|&&(idx, _)| idx == self.cursor_pos)
            .map(|&(_, x)| x)
            .unwrap_or(0.0);
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for TextInput {
    fn build(&mut self, scene: &mut SceneGraph) {
        self.bg_id = Some(scene.add_rect());
        self.clip_id = Some(scene.add_clip());
        self.transform_id = Some(scene.add_transform());
        self.selection_id = Some(scene.add_rect());
        self.text_id = Some(scene.add_text());
        self.cursor_id = Some(scene.add_rect());

        let clip = self.clip_id.unwrap();
        let transform = self.transform_id.unwrap();
        let selection = self.selection_id.unwrap();
        let text = self.text_id.unwrap();
        let cursor = self.cursor_id.unwrap();

        scene.add_child(SceneNodeId(clip.0), SceneNodeId(transform.0));
        scene.add_child(SceneNodeId(transform.0), SceneNodeId(selection.0));
        scene.add_child(SceneNodeId(transform.0), SceneNodeId(text.0));
        scene.add_child(SceneNodeId(transform.0), SceneNodeId(cursor.0));
    }

    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, h: f32) {
        self.width = w;
        self.height = h;

        let inner_w = w - self.padding_x * 2.0;
        let text_y = y + (h - self.font_size) / 2.0 - self.font_size * 0.15;
        self.text_x = x + self.padding_x;
        self.text_y = text_y;

        // scroll to keep cursor visible
        if self.cursor_offset_x - self.scroll_x > inner_w {
            self.scroll_x = self.cursor_offset_x - inner_w + 4.0;
        } else if self.cursor_offset_x < self.scroll_x {
            self.scroll_x = (self.cursor_offset_x - 4.0).max(0.0);
        }

        // background
        if let Some(id) = self.bg_id {
            let n = scene.rect_mut(id);
            n.set_rect(x, y, w, h);
            n.set_color(self.background_color.to_array());
            n.set_radius(self.radius);
            n.set_border_color(self.border_color.to_array());
            n.set_border_widths([self.border_width; 4]);
            n.set_visible(true);
        }

        // clip
        if let Some(id) = self.clip_id {
            scene
                .clip_mut(id)
                .set_rect(x + self.padding_x, y, inner_w, h);
        }

        // scroll transform
        if let Some(id) = self.transform_id {
            scene.transform_mut(id).set_offset(-self.scroll_x, 0.0);
        }

        // selection highlight
        if let Some(id) = self.selection_id {
            let n = scene.rect_mut(id);
            if self.has_selection() {
                let sel_x1 = self
                    .char_offsets
                    .iter()
                    .find(|&&(i, _)| i == self.selection_start)
                    .map(|&(_, x)| x)
                    .unwrap_or(0.0);
                let sel_x2 = self
                    .char_offsets
                    .iter()
                    .find(|&&(i, _)| i == self.selection_end)
                    .map(|&(_, x)| x)
                    .unwrap_or(0.0);
                n.set_rect(
                    self.text_x + sel_x1,
                    text_y - 1.0,
                    sel_x2 - sel_x1,
                    self.font_size * 1.4,
                );
                n.set_color(self.selection_color.to_array());
                n.set_visible(true);
            } else {
                n.set_visible(false);
            }
        }

        // text
        // show placeholder if empty
        if let Some(id) = self.text_id {
            let n = scene.text_mut(id);
            n.set_pos(self.text_x, text_y);
            if self.value.is_empty() && !self.placeholder.is_empty() {
                n.set_content(&self.placeholder);
                n.set_color(self.placeholder_color.to_array());
            } else {
                n.set_content(&self.value);
                n.set_color(self.text_color.to_array());
            }
            n.set_family(&self.font_family);
            n.set_size(self.font_size);
            n.set_weight(self.font_weight);
            n.set_width(f32::MAX);
            n.set_visible(true);
        }

        // cursor
        if let Some(id) = self.cursor_id {
            let n = scene.rect_mut(id);
            if self.base.focused && self.cursor_visible {
                let cursor_x = self.text_x + self.cursor_offset_x;
                n.set_rect(cursor_x, text_y - 1.0, 1.0, self.font_size * 1.4);
                n.set_color(self.cursor_color.to_array());
                n.set_visible(true);
            } else {
                n.set_visible(false);
            }
        }
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn measure(&self, _fonts: &mut Fonts, _max_width: Option<f32>) -> Option<(f32, f32)> {
        Some((200.0, self.font_size * 1.6 + self.padding_x))
    }

    fn has_measure(&self) -> bool {
        true
    }

    fn on_mouse_enter(&mut self) {
        self.set_cursor(Cursor::Text);
        self.set_border_color(Color::rgb(230, 230, 230));
    }

    fn on_mouse_leave(&mut self) {
        self.set_cursor(Cursor::Default);
        if !self.base.focused {
            self.set_border_color(Color::rgb(80, 80, 80));
        }
    }

    fn on_focus_gained(&mut self) {
        self.base.focused = true;
        self.cursor_visible = true;
        self.set_border_color(Color::rgb(230, 230, 230));
    }

    fn on_focus_lost(&mut self) {
        self.base.focused = false;
        self.selecting = false;
        self.click_count = 0;
        self.clear_selection();
        self.set_border_color(Color::rgb(80, 80, 80));
    }

    fn on_mouse_press(&mut self, mx: f32, _my: f32, button: MouseButton) {
        self.base.render_dirty = true;

        if button != MouseButton::Left {
            return;
        }

        let x_in_text = mx - self.text_x + self.scroll_x;
        let pos = self.x_to_cursor(x_in_text);

        // increment click count if clicking near same spot
        if (mx - self.last_click_x).abs() < 4.0 {
            self.click_count += 1;
        } else {
            self.click_count = 1;
        }
        self.last_click_x = mx;

        match self.click_count {
            1 => {
                // single click
                // position cursor
                self.cursor_pos = pos;
                self.selection_anchor = pos;
                self.clear_selection();
                self.selecting = true;
            }
            2 => {
                // double click
                // select word
                self.select_word_at(pos);
                self.selecting = false;
            }
            _ => {
                // triple click
                // select all
                self.selection_anchor = 0;
                self.cursor_pos = self.value.len();
                self.update_selection();
                self.selecting = false;
                self.click_count = 0;
            }
        }
    }

    fn on_mouse_move(&mut self, mx: f32, _my: f32) {
        if !self.selecting {
            return;
        }
        let x_in_text = mx - self.text_x + self.scroll_x;
        self.cursor_pos = self.x_to_cursor(x_in_text);
        self.update_selection();
        self.cursor_visible = true;
        self.base.render_dirty = true;
    }

    fn on_mouse_release(&mut self, _mx: f32, _my: f32, _button: MouseButton) {
        self.selecting = false;
        self.cursor_visible = true;
    }

    fn on_key_press(&mut self, key: Key, mods: Modifiers, text: Option<char>) {
        self.base.render_dirty = true;
        self.value_dirty = true;

        if !self.base.focused {
            return;
        }
        self.cursor_visible = true; // reset blink on any keypress

        let shift = mods.shift;

        match key {
            Key::Backspace => {
                if self.has_selection() {
                    self.delete_selection();
                } else if self.cursor_pos > 0 {
                    let new_pos = self.prev_char_boundary(self.cursor_pos);
                    self.value.drain(new_pos..self.cursor_pos);
                    self.cursor_pos = new_pos;
                    self.value_dirty = true;
                }
            }
            Key::Delete => {
                if self.has_selection() {
                    self.delete_selection();
                } else if self.cursor_pos < self.value.len() {
                    let next = self.next_char_boundary(self.cursor_pos);
                    self.value.drain(self.cursor_pos..next);
                    self.value_dirty = true;
                }
            }
            Key::Left => {
                if shift {
                    if !self.has_selection() {
                        self.selection_anchor = self.cursor_pos;
                    }
                    if self.cursor_pos > 0 {
                        self.cursor_pos = self.prev_char_boundary(self.cursor_pos);
                    }
                    self.update_selection();
                } else {
                    if self.has_selection() {
                        self.cursor_pos = self.selection_start;
                    } else if self.cursor_pos > 0 {
                        self.cursor_pos = self.prev_char_boundary(self.cursor_pos);
                    }
                    self.clear_selection();
                }
            }
            Key::Right => {
                if shift {
                    if !self.has_selection() {
                        self.selection_anchor = self.cursor_pos;
                    }
                    if self.cursor_pos < self.value.len() {
                        self.cursor_pos = self.next_char_boundary(self.cursor_pos);
                    }
                    self.update_selection();
                } else {
                    if self.has_selection() {
                        self.cursor_pos = self.selection_end;
                    } else if self.cursor_pos < self.value.len() {
                        self.cursor_pos = self.next_char_boundary(self.cursor_pos);
                    }
                    self.clear_selection();
                }
            }
            Key::Home => {
                if shift {
                    if !self.has_selection() {
                        self.selection_anchor = self.cursor_pos;
                    }
                    self.cursor_pos = 0;
                    self.update_selection();
                } else {
                    self.cursor_pos = 0;
                    self.clear_selection();
                }
            }
            Key::End => {
                if shift {
                    if !self.has_selection() {
                        self.selection_anchor = self.cursor_pos;
                    }
                    self.cursor_pos = self.value.len();
                    self.update_selection();
                } else {
                    self.cursor_pos = self.value.len();
                    self.clear_selection();
                }
            }
            Key::A if mods.ctrl || mods.cmd => {
                self.selection_anchor = 0;
                self.cursor_pos = self.value.len();
                self.update_selection();
            }
            _ => {
                if let Some(c) = text {
                    if !c.is_control() {
                        if self.has_selection() {
                            self.delete_selection();
                        }
                        self.value.insert(self.cursor_pos, c);
                        self.cursor_pos += c.len_utf8();
                        self.clear_selection();
                        self.value_dirty = true;
                    }
                }
            }
        }
    }
}
