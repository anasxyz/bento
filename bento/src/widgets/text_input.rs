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

    // style
    background_color: Color,
    text_color: Color,
    cursor_color: Color,
    border_color: Color,
    border_width: f32,
    radius: f32,
    padding_x: f32,
    font_family: String,
    font_size: f32,
    font_weight: u16,

    // state
    pub cursor_pos: usize,    // byte index into value
    pub cursor_offset_x: f32, // pixel offset of cursor, this is set by update.rs
    scroll_x: f32,            // horizontal scroll offset

    // computed in sync
    text_x: f32,
    text_y: f32,
    width: f32,
    height: f32,

    // scene nodes
    bg_id: Option<RectId>,
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
            background_color: Color::rgb(40, 40, 40),
            text_color: Color::WHITE,
            cursor_color: Color::WHITE,
            border_color: Color::rgb(80, 80, 80),
            border_width: 1.0,
            radius: 4.0,
            padding_x: 8.0,
            font_family: "sans-serif".to_string(),
            font_size: 14.0,
            font_weight: 400,
            cursor_pos: 0,
            cursor_offset_x: 0.0,
            scroll_x: 0.0,
            text_x: 0.0,
            text_y: 0.0,
            width: 0.0,
            height: 0.0,
            bg_id: None,
            clip_id: None,
            transform_id: None,
            text_id: None,
            cursor_id: None,
        }
    }

    pub fn set_value(&mut self, s: &str) -> &mut Self {
        self.value = s.to_string();
        self.cursor_pos = self.value.len();
        self
    }
    pub fn set_background_color(&mut self, c: Color) -> &mut Self {
        self.background_color = c;
        self
    }
    pub fn set_text_color(&mut self, c: Color) -> &mut Self {
        self.text_color = c;
        self
    }
    pub fn set_cursor_color(&mut self, c: Color) -> &mut Self {
        self.cursor_color = c;
        self
    }
    pub fn set_border_color(&mut self, c: Color) -> &mut Self {
        self.border_color = c;
        self
    }
    pub fn set_border_width(&mut self, v: f32) -> &mut Self {
        self.border_width = v;
        self
    }
    pub fn set_border_radius(&mut self, v: f32) -> &mut Self {
        self.radius = v;
        self
    }
    pub fn set_padding_x(&mut self, v: f32) -> &mut Self {
        self.padding_x = v;
        self
    }
    pub fn set_font_family(&mut self, s: &str) -> &mut Self {
        self.font_family = s.to_string();
        self
    }
    pub fn set_font_size(&mut self, v: f32) -> &mut Self {
        self.font_size = v;
        self
    }
    pub fn set_font_weight(&mut self, v: u16) -> &mut Self {
        self.font_weight = v;
        self
    }

    /// measure cursor x offset
    /// called by update.rs which has access to fonts
    pub fn update_cursor_offset(&mut self, fonts: &mut Fonts) {
        let before = &self.value[..self.cursor_pos];
        if before.is_empty() {
            self.cursor_offset_x = 0.0;
            return;
        }
        let attrs = FontAttrs {
            family: self.font_family.clone(),
            size: self.font_size,
            weight: self.font_weight,
            italic: false,
            line_height: None,
        };
        self.cursor_offset_x = fonts.measure(before, &attrs, None).0;
    }

    fn clamp_cursor(&self, pos: usize) -> usize {
        let pos = pos.min(self.value.len());
        let mut p = pos;
        while p > 0 && !self.value.is_char_boundary(p) {
            p -= 1;
        }
        p
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
        self.text_id = Some(scene.add_text());
        self.cursor_id = Some(scene.add_rect());

        let clip = self.clip_id.unwrap();
        let transform = self.transform_id.unwrap();
        let text = self.text_id.unwrap();
        let cursor = self.cursor_id.unwrap();

        // clip contains transform, transform contains text and cursor
        scene.add_child(SceneNodeId(clip.0), SceneNodeId(transform.0));
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
        let cursor_x = self.cursor_offset_x;
        if cursor_x - self.scroll_x > inner_w {
            self.scroll_x = cursor_x - inner_w + 4.0;
        } else if cursor_x < self.scroll_x {
            self.scroll_x = (cursor_x - 4.0).max(0.0);
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

        // clip to text area
        if let Some(id) = self.clip_id {
            scene
                .clip_mut(id)
                .set_rect(x + self.padding_x, y, inner_w, h);
        }

        // scroll transform
        if let Some(id) = self.transform_id {
            scene.transform_mut(id).set_offset(-self.scroll_x, 0.0);
        }

        // text node
        if let Some(id) = self.text_id {
            let n = scene.text_mut(id);
            n.set_pos(self.text_x, text_y);
            n.set_content(&self.value);
            n.set_family(&self.font_family);
            n.set_size(self.font_size);
            n.set_weight(self.font_weight);
            n.set_color(self.text_color.to_array());
            n.set_width(f32::MAX);
            n.set_visible(true);
        }

        // cursor rect 
        // inside transform so position is unscrolled
        if let Some(id) = self.cursor_id {
            let n = scene.rect_mut(id);
            if self.base.focused {
                let cursor_x = self.text_x + self.cursor_offset_x;
                n.set_rect(cursor_x, text_y - 1.0, 2.0, self.font_size * 1.4); // cursor taller than text
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
        self.set_border_color(Color::rgb(10, 80, 80));
    }

    fn on_mouse_leave(&mut self) {
        if !self.base.focused {
            self.set_border_color(Color::rgb(80, 80, 80));
        }
    }

    fn on_focus_gained(&mut self) {
        self.base.focused = true;
        self.set_border_color(Color::rgb(10, 80, 80));
    }

    fn on_focus_lost(&mut self) {
        self.base.focused = false;
        self.set_border_color(Color::rgb(80, 80, 80));
    }

    fn on_key_press(&mut self, key: Key, _mods: Modifiers, text: Option<char>) {
        if !self.base.focused {
            return;
        }
        match key {
            Key::Backspace => {
                if self.cursor_pos > 0 {
                    let new_pos = self.prev_char_boundary(self.cursor_pos);
                    self.value.drain(new_pos..self.cursor_pos);
                    self.cursor_pos = new_pos;
                }
            }
            Key::Delete => {
                if self.cursor_pos < self.value.len() {
                    let next = self.next_char_boundary(self.cursor_pos);
                    self.value.drain(self.cursor_pos..next);
                }
            }
            Key::Left => {
                if self.cursor_pos > 0 {
                    self.cursor_pos = self.prev_char_boundary(self.cursor_pos);
                }
            }
            Key::Right => {
                if self.cursor_pos < self.value.len() {
                    self.cursor_pos = self.next_char_boundary(self.cursor_pos);
                }
            }
            Key::Home => {
                self.cursor_pos = 0;
            }
            Key::End => {
                self.cursor_pos = self.value.len();
            }
            _ => {
                if let Some(c) = text {
                    if !c.is_control() {
                        self.value.insert(self.cursor_pos, c);
                        self.cursor_pos += c.len_utf8();
                    }
                }
            }
        }
    }
}
