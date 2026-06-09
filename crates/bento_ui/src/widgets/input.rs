use crate::events::{Click, KeyPress};
use crate::input::keyboard::Key;
use crate::layout::LayoutProps;
use crate::node::{self, Node};
use crate::reactive::signal::Signal;
use crate::tree;
use crate::ui;
use crate::views::{View, ViewConfig, ViewId};
use bento_wgpu::{DrawCommand, RectDraw, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};
use std::any::Any;

pub struct TextInput {
    pub value: Signal<String>,
    pub cursor: usize,
    pub cursor_x: f32,
    pub scroll_x: f32,
    pub font_size: f32,
    pub glyph_positions: Vec<f32>,
    pub last_w: f32,
    pub text_width: f32,
}

impl TextInput {
    pub fn font_size(mut self, v: f32) -> Self {
        self.font_size = v;
        self
    }
}

impl ViewConfig<TextInput> {
    pub fn font_size(mut self, v: f32) -> Self {
        self.inner.font_size = v;
        self
    }
}

impl View for TextInput {
    fn name(&self) -> &'static str {
        "TextInput"
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn measure(&mut self, measurer: &mut TextMeasurer) -> (f32, f32) {
        let text = self.value.get();
        let font_size = self.font_size;
        let line_height = font_size * 1.4;

        let result = measurer.measure(TextMeasureRequest {
            text: if text.is_empty() { " " } else { &text },
            font_family: "",
            size: font_size,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: Some(line_height),
            max_width: None,
            tab_width: 4,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });

        self.glyph_positions = result.glyph_positions.clone();
        self.text_width = result.width;

        let cur = self
            .cursor
            .min(self.glyph_positions.len().saturating_sub(1));
        self.cursor_x = self.glyph_positions[cur];

        (result.width.max(100.0), line_height.ceil())
    }

    fn render(&mut self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        let padding = 8.0;
        let available = w - padding * 2.0;
        let text_y = y + (h - self.font_size * 1.4) / 2.0;
        self.last_w = w;

        // clamp scroll to valid range
        let max_scroll = (self.text_width - available).max(0.0);
        self.scroll_x = self.scroll_x.clamp(0.0, max_scroll);

        let text_x = x + padding - self.scroll_x;
        let mut cmds = vec![];

        // background
        cmds.push(DrawCommand::Rect(RectDraw {
            x,
            y,
            w,
            h,
            color: [0.15, 0.15, 0.15, 1.0],
            radii: [4.0; 4],
            border_color: [0.4, 0.4, 0.4, 1.0],
            border_widths: [1.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: None,
            z: 0,
        }));

        // text
        cmds.push(DrawCommand::Text(TextDraw {
            x: text_x,
            y: text_y,
            w: w - padding * 2.0,
            h,
            text: self.value.get(),
            size: self.font_size,
            color: [1.0, 1.0, 1.0, 1.0],
            weight: 400,
            italic: false,
            font_family: String::new(),
            max_width: None,
            line_height: Some(self.font_size * 1.4),
            tab_width: 4,
            letter_spacing: 0.0,
            align: TextAlign::Left,
            opacity: 1.0,
            clip: Some([x, y, w, h]),
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            z: 0,
            color_ranges: vec![],
            background_ranges: vec![],
            underline_ranges: vec![],
            strikethrough_ranges: vec![],
            weight_ranges: vec![],
            italic_ranges: vec![],
            font_family_ranges: vec![],
        }));

        // cursor
        let cursor_h = self.font_size * 1.4;
        let cursor_y = y + (h - cursor_h) / 2.0;
        cmds.push(DrawCommand::Rect(RectDraw {
            x: text_x + self.cursor_x,
            y: cursor_y,
            w: 1.0,
            h: cursor_h,
            color: [1.0, 1.0, 1.0, 1.0],
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: Some([x, y, w, h]),
            z: 0,
        }));

        cmds
    }

    fn build(self: Box<Self>) -> ViewId {
        let value = self.value;

        let node = Node {
            name: Some("TextInput (Primitive)"),
            taffy_id: node::placeholder_taffy_id(),
            parent: None,
            children: Vec::new(),
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            layout: LayoutProps::default(),
            handlers: Vec::new(),
            owners: Vec::new(),
            paint_dirty: true,
            cache: Vec::new(),
            paint_subscriber: None,
            scroll_x: 0.0,
            scroll_y: 0.0,
            scrollable: false,
            clip: false,
        };

        let id = tree::add_node(node, self);

        tree::add_handler(id, move |e: &KeyPress| {
            tree::mutate_view(id, |view: &mut TextInput| {
                let mut val = value.get();
                let cur = view.cursor.min(val.chars().count());

                match e.key {
                    Key::Backspace => {
                        if cur > 0 {
                            let byte_idx =
                                val.char_indices().nth(cur - 1).map(|(i, _)| i).unwrap_or(0);
                            val.remove(byte_idx);
                            value.set(val);
                            view.cursor = cur - 1;
                        }
                    }
                    Key::Delete => {
                        if cur < val.chars().count() {
                            let byte_idx = val.char_indices().nth(cur).map(|(i, _)| i).unwrap_or(0);
                            val.remove(byte_idx);
                            value.set(val);
                        }
                    }
                    Key::Left => {
                        if cur > 0 {
                            view.cursor = cur - 1;
                        }
                    }
                    Key::Right => {
                        if cur < val.chars().count() {
                            view.cursor = cur + 1;
                        }
                    }
                    Key::Home => {
                        view.cursor = 0;
                    }
                    Key::End => {
                        view.cursor = val.chars().count();
                    }
                    _ => {
                        if let Some(ch) = e.ch {
                            if !ch.is_control() && !e.key.is_modifier() {
                                let byte_idx = val
                                    .char_indices()
                                    .nth(cur)
                                    .map(|(i, _)| i)
                                    .unwrap_or(val.len());
                                val.insert(byte_idx, ch);
                                value.set(val);
                                view.cursor = cur + 1;
                            }
                        }
                    }
                }

                // update cursor_x from stored glyph positions
                let new_cur = view
                    .cursor
                    .min(view.glyph_positions.len().saturating_sub(1));
                view.cursor_x = if view.glyph_positions.is_empty() {
                    0.0
                } else {
                    view.glyph_positions[new_cur]
                };

                // snap scroll to keep cursor visible
                let available = view.last_w - 16.0;
                if view.cursor_x - view.scroll_x > available {
                    view.scroll_x = view.cursor_x - available;
                }
                if view.cursor_x < view.scroll_x {
                    view.scroll_x = view.cursor_x;
                }
            });

            ui::request_layout();
        });

        tree::add_handler(id, move |e: &crate::events::MouseScroll| {
            tree::mutate_view(id, |view: &mut TextInput| {
                view.scroll_x = (view.scroll_x - e.x).max(0.0);
            });
            ui::request_redraw();
        });

        tree::add_handler(id, move |e: &Click| {
            let (node_x, _, _, _) = tree::get_rect(id);
            tree::mutate_view(id, |view: &mut TextInput| {
                let padding = 8.0;
                let relative_x = e.x - node_x - padding + view.scroll_x;

                let mut best = 0;
                let mut best_dist = f32::MAX;
                for (i, &gx) in view.glyph_positions.iter().enumerate() {
                    let dist = (gx - relative_x).abs();
                    if dist < best_dist {
                        best_dist = dist;
                        best = i;
                    }
                }
                view.cursor = best;
                view.cursor_x = view.glyph_positions.get(best).copied().unwrap_or(0.0);
            });
            ui::request_redraw();
        });

        id
    }
}

pub fn text_input(value: Signal<String>) -> TextInput {
    TextInput {
        value,
        cursor: 0,
        cursor_x: 0.0,
        scroll_x: 0.0,
        font_size: 14.0,
        glyph_positions: Vec::new(),
        last_w: 0.0,
        text_width: 0.0,
    }
}
