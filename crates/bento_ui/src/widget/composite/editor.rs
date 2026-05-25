use crate::events::types::{FocusGained, FocusLost, KeyPress, MouseScroll};
use crate::layout::Size;
use crate::ui::TimerHandle;
use crate::widget::{Canvas, Widget, WidgetHandle};
use crate::{Key, Ui};
use bento_wgpu::{RectDraw, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};

pub struct MultilineInput {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub width: Size,
    pub height: Size,
    pub lines: Vec<String>,
    pub color: [f32; 4],
    pub background: [f32; 4],
    pub font_size: f32,
    pub line_height: f32,
    pub padding: f32,
    pub z: i32,
    cursor_line: usize,
    cursor_col: usize,
    focused: bool,
    cursor_visible: bool,
    blink_handle: Option<TimerHandle>,
    cursor_x: f32,
    scroll_y: f32,
}

impl MultilineInput {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 400.0,
            h: 300.0,
            width: Size::Fixed(400.0),
            height: Size::Fixed(300.0),
            lines: vec![String::new()],
            color: [1.0, 1.0, 1.0, 1.0],
            background: [0.15, 0.15, 0.15, 1.0],
            font_size: 14.0,
            line_height: 20.0,
            padding: 8.0,
            z: 0,
            cursor_line: 0,
            cursor_col: 0,
            focused: false,
            cursor_visible: true,
            blink_handle: None,
            cursor_x: 0.0,
            scroll_y: 0.0,
        }
    }

    fn handle_key(&mut self, e: &KeyPress) -> bool {
        match e.key {
            Key::Enter => {
                let line = &self.lines[self.cursor_line];
                let byte_idx = char_to_byte(line, self.cursor_col);
                let rest = line[byte_idx..].to_string();
                self.lines[self.cursor_line].truncate(byte_idx);
                self.cursor_line += 1;
                self.lines.insert(self.cursor_line, rest);
                self.cursor_col = 0;
                return true;
            }
            Key::Backspace => {
                if self.cursor_col > 0 {
                    let line = &mut self.lines[self.cursor_line];
                    let byte_idx = char_to_byte(line, self.cursor_col - 1);
                    let end_idx = char_to_byte(line, self.cursor_col);
                    line.drain(byte_idx..end_idx);
                    self.cursor_col -= 1;
                    return true;
                } else if self.cursor_line > 0 {
                    let line = self.lines.remove(self.cursor_line);
                    self.cursor_line -= 1;
                    self.cursor_col = self.lines[self.cursor_line].chars().count();
                    self.lines[self.cursor_line].push_str(&line);
                    return true;
                }
            }
            Key::Delete => {
                let line_len = self.lines[self.cursor_line].chars().count();
                if self.cursor_col < line_len {
                    let line = &mut self.lines[self.cursor_line];
                    let byte_idx = char_to_byte(line, self.cursor_col);
                    let end_idx = char_to_byte(line, self.cursor_col + 1);
                    line.drain(byte_idx..end_idx);
                    return true;
                } else if self.cursor_line < self.lines.len() - 1 {
                    let next = self.lines.remove(self.cursor_line + 1);
                    self.lines[self.cursor_line].push_str(&next);
                    return true;
                }
            }
            Key::Left => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                    return true;
                } else if self.cursor_line > 0 {
                    self.cursor_line -= 1;
                    self.cursor_col = self.lines[self.cursor_line].chars().count();
                    return true;
                }
            }
            Key::Right => {
                let line_len = self.lines[self.cursor_line].chars().count();
                if self.cursor_col < line_len {
                    self.cursor_col += 1;
                    return true;
                } else if self.cursor_line < self.lines.len() - 1 {
                    self.cursor_line += 1;
                    self.cursor_col = 0;
                    return true;
                }
            }
            Key::Up => {
                if self.cursor_line > 0 {
                    self.cursor_line -= 1;
                    let line_len = self.lines[self.cursor_line].chars().count();
                    self.cursor_col = self.cursor_col.min(line_len);
                    return true;
                }
            }
            Key::Down => {
                if self.cursor_line < self.lines.len() - 1 {
                    self.cursor_line += 1;
                    let line_len = self.lines[self.cursor_line].chars().count();
                    self.cursor_col = self.cursor_col.min(line_len);
                    return true;
                }
            }
            Key::Home => {
                if self.cursor_col != 0 {
                    self.cursor_col = 0;
                    return true;
                }
            }
            Key::End => {
                let len = self.lines[self.cursor_line].chars().count();
                if self.cursor_col != len {
                    self.cursor_col = len;
                    return true;
                }
            }
            _ => {
                if let Some(ch) = e.ch {
                    if !ch.is_control() {
                        let line = &mut self.lines[self.cursor_line];
                        let byte_idx = char_to_byte(line, self.cursor_col);
                        line.insert(byte_idx, ch);
                        self.cursor_col += 1;
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn blink_tick_multi(ui: &mut Ui, handle: WidgetHandle<MultilineInput>) {
    let h = ui.asyncs.timer(0.53, move |ui| {
        if let Some(input) = ui.get_mut_internal(handle) {
            if input.focused {
                input.cursor_visible = !input.cursor_visible;
                ui.needs_redraw = true;
                blink_tick_multi(ui, handle);
            }
        }
    });
    if let Some(input) = ui.get_mut_internal(handle) {
        input.blink_handle = Some(h);
    }
}

impl Widget for MultilineInput {
    fn name(&self) -> &str {
        "MultilineInput"
    }

    fn build(&mut self, ui: &mut Ui, handle: WidgetHandle<()>) {
        let handle = handle.typed::<MultilineInput>();

        ui.listen(handle, move |e: &FocusGained, ui: &mut Ui| {
            if let Some(input) = ui.get_mut_internal(handle) {
                input.focused = true;
                input.cursor_visible = true;
            }
            blink_tick_multi(ui, handle);
        });

        ui.listen(handle, move |e: &FocusLost, ui: &mut Ui| {
            if let Some(input) = ui.get_mut_internal(handle) {
                input.focused = false;
                input.cursor_visible = true;
                if let Some(h) = input.blink_handle.take() {
                    h.cancel();
                }
            }
        });

        ui.listen(handle, move |e: &KeyPress, ui: &mut Ui| {
            if let Some(input) = ui.get_mut_internal(handle) {
                let changed = input.handle_key(e);
                if changed {
                    input.cursor_visible = true;
                    if let Some(h) = input.blink_handle.take() {
                        h.cancel();
                    }
                    ui.dirty.insert(handle.id);
                    ui.needs_redraw = true;
                    blink_tick_multi(ui, handle);
                }
            }
        });

        ui.listen(handle, move |e: &MouseScroll, ui: &mut Ui| {
            if let Some(input) = ui.get_mut_internal(handle) {
                let max_scroll = (input.lines.len() as f32 * input.line_height
                    - (input.h - input.padding * 2.0))
                    .max(0.0);
                input.scroll_y =
                    (input.scroll_y - e.y * input.line_height * 3.0).clamp(0.0, max_scroll);
                ui.needs_redraw = true;
            }
        });
    }

    fn update(&mut self, measurer: &mut TextMeasurer) {
        let current_line = &self.lines[self.cursor_line];
        let result = measurer.measure(TextMeasureRequest {
            text: if current_line.is_empty() {
                " "
            } else {
                current_line
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

        self.cursor_x = result
            .glyph_positions
            .get(self.cursor_col)
            .copied()
            .unwrap_or(result.width);

        let inner_h = self.h - self.padding * 2.0;
        let cursor_top = self.cursor_line as f32 * self.line_height;
        let cursor_bottom = cursor_top + self.line_height;
        if cursor_top < self.scroll_y {
            self.scroll_y = cursor_top;
        } else if cursor_bottom > self.scroll_y + inner_h {
            self.scroll_y = cursor_bottom - inner_h;
        }
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

    fn render(&self, canvas: &mut Canvas) {
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

        let clip = Some([canvas.x, canvas.y, self.w, self.h]);

        for (i, line) in self.lines.iter().enumerate() {
            let line_y = canvas.y + self.padding + i as f32 * self.line_height - self.scroll_y;

            if line_y + self.line_height < canvas.y || line_y > canvas.y + self.h {
                continue;
            }

            canvas.draw_list.push_text(TextDraw {
                x: canvas.x + self.padding,
                y: line_y,
                w: self.w - self.padding * 2.0,
                h: self.line_height,
                text: line.clone(),
                size: self.font_size,
                color: self.color,
                weight: 400,
                italic: false,
                font_family: String::new(),
                max_width: None,
                line_height: Some(self.line_height),
                letter_spacing: 0.0,
                align: TextAlign::Left,
                opacity: canvas.opacity,
                clip,
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
        }

        if self.focused && self.cursor_visible {
            let cursor_y = canvas.y + self.padding + self.cursor_line as f32 * self.line_height
                - self.scroll_y;
            canvas.draw_list.push_rect(RectDraw {
                x: (canvas.x + self.padding + self.cursor_x).floor(),
                y: cursor_y,
                w: 1.5,
                h: self.line_height,
                color: [1.0, 1.0, 1.0, 1.0],
                radii: [0.0; 4],
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                rotate: canvas.rotate,
                scale_x: canvas.scale_x,
                scale_y: canvas.scale_y,
                opacity: canvas.opacity,
                clip,
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
