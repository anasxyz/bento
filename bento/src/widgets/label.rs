use crate::color::Color;
use crate::fonts::{FontAttrs, Fonts};
use crate::input::Cursor;
use std::time::Instant;

use crate::ui::{FocusGained, FocusLost, Hover, HoverEnd, MouseMove, Press, Release, Ui};
use crate::widget::{AsAny, Base, Handle, HasBase, Widget};
use bento_derive::Widget;
use bento_wgpu::{SceneGraph, TextDecoration, TextId};

const DOUBLE_CLICK_MS: u128 = 500;

#[derive(Widget)]
pub struct Label {
    pub base: Base,
    text: String,
    family: String,
    size: f32,
    weight: u16,
    italic: bool,
    color: Color,
    wrap: bool,
    pub selectable: bool,
    selection_color: Color,
    pub text_dirty: bool,

    // decorations
    underlines: Vec<TextDecoration>,
    strikethroughs: Vec<TextDecoration>,

    // selection state
    selecting: bool,
    selection_anchor: usize,
    selection_start: usize,
    selection_end: usize,

    // click tracking
    click_count: u32,
    last_click_time: Instant,

    // for hit testing
    pub char_offsets: Vec<(usize, f32)>,
    text_x: f32,
    text_y: f32,

    text_id: Option<TextId>,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            base: Base::new(),
            text: text.to_string(),
            family: "sans-serif".to_string(),
            size: 14.0,
            weight: 400,
            italic: false,
            color: Color::WHITE,
            wrap: true,
            selectable: false,
            selection_color: Color::rgba(68, 152, 227, 80),
            text_dirty: true,
            underlines: Vec::new(),
            strikethroughs: Vec::new(),
            selecting: false,
            selection_anchor: 0,
            selection_start: 0,
            selection_end: 0,
            click_count: 0,
            last_click_time: Instant::now(),
            char_offsets: Vec::new(),
            text_x: 0.0,
            text_y: 0.0,
            text_id: None,
        }
    }

    pub fn set_text(&mut self, s: &str) -> &mut Self {
        self.text = s.to_string();
        self.selection_start = 0;
        self.selection_end = 0;
        self.text_dirty = true;
        self.base.render_dirty = true;
        self.base.layout_dirty = true;
        self
    }
    pub fn set_font_family(&mut self, s: &str) -> &mut Self {
        self.family = s.to_string();
        self.text_dirty = true;
        self.base.render_dirty = true;
        self
    }
    pub fn set_size(&mut self, v: f32) -> &mut Self {
        self.size = v;
        self.text_dirty = true;
        self.base.render_dirty = true;
        self
    }
    pub fn set_weight(&mut self, v: u16) -> &mut Self {
        self.weight = v;
        self.text_dirty = true;
        self.base.render_dirty = true;
        self
    }
    pub fn set_italic(&mut self, v: bool) -> &mut Self {
        self.italic = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_color(&mut self, c: Color) -> &mut Self {
        self.color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_wrap(&mut self, v: bool) -> &mut Self {
        self.wrap = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_selectable(&mut self, v: bool) -> &mut Self {
        self.selectable = v;
        self.base.layout_dirty = true;
        self
    }
    pub fn set_selection_color(&mut self, c: Color) -> &mut Self {
        self.selection_color = c;
        self.base.render_dirty = true;
        self
    }

    pub fn add_underline(
        &mut self,
        start: usize,
        end: usize,
        color: Color,
        thickness: f32,
    ) -> &mut Self {
        self.underlines.push(TextDecoration {
            start,
            end,
            color: color.to_array(),
            thickness,
        });
        self.base.render_dirty = true;
        self
    }

    pub fn add_strikethrough(
        &mut self,
        start: usize,
        end: usize,
        color: Color,
        thickness: f32,
    ) -> &mut Self {
        self.strikethroughs.push(TextDecoration {
            start,
            end,
            color: color.to_array(),
            thickness,
        });
        self.base.render_dirty = true;
        self
    }

    pub fn clear_underlines(&mut self) -> &mut Self {
        self.underlines.clear();
        self.base.render_dirty = true;
        self
    }

    pub fn clear_strikethroughs(&mut self) -> &mut Self {
        self.strikethroughs.clear();
        self.base.render_dirty = true;
        self
    }

    fn has_selection(&self) -> bool {
        self.selection_start < self.selection_end
    }

    fn x_to_cursor(&self, x: f32) -> usize {
        if self.char_offsets.is_empty() {
            return 0;
        }
        let mut best_idx = 0;
        let mut best_dist = f32::MAX;
        for &(byte_idx, offset) in &self.char_offsets {
            let dist = (offset - x).abs();
            if dist < best_dist {
                best_dist = dist;
                best_idx = byte_idx;
            }
        }
        best_idx
    }

    fn word_start(&self, pos: usize) -> usize {
        let mut p = pos.min(self.text.len());
        while p > 0 {
            let prev = self.prev_char(p);
            let c = self.text[prev..p].chars().next().unwrap_or(' ');
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
        while p < self.text.len() {
            let next = self.next_char(p);
            let c = self.text[p..next].chars().next().unwrap_or(' ');
            if c.is_alphanumeric() || c == '_' {
                p = next;
            } else {
                break;
            }
        }
        p
    }

    fn prev_char(&self, pos: usize) -> usize {
        let mut p = pos;
        if p == 0 {
            return 0;
        }
        p -= 1;
        while p > 0 && !self.text.is_char_boundary(p) {
            p -= 1;
        }
        p
    }

    fn next_char(&self, pos: usize) -> usize {
        let mut p = pos + 1;
        while p < self.text.len() && !self.text.is_char_boundary(p) {
            p += 1;
        }
        p.min(self.text.len())
    }

    pub fn update_char_offsets(&mut self, fonts: &mut Fonts) {
        if !self.selectable {
            return;
        }
        let attrs = FontAttrs {
            family: self.family.clone(),
            size: self.size,
            weight: self.weight,
            italic: self.italic,
            line_height: None,
        };
        self.char_offsets.clear();
        let mut i = 0;
        while i <= self.text.len() {
            if self.text.is_char_boundary(i) {
                let x = if i == 0 {
                    0.0
                } else {
                    fonts.measure(&self.text[..i], &attrs, None).0
                };
                self.char_offsets.push((i, x));
            }
            i += 1;
        }
    }

    fn font_weight_u16(&self) -> u16 {
        self.weight
    }
}

impl Widget for Label {
    fn build(&mut self, scene: &mut SceneGraph) {
        self.text_id = Some(scene.add_text());
    }

    fn register(&mut self, handle: Handle<()>, ui: &mut Ui) {
        let h = Handle::<Label>::new(handle.id, handle.generation);

        ui.on::<Label, Hover>(h, |ui, this, e| {
            if !this.selectable {
                return;
            }
            this.base.cursor = Cursor::Text;
        });

        ui.on::<Label, HoverEnd>(h, |ui, this, e| {
            this.base.cursor = Cursor::Default;
        });

        ui.on::<Label, FocusLost>(h, |ui, this, e| {
            this.selection_start = 0;
            this.selection_end = 0;
            this.selecting = false;
            this.click_count = 0;
            this.base.render_dirty = true;
        });

        ui.on::<Label, Press>(h, |ui, this, e| {
            if !this.selectable {
                return;
            }
            let x_in_text = e.x - this.text_x;
            let pos = this.x_to_cursor(x_in_text);

            let now = Instant::now();
            if now.duration_since(this.last_click_time).as_millis() < DOUBLE_CLICK_MS {
                this.click_count += 1;
            } else {
                this.click_count = 1;
            }
            this.last_click_time = now;

            match this.click_count {
                1 => {
                    this.selection_anchor = pos;
                    this.selection_start = pos;
                    this.selection_end = pos;
                    this.selecting = true;
                }
                2 => {
                    // word selection
                    let start = this.word_start(pos);
                    let end = this.word_end(pos);
                    this.selection_anchor = start;
                    this.selection_start = start;
                    this.selection_end = end;
                    this.selecting = false;
                }
                _ => {
                    // select all
                    this.selection_anchor = 0;
                    this.selection_start = 0;
                    this.selection_end = this.text.len();
                    this.selecting = false;
                    this.click_count = 0;
                }
            }
            this.base.render_dirty = true;
        });

        ui.on::<Label, MouseMove>(h, |ui, this, e| {
            if !this.selecting {
                return;
            }
            let x_in_text = e.x - this.text_x;
            let pos = this.x_to_cursor(x_in_text);
            this.selection_start = this.selection_anchor.min(pos);
            this.selection_end = this.selection_anchor.max(pos);
            this.base.render_dirty = true;
        });

        ui.on::<Label, Release>(h, |ui, this, e| {
            this.selecting = false;
        });
    }

    fn sync(&mut self, scene: &mut SceneGraph) {
        let layer = self.base.layer();
        let x = self.base.x();
        let y = self.base.y();
        let w = self.base.w();
        let h = self.base.h();

        self.text_x = x;
        self.text_y = y;

        if let Some(id) = self.text_id {
            let node = scene.text_mut(id);
            node.set_pos(x, y);
            node.set_content(&self.text);
            node.set_family(&self.family);
            node.set_size(self.size);
            node.set_weight(self.weight);
            node.set_italic(self.italic);
            node.set_color(self.color.to_array());
            node.set_width(if self.wrap { w } else { f32::MAX });
            node.set_z(layer as i32);
            node.set_visible(self.base.visible);

            if self.selectable && self.has_selection() {
                node.set_selection(self.selection_start, self.selection_end);
                node.set_selection_color(self.selection_color.to_array());
            } else {
                node.clear_selection();
            }

            node.clear_underlines();
            for d in &self.underlines {
                node.add_underline(d.start, d.end, d.color, d.thickness);
            }
            node.clear_strikethroughs();
            for d in &self.strikethroughs {
                node.add_strikethrough(d.start, d.end, d.color, d.thickness);
            }
        }
    }

    fn is_interactive(&self) -> bool {
        self.selectable
    }

    fn measure(&mut self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)> {
        let attrs = FontAttrs {
            family: self.family.clone(),
            size: self.size,
            weight: self.weight,
            italic: self.italic,
            line_height: None,
        };
        let max = if self.wrap { max_width } else { None };
        Some(fonts.measure(&self.text, &attrs, max))
    }

    fn has_measure(&self) -> bool {
        true
    }
}
