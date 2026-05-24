use crate::Key;
use crate::events::types::{Change, FocusGained, FocusLost, KeyPress};
use crate::layout::Size;
use crate::widget::{Canvas, Widget};
use bento_wgpu::{RectDraw, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};
use std::any::Any;

pub struct TextInput {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub width: Size,
    pub height: Size,
    pub value: String,
    pub color: [f32; 4],
    pub background: [f32; 4],
    pub font_size: f32,
    pub padding: f32,
    pub z: i32,
    cursor: usize,
    focused: bool,
    text_w: f32,
    text_h: f32,
    cursor_x: f32,
    scroll_offset: f32,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 200.0,
            h: 0.0,
            width: Size::Fixed(200.0),
            height: Size::Auto,
            value: String::new(),
            color: [1.0, 1.0, 1.0, 1.0],
            background: [0.15, 0.15, 0.15, 1.0],
            font_size: 14.0,
            padding: 8.0,
            z: 0,
            cursor: 0,
            focused: false,
            text_w: 0.0,
            text_h: 0.0,
            cursor_x: 0.0,
            scroll_offset: 0.0,
        }
    }
}

impl Widget for TextInput {
    fn name(&self) -> &str {
        "TextInput"
    }

    fn update(&mut self, measurer: &mut TextMeasurer) {
        let t = std::time::Instant::now();
        let result = measurer.measure(TextMeasureRequest {
            text: if self.value.is_empty() {
                " "
            } else {
                &self.value
            },
            font_family: "",
            size: self.font_size,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            max_width: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        println!("[update] measure time: {:?} +", t.elapsed());
        self.text_w = result.width;
        self.text_h = result.height;
        if matches!(self.height, Size::Auto) {
            self.h = self.text_h + self.padding * 2.0;
        }
        if matches!(self.width, Size::Auto) {
            self.w = (result.width + self.padding * 2.0).max(100.0);
        }

        // measure text up to cursor to get accurate cursor x
        let text_before: String = self.value.chars().take(self.cursor).collect();
        let t = std::time::Instant::now();
        let cursor_result = measurer.measure(TextMeasureRequest {
            text: if text_before.is_empty() {
                " "
            } else {
                &text_before
            },
            font_family: "",
            size: self.font_size,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            max_width: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        println!("[update] second measure time: {:?} +", t.elapsed());
        self.cursor_x = if text_before.is_empty() {
            0.0
        } else {
            cursor_result.width
        };

        // update scroll offset to keep cursor visible
        let inner_w = self.w - self.padding * 2.0;
        let cursor_local = self.cursor_x - self.scroll_offset;
        if cursor_local < 0.0 {
            self.scroll_offset = self.cursor_x;
        } else if cursor_local > inner_w {
            self.scroll_offset = self.cursor_x - inner_w;
        }
        let total_text_w = result.width;
        let max_scroll = (total_text_w - inner_w).max(0.0);
        self.scroll_offset = self.scroll_offset.clamp(0.0, max_scroll);
        println!(
            "[input] value_len={} result.width={:.1} inner_w={:.1} scroll={:.1} cursor_x={:.1} max_scroll={:.1}",
            self.value.len(),
            result.width,
            inner_w,
            self.scroll_offset,
            self.cursor_x,
            (result.width - inner_w).max(0.0)
        );
    }

    fn size(&self) -> (f32, f32) {
        (self.w, self.h)
    }
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    fn set_size(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
    }
    fn width_sizing(&self) -> &Size {
        &self.width
    }
    fn height_sizing(&self) -> &Size {
        &self.height
    }
    fn z(&self) -> i32 {
        self.z
    }

    fn on_event(&mut self, event: &dyn Any) -> (bool, Vec<Box<dyn Any>>) {
        let mut changed = false;

        if event.downcast_ref::<FocusGained>().is_some() {
            self.focused = true;
            changed = true;
        }

        if event.downcast_ref::<FocusLost>().is_some() {
            self.focused = false;
            changed = true;
        }

        if let Some(e) = event.downcast_ref::<KeyPress>() {
            match e.key {
                Key::Backspace => {
                    if self.cursor > 0 {
                        let byte_idx = char_to_byte(&self.value, self.cursor - 1);
                        let end_idx = char_to_byte(&self.value, self.cursor);
                        self.value.drain(byte_idx..end_idx);
                        self.cursor -= 1;
                        changed = true;
                    }
                }
                Key::Delete => {
                    if self.cursor < self.value.chars().count() {
                        let byte_idx = char_to_byte(&self.value, self.cursor);
                        let end_idx = char_to_byte(&self.value, self.cursor + 1);
                        self.value.drain(byte_idx..end_idx);
                        changed = true;
                    }
                }
                Key::Left => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                        changed = true;
                    }
                }
                Key::Right => {
                    if self.cursor < self.value.chars().count() {
                        self.cursor += 1;
                        changed = true;
                    }
                }
                Key::Home => {
                    if self.cursor != 0 {
                        self.cursor = 0;
                        changed = true;
                    }
                }
                Key::End => {
                    let len = self.value.chars().count();
                    if self.cursor != len {
                        self.cursor = len;
                        changed = true;
                    }
                }
                _ => {
                    if let Some(ch) = e.ch {
                        if !ch.is_control() {
                            let byte_idx = char_to_byte(&self.value, self.cursor);
                            self.value.insert(byte_idx, ch);
                            self.cursor += 1;
                            changed = true;
                        }
                    }
                }
            }
        }

        (changed, vec![])
    }

    fn render(&self, canvas: &mut Canvas) {
        // background
        canvas.draw_list.push_rect(RectDraw {
            x: canvas.x,
            y: canvas.y,
            w: self.w,
            h: self.h,
            color: self.background,
            radii: [0.0; 4],
            border_color: if self.focused {
                [0.0, 0.5, 1.0, 1.0]
            } else {
                [0.3, 0.3, 0.3, 1.0]
            },
            border_widths: [1.0; 4],
            rotate: canvas.rotate,
            scale_x: canvas.scale_x,
            scale_y: canvas.scale_y,
            opacity: canvas.opacity,
            clip: canvas.clip,
            z: canvas.z,
        });

        // text
        canvas.draw_list.push_text(TextDraw {
            x: canvas.x + self.padding - self.scroll_offset,
            y: canvas.y + self.padding,
            w: self.text_w.max(self.w - self.padding * 2.0),
            h: self.text_h,
            text: self.value.clone(),
            size: self.font_size,
            color: self.color,
            weight: 400,
            italic: false,
            font_family: String::new(),
            max_width: None,
            line_height: None,
            letter_spacing: 0.0,
            align: TextAlign::Left,
            opacity: canvas.opacity,
            clip: Some([canvas.x, canvas.y, self.w, self.h]),
            rotate: canvas.rotate,
            scale_x: canvas.scale_x,
            scale_y: canvas.scale_y,
            z: canvas.z + 1,
            color_ranges: vec![],
            background_ranges: vec![],
            underline_ranges: vec![],
            strikethrough_ranges: vec![],
            weight_ranges: vec![],
            italic_ranges: vec![],
            font_family_ranges: vec![],
        });

        // cursor
        if self.focused {
            canvas.draw_list.push_rect(RectDraw {
                x: canvas.x + self.padding + self.cursor_x - self.scroll_offset,
                y: canvas.y + self.padding,
                w: 1.5,
                h: self.text_h,
                color: [1.0, 1.0, 1.0, 1.0],
                radii: [0.0; 4],
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                rotate: canvas.rotate,
                scale_x: canvas.scale_x,
                scale_y: canvas.scale_y,
                opacity: canvas.opacity,
                clip: Some([canvas.x, canvas.y, self.w, self.h]),
                z: canvas.z + 2,
            });
        }
    }
}

fn char_to_byte(text: &str, char_idx: usize) -> usize {
    text.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(text.len())
}
