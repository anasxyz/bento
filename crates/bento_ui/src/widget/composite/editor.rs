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
    cursor_visual_row: usize,
    scroll_y: f32,
    line_visual_rows: Vec<usize>,
    line_dirty: Vec<bool>,
    cached_inner_w: f32,
    cached_line_glyph_positions: Vec<Vec<f32>>,
    cached_line_start_chars: Vec<usize>,
    current_visual_line_in_logical: usize,
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
            cursor_visual_row: 0,
            scroll_y: 0.0,
            line_visual_rows: vec![1],
            line_dirty: vec![true],
            cached_inner_w: 0.0,
            cached_line_glyph_positions: Vec::new(),
            cached_line_start_chars: Vec::new(),
            current_visual_line_in_logical: 0,
        }
    }

    fn mark_line_dirty(&mut self, line: usize) {
        if line < self.line_dirty.len() {
            self.line_dirty[line] = true;
        }
    }

    fn ensure_cache_size(&mut self) {
        while self.line_visual_rows.len() < self.lines.len() {
            self.line_visual_rows.push(1);
            self.line_dirty.push(true);
        }
        self.line_visual_rows.truncate(self.lines.len());
        self.line_dirty.truncate(self.lines.len());
    }

    fn visual_row_of_line(&self, line_idx: usize) -> usize {
        self.line_visual_rows[..line_idx].iter().sum()
    }

    fn total_visual_rows(&self) -> usize {
        self.line_visual_rows.iter().sum()
    }

    fn handle_key(&mut self, e: &KeyPress) -> bool {
        match e.key {
            Key::Enter => {
                let line = &self.lines[self.cursor_line];
                let byte_idx = char_to_byte(line, self.cursor_col);
                let rest = line[byte_idx..].to_string();
                self.lines[self.cursor_line].truncate(byte_idx);
                self.mark_line_dirty(self.cursor_line);
                self.cursor_line += 1;
                self.lines.insert(self.cursor_line, rest);
                self.line_visual_rows.insert(self.cursor_line, 1);
                self.line_dirty.insert(self.cursor_line, true);
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
                    self.mark_line_dirty(self.cursor_line);
                    return true;
                } else if self.cursor_line > 0 {
                    let line = self.lines.remove(self.cursor_line);
                    self.line_visual_rows.remove(self.cursor_line);
                    self.line_dirty.remove(self.cursor_line);
                    self.cursor_line -= 1;
                    self.cursor_col = self.lines[self.cursor_line].chars().count();
                    self.lines[self.cursor_line].push_str(&line);
                    self.mark_line_dirty(self.cursor_line);
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
                    self.mark_line_dirty(self.cursor_line);
                    return true;
                } else if self.cursor_line < self.lines.len() - 1 {
                    let next = self.lines.remove(self.cursor_line + 1);
                    self.line_visual_rows.remove(self.cursor_line + 1);
                    self.line_dirty.remove(self.cursor_line + 1);
                    self.lines[self.cursor_line].push_str(&next);
                    self.mark_line_dirty(self.cursor_line);
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
                if self.current_visual_line_in_logical > 0 {
                    // move up within same logical line
                    let target_visual = self.current_visual_line_in_logical - 1;
                    let start = self
                        .cached_line_start_chars
                        .get(target_visual)
                        .copied()
                        .unwrap_or(0);
                    let col_in_row = find_col_at_x(
                        self.cached_line_glyph_positions
                            .get(target_visual)
                            .map(|v| v.as_slice())
                            .unwrap_or(&[]),
                        self.cursor_x,
                    );
                    self.cursor_col = start + col_in_row;
                    return true;
                } else if self.cursor_line > 0 {
                    self.cursor_line -= 1;
                    let line_len = self.lines[self.cursor_line].chars().count();
                    self.cursor_col = self.cursor_col.min(line_len);
                    return true;
                }
            }
            Key::Down => {
                let visual_rows_in_line = self
                    .line_visual_rows
                    .get(self.cursor_line)
                    .copied()
                    .unwrap_or(1);
                if self.current_visual_line_in_logical + 1 < visual_rows_in_line {
                    // move down within same logical line
                    let target_visual = self.current_visual_line_in_logical + 1;
                    let start = self
                        .cached_line_start_chars
                        .get(target_visual)
                        .copied()
                        .unwrap_or(0);
                    let col_in_row = find_col_at_x(
                        self.cached_line_glyph_positions
                            .get(target_visual)
                            .map(|v| v.as_slice())
                            .unwrap_or(&[]),
                        self.cursor_x,
                    );
                    self.cursor_col = start + col_in_row;
                    return true;
                } else if self.cursor_line < self.lines.len() - 1 {
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
                        self.mark_line_dirty(self.cursor_line);
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn find_col_at_x(positions: &[f32], target_x: f32) -> usize {
    if positions.is_empty() {
        return 0;
    }
    positions
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            (**a - target_x)
                .abs()
                .partial_cmp(&(**b - target_x).abs())
                .unwrap()
        })
        .map(|(i, _)| i)
        .unwrap_or(0)
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
                let total = input.total_visual_rows();
                let max_scroll =
                    (total as f32 * input.line_height - (input.h - input.padding * 2.0)).max(0.0);
                input.scroll_y =
                    (input.scroll_y - e.y * input.line_height * 3.0).clamp(0.0, max_scroll);
                ui.needs_redraw = true;
            }
        });
    }

    fn update(&mut self, measurer: &mut TextMeasurer) {
        let inner_w = self.w - self.padding * 2.0;

        // if width changed, mark all lines dirty
        if (inner_w - self.cached_inner_w).abs() > 0.1 {
            self.cached_inner_w = inner_w;
            for d in &mut self.line_dirty {
                *d = true;
            }
        }

        self.ensure_cache_size();

        // remeasure only dirty lines to update visual row counts
        for i in 0..self.lines.len() {
            if !self.line_dirty[i] {
                continue;
            }
            let line = &self.lines[i];
            let result = measurer.measure(TextMeasureRequest {
                text: if line.is_empty() { " " } else { line },
                font_family: "",
                size: self.font_size,
                weight: 400,
                italic: false,
                letter_spacing: 0.0,
                line_height: None,
                max_width: Some(inner_w),
                weight_ranges: &[],
                italic_ranges: &[],
                font_family_ranges: &[],
            });
            self.line_visual_rows[i] = result.line_count.max(1);
            self.line_dirty[i] = false;
        }

        // measure current line for cursor position — always a cache hit
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
            max_width: Some(inner_w),
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });

        // store for up/down navigation
        self.cached_line_glyph_positions = result.line_glyph_positions.clone();
        self.cached_line_start_chars = result.line_start_chars.clone();

        // find which visual line the cursor is on using line_start_chars
        let mut visual_line_in_logical = result.line_start_chars.len().saturating_sub(1);
        let mut col_in_visual = self.cursor_col;

        for (vi, &start_char) in result.line_start_chars.iter().enumerate() {
            let next_start = result
                .line_start_chars
                .get(vi + 1)
                .copied()
                .unwrap_or(usize::MAX);
            if self.cursor_col >= start_char && self.cursor_col < next_start {
                visual_line_in_logical = vi;
                col_in_visual = self.cursor_col - start_char;
                break;
            }
        }

        self.current_visual_line_in_logical = visual_line_in_logical;

        self.cursor_x = result
            .line_glyph_positions
            .get(visual_line_in_logical)
            .and_then(|p| p.get(col_in_visual))
            .copied()
            .unwrap_or(result.width);

        self.cursor_visual_row = self.visual_row_of_line(self.cursor_line) + visual_line_in_logical;

        // scroll to keep cursor visible
        let inner_h = self.h - self.padding * 2.0;
        let cursor_top = self.cursor_visual_row as f32 * self.line_height;
        let cursor_bottom = cursor_top + self.line_height;
        if cursor_top < self.scroll_y {
            self.scroll_y = cursor_top;
        } else if cursor_bottom > self.scroll_y + inner_h {
            self.scroll_y = cursor_bottom - inner_h;
        }
        let max_scroll = (self.total_visual_rows() as f32 * self.line_height - inner_h).max(0.0);
        self.scroll_y = self.scroll_y.clamp(0.0, max_scroll);
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
        let inner_w = self.w - self.padding * 2.0;
        let mut visual_row = 0usize;

        for (line_idx, line) in self.lines.iter().enumerate() {
            let visual_rows_for_line = self.line_visual_rows[line_idx];
            for vr in 0..visual_rows_for_line {
                let row_y =
                    canvas.y + self.padding + visual_row as f32 * self.line_height - self.scroll_y;

                if row_y + self.line_height >= canvas.y && row_y <= canvas.y + self.h {
                    // only draw on the first visual row — cosmic-text handles wrapping internally
                    if vr == 0 {
                        canvas.draw_list.push_text(TextDraw {
                            x: canvas.x + self.padding,
                            y: row_y,
                            w: inner_w,
                            h: self.line_height * visual_rows_for_line as f32,
                            text: line.clone(),
                            size: self.font_size,
                            color: self.color,
                            weight: 400,
                            italic: false,
                            font_family: String::new(),
                            max_width: Some(inner_w),
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
                }
                visual_row += 1;
            }
        }

        if self.focused && self.cursor_visible {
            let cursor_y =
                canvas.y + self.padding + self.cursor_visual_row as f32 * self.line_height
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
