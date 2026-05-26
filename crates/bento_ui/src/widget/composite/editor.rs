use crate::events::types::{
    FocusGained, FocusLost, KeyPress, MouseDown, MouseMove, MouseScroll, MouseUp,
};
use crate::layout::Size;
use crate::ui::TimerHandle;
use crate::widget::{Canvas, Widget, WidgetHandle};
use crate::{CursorIcon, HoverEnter, Key, Ui};
use bento_wgpu::{
    DecorationRange, RectDraw, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer,
};

pub struct Editor {
    // layout
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub width: Size,
    pub height: Size,
    pub z: i32,
    pub padding: f32,

    // appearance
    pub color: [f32; 4],
    pub background: [f32; 4],
    pub font_size: f32,
    pub font_family: String,
    pub line_height: f32,

    // content
    pub lines: Vec<String>,

    // cursor and selection
    cursor_line: usize,
    cursor_col: usize,
    selection_anchor: Option<(usize, usize)>,

    // focus and blink
    focused: bool,
    cursor_visible: bool,
    blink_handle: Option<TimerHandle>,

    // scroll
    pub wrap: bool,
    scroll_x: f32,
    scroll_y: f32,

    // measure cache, per logical line
    cached_inner_w: f32,
    line_visual_rows: Vec<usize>,
    line_dirty: Vec<bool>,
    // per line glyph positions for click/cursor navigation
    all_line_glyph_positions: Vec<Vec<Vec<f32>>>, // [line][visual_row][col]
    all_line_start_chars: Vec<Vec<usize>>,        // [line][visual_row]

    // cursor render state, computed in update)
    cursor_x: f32,
    cursor_visual_row: usize,
    current_visual_line_in_logical: usize,

    id: u64,

    resizing: bool,
    resize_start_mouse_x: f32,
    resize_start_mouse_y: f32,
    resize_start_w: f32,
    resize_start_h: f32,

    pub tab_width: usize,
    pub use_spaces: bool,

    line_widths: Vec<f32>,
    max_line_width: f32,
    max_line_width_dirty: bool,
}

impl Editor {
    pub fn new() -> Self {
        let font_size = 20.0;
        Self {
            x: 0.0,
            y: 0.0,
            w: 200.0,
            h: 200.0,
            width: Size::Fixed(200.0),
            height: Size::Fixed(200.0),
            z: 0,
            padding: 8.0,
            color: [1.0, 1.0, 1.0, 1.0],
            background: [0.02, 0.02, 0.02, 1.0],
            font_size,
            font_family: "JetBrainsMono Nerd Font".to_string(),
            line_height: font_size * 1.5,
            lines: vec![String::new()],
            cursor_line: 0,
            cursor_col: 0,
            selection_anchor: None,
            focused: false,
            cursor_visible: true,
            blink_handle: None,
            wrap: true,
            scroll_x: 0.0,
            scroll_y: 0.0,
            cached_inner_w: 0.0,
            line_visual_rows: vec![1],
            line_dirty: vec![true],
            all_line_glyph_positions: vec![vec![]],
            all_line_start_chars: vec![vec![]],
            cursor_x: 0.0,
            cursor_visual_row: 0,
            current_visual_line_in_logical: 0,
            id: 0,
            resizing: false,
            resize_start_mouse_x: 0.0,
            resize_start_mouse_y: 0.0,
            resize_start_w: 0.0,
            resize_start_h: 0.0,
            tab_width: 4,
            use_spaces: false,
            line_widths: vec![0.0],
            max_line_width: 0.0,
            max_line_width_dirty: false,
        }
    }

    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size;
        self.line_height = size * 1.5;
    }
}

impl Editor {
    fn near_resize_corner(&self, mx: f32, my: f32) -> bool {
        let corner_x = self.x + self.w;
        let corner_y = self.y + self.h;
        (mx - corner_x).abs() < 20.0 && (my - corner_y).abs() < 20.0
    }

    fn total_visual_rows(&self) -> usize {
        self.line_visual_rows.iter().sum()
    }

    fn visual_row_of_line(&self, line_idx: usize) -> usize {
        self.line_visual_rows[..line_idx].iter().sum()
    }

    fn mark_dirty(&mut self, line: usize) {
        if line < self.line_dirty.len() {
            self.line_dirty[line] = true;
        }
    }

    fn sync_cache_len(&mut self) {
        let n = self.lines.len();
        self.line_visual_rows.resize(n, 1);
        self.line_dirty.resize(n, true);
        self.all_line_glyph_positions.resize_with(n, Vec::new);
        self.all_line_start_chars.resize_with(n, Vec::new);
        self.line_visual_rows.truncate(n);
        self.line_dirty.truncate(n);
        self.all_line_glyph_positions.truncate(n);
        self.all_line_start_chars.truncate(n);
        self.line_widths.resize(n, 0.0);
        self.line_widths.truncate(n);
    }

    fn selection_range(&self) -> Option<((usize, usize), (usize, usize))> {
        let anchor = self.selection_anchor?;
        let cursor = (self.cursor_line, self.cursor_col);
        if anchor == cursor {
            return None;
        }
        Some(if anchor < cursor {
            (anchor, cursor)
        } else {
            (cursor, anchor)
        })
    }

    fn selected_text(&self) -> String {
        let Some(((sl, sc), (el, ec))) = self.selection_range() else {
            return String::new();
        };
        let mut out = String::new();
        for li in sl..=el {
            let line = &self.lines[li];
            let from = if li == sl { sc } else { 0 };
            let to = if li == el { ec } else { line.chars().count() };
            if li > sl {
                out.push('\n');
            }
            out.push_str(&line[char_to_byte(line, from)..char_to_byte(line, to)]);
        }
        out
    }

    fn delete_selection(&mut self) -> bool {
        let Some(((sl, sc), (el, ec))) = self.selection_range() else {
            return false;
        };
        if sl == el {
            let line = &mut self.lines[sl];
            let b0 = char_to_byte(line, sc);
            let b1 = char_to_byte(line, ec);
            line.drain(b0..b1);
            self.mark_dirty(sl);
        } else {
            let end_byte = char_to_byte(&self.lines[el], ec);
            let tail = self.lines[el][end_byte..].to_string();
            for li in (sl + 1..=el).rev() {
                self.lines.remove(li);
                self.line_visual_rows.remove(li);
                self.line_dirty.remove(li);
                self.all_line_glyph_positions.remove(li);
                self.all_line_start_chars.remove(li);
            }
            self.max_line_width_dirty = true;
            let truncate_at = char_to_byte(&self.lines[sl], sc);
            self.lines[sl].truncate(truncate_at);
            self.lines[sl].push_str(&tail);
            self.mark_dirty(sl);
        }
        self.cursor_line = sl;
        self.cursor_col = sc;
        self.selection_anchor = None;
        true
    }

    // convert pixel position to (logical_line, col)
    fn pos_to_cursor(&self, mx: f32, my: f32, editor_x: f32, editor_y: f32) -> (usize, usize) {
        let rel_y = my - editor_y - self.padding + self.scroll_y;
        let rel_x = mx - editor_x - self.padding + self.scroll_x;
        let visual_row =
            ((rel_y / self.line_height) as usize).min(self.total_visual_rows().saturating_sub(1));

        // find logical line and visual within logical from visual_row
        let mut cumulative = 0usize;
        let mut logical = self.lines.len().saturating_sub(1);
        let mut vwl = 0usize;
        for (i, &rows) in self.line_visual_rows.iter().enumerate() {
            if visual_row < cumulative + rows {
                logical = i;
                vwl = visual_row - cumulative;
                break;
            }
            cumulative += rows;
        }

        let positions = self
            .all_line_glyph_positions
            .get(logical)
            .and_then(|v| v.get(vwl))
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        let start = self
            .all_line_start_chars
            .get(logical)
            .and_then(|v| v.get(vwl))
            .copied()
            .unwrap_or(0);

        (logical, start + find_col_at_x(positions, rel_x))
    }
}

impl Editor {
    // returns true if the document/cursor changed and a redraw is needed
    fn handle_key(&mut self, e: &KeyPress, shift: bool, ctrl: bool) -> bool {
        if ctrl && e.key == Key::C {
            println!("[copy] {:?}", self.selected_text());
            return false;
        }

        if ctrl && e.key == Key::A {
            self.selection_anchor = Some((0, 0));
            self.cursor_line = self.lines.len() - 1;
            self.cursor_col = self.lines[self.cursor_line].chars().count();
            return true;
        }

        // set/extend selection anchor on shift, clear it otherwise
        // Called before moving the cursor
        let anchor = (self.cursor_line, self.cursor_col);

        match e.key {
            Key::Enter => {
                self.delete_selection();
                let byte = char_to_byte(&self.lines[self.cursor_line], self.cursor_col);
                let rest = self.lines[self.cursor_line][byte..].to_string();
                self.lines[self.cursor_line].truncate(byte);
                self.mark_dirty(self.cursor_line);
                self.cursor_line += 1;
                self.lines.insert(self.cursor_line, rest);
                self.line_visual_rows.insert(self.cursor_line, 1);
                self.line_dirty.insert(self.cursor_line, true);
                self.all_line_glyph_positions
                    .insert(self.cursor_line, vec![]);
                self.all_line_start_chars.insert(self.cursor_line, vec![]);
                self.cursor_col = 0;
                true
            }

            Key::Backspace => {
                if self.delete_selection() {
                    return true;
                }
                if self.cursor_col > 0 {
                    let line = &mut self.lines[self.cursor_line];
                    let b0 = char_to_byte(line, self.cursor_col - 1);
                    let b1 = char_to_byte(line, self.cursor_col);
                    line.drain(b0..b1);
                    self.cursor_col -= 1;
                    self.mark_dirty(self.cursor_line);
                    true
                } else if self.cursor_line > 0 {
                    let removed = self.lines.remove(self.cursor_line);
                    self.max_line_width_dirty = true;
                    self.line_visual_rows.remove(self.cursor_line);
                    self.line_dirty.remove(self.cursor_line);
                    self.all_line_glyph_positions.remove(self.cursor_line);
                    self.all_line_start_chars.remove(self.cursor_line);
                    self.cursor_line -= 1;
                    self.cursor_col = self.lines[self.cursor_line].chars().count();
                    self.lines[self.cursor_line].push_str(&removed);
                    self.mark_dirty(self.cursor_line);
                    true
                } else {
                    false
                }
            }

            Key::Delete => {
                if self.delete_selection() {
                    return true;
                }
                let len = self.lines[self.cursor_line].chars().count();
                if self.cursor_col < len {
                    let line = &mut self.lines[self.cursor_line];
                    let b0 = char_to_byte(line, self.cursor_col);
                    let b1 = char_to_byte(line, self.cursor_col + 1);
                    line.drain(b0..b1);
                    self.mark_dirty(self.cursor_line);
                    true
                } else if self.cursor_line < self.lines.len() - 1 {
                    let next = self.lines.remove(self.cursor_line + 1);
                    self.max_line_width_dirty = true;
                    self.line_visual_rows.remove(self.cursor_line + 1);
                    self.line_dirty.remove(self.cursor_line + 1);
                    self.all_line_glyph_positions.remove(self.cursor_line + 1);
                    self.all_line_start_chars.remove(self.cursor_line + 1);
                    self.lines[self.cursor_line].push_str(&next);
                    self.mark_dirty(self.cursor_line);
                    true
                } else {
                    false
                }
            }

            Key::Left => {
                if !shift {
                    if let Some(((sl, sc), _)) = self.selection_range() {
                        self.cursor_line = sl;
                        self.cursor_col = sc;
                        self.selection_anchor = None;
                        return true;
                    }
                    self.selection_anchor = None;
                } else if self.selection_anchor.is_none() {
                    self.selection_anchor = Some(anchor);
                }
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                    true
                } else if self.cursor_line > 0 {
                    self.cursor_line -= 1;
                    self.cursor_col = self.lines[self.cursor_line].chars().count();
                    true
                } else {
                    false
                }
            }

            Key::Right => {
                if !shift {
                    if let Some((_, (el, ec))) = self.selection_range() {
                        self.cursor_line = el;
                        self.cursor_col = ec;
                        self.selection_anchor = None;
                        return true;
                    }
                    self.selection_anchor = None;
                } else if self.selection_anchor.is_none() {
                    self.selection_anchor = Some(anchor);
                }
                let len = self.lines[self.cursor_line].chars().count();
                if self.cursor_col < len {
                    self.cursor_col += 1;
                    true
                } else if self.cursor_line < self.lines.len() - 1 {
                    self.cursor_line += 1;
                    self.cursor_col = 0;
                    true
                } else {
                    false
                }
            }

            Key::Up => {
                if shift {
                    if self.selection_anchor.is_none() {
                        self.selection_anchor = Some(anchor);
                    }
                } else {
                    self.selection_anchor = None;
                }
                if self.current_visual_line_in_logical > 0 {
                    let tv = self.current_visual_line_in_logical - 1;
                    let start = self
                        .all_line_start_chars
                        .get(self.cursor_line)
                        .and_then(|v| v.get(tv))
                        .copied()
                        .unwrap_or(0);
                    let pos = self
                        .all_line_glyph_positions
                        .get(self.cursor_line)
                        .and_then(|v| v.get(tv))
                        .map(|v| v.as_slice())
                        .unwrap_or(&[]);
                    self.cursor_col = start + find_col_at_x(pos, self.cursor_x);
                    true
                } else if self.cursor_line > 0 {
                    self.cursor_line -= 1;
                    let rows = self
                        .line_visual_rows
                        .get(self.cursor_line)
                        .copied()
                        .unwrap_or(1);
                    let tv = rows - 1;
                    let start = self
                        .all_line_start_chars
                        .get(self.cursor_line)
                        .and_then(|v| v.get(tv))
                        .copied()
                        .unwrap_or(0);
                    let pos = self
                        .all_line_glyph_positions
                        .get(self.cursor_line)
                        .and_then(|v| v.get(tv))
                        .map(|v| v.as_slice())
                        .unwrap_or(&[]);
                    self.cursor_col = start + find_col_at_x(pos, self.cursor_x);
                    true
                } else {
                    false
                }
            }

            Key::Down => {
                if shift {
                    if self.selection_anchor.is_none() {
                        self.selection_anchor = Some(anchor);
                    }
                } else {
                    self.selection_anchor = None;
                }
                let rows = self
                    .line_visual_rows
                    .get(self.cursor_line)
                    .copied()
                    .unwrap_or(1);
                if self.current_visual_line_in_logical + 1 < rows {
                    let tv = self.current_visual_line_in_logical + 1;
                    let start = self
                        .all_line_start_chars
                        .get(self.cursor_line)
                        .and_then(|v| v.get(tv))
                        .copied()
                        .unwrap_or(0);
                    let pos = self
                        .all_line_glyph_positions
                        .get(self.cursor_line)
                        .and_then(|v| v.get(tv))
                        .map(|v| v.as_slice())
                        .unwrap_or(&[]);
                    self.cursor_col = start + find_col_at_x(pos, self.cursor_x);
                    true
                } else if self.cursor_line < self.lines.len() - 1 {
                    self.cursor_line += 1;
                    self.cursor_col = self
                        .cursor_col
                        .min(self.lines[self.cursor_line].chars().count());
                    self.current_visual_line_in_logical = 0;
                    true
                } else {
                    false
                }
            }

            Key::Home => {
                if shift {
                    if self.selection_anchor.is_none() {
                        self.selection_anchor = Some(anchor);
                    }
                } else {
                    self.selection_anchor = None;
                }
                if self.cursor_col != 0 {
                    self.cursor_col = 0;
                    true
                } else {
                    false
                }
            }

            Key::End => {
                if shift {
                    if self.selection_anchor.is_none() {
                        self.selection_anchor = Some(anchor);
                    }
                } else {
                    self.selection_anchor = None;
                }
                let len = self.lines[self.cursor_line].chars().count();
                if self.cursor_col != len {
                    self.cursor_col = len;
                    true
                } else {
                    false
                }
            }

            Key::Tab => {
                self.delete_selection();
                if self.use_spaces {
                    for _ in 0..self.tab_width {
                        let byte = char_to_byte(&self.lines[self.cursor_line], self.cursor_col);
                        self.lines[self.cursor_line].insert(byte, ' ');
                        self.cursor_col += 1;
                    }
                } else {
                    let byte = char_to_byte(&self.lines[self.cursor_line], self.cursor_col);
                    self.lines[self.cursor_line].insert(byte, '\t');
                    self.cursor_col += 1;
                }
                self.mark_dirty(self.cursor_line);
                true
            }

            _ => {
                if let Some(ch) = e.ch {
                    if !ch.is_control() {
                        self.delete_selection();
                        let byte = char_to_byte(&self.lines[self.cursor_line], self.cursor_col);
                        self.lines[self.cursor_line].insert(byte, ch);
                        self.cursor_col += 1;
                        self.mark_dirty(self.cursor_line);
                        return true;
                    }
                }
                false
            }
        }
    }
}

fn blink_tick(ui: &mut Ui, handle: WidgetHandle<Editor>) {
    let h = ui.asyncs.timer(0.53, move |ui| {
        if let Some(e) = ui.get_mut(handle) {
            if e.focused {
                e.cursor_visible = !e.cursor_visible;
                ui.request_redraw();
                blink_tick(ui, handle);
            }
        }
    });
    if let Some(e) = ui.get_mut(handle) {
        e.blink_handle = Some(h);
    }
}

fn start_blink(ui: &mut Ui, handle: WidgetHandle<Editor>) {
    if let Some(e) = ui.get_mut(handle) {
        e.cursor_visible = true;
        if let Some(h) = e.blink_handle.take() {
            h.cancel();
        }
    }
    ui.request_update(handle);
    ui.request_redraw();
    blink_tick(ui, handle);
}

impl Widget for Editor {
    fn name(&self) -> &str {
        "Editor"
    }

    fn build(&mut self, ui: &mut Ui, handle: WidgetHandle<()>) {
        self.id = handle.id as u64;
        let handle = handle.typed::<Editor>();

        ui.listen(handle, move |_: &FocusGained, ui: &mut Ui| {
            if let Some(e) = ui.get_mut(handle) {
                e.focused = true;
            }
            start_blink(ui, handle);
        });

        ui.listen(handle, move |_: &FocusLost, ui: &mut Ui| {
            if let Some(e) = ui.get_mut(handle) {
                e.focused = false;
                e.cursor_visible = true;
                if let Some(h) = e.blink_handle.take() {
                    h.cancel();
                }
            }
            ui.request_update(handle);
        });

        ui.listen(handle, move |_: &HoverEnter, ui: &mut Ui| {
            ui.set_cursor(CursorIcon::Text);
        });

        ui.listen(handle, move |ev: &KeyPress, ui: &mut Ui| {
            let shift = ui.input.keyboard.modifiers.shift;
            let ctrl = ui.input.keyboard.modifiers.ctrl;
            if let Some(e) = ui.get_mut(handle) {
                if e.handle_key(ev, shift, ctrl) {
                    start_blink(ui, handle);
                }
            }
        });

        ui.listen(handle, move |ev: &MouseDown, ui: &mut Ui| {
            let click_count = ui.input.mouse.left.click_count;
            if let Some(e) = ui.get_mut(handle) {
                if e.near_resize_corner(ev.x, ev.y) {
                    e.resizing = true;
                    e.resize_start_mouse_x = ev.x;
                    e.resize_start_mouse_y = ev.y;
                    e.resize_start_w = e.w;
                    e.resize_start_h = e.h;
                    ui.capture_mouse(handle);
                    return;
                }
                let (line, col) = e.pos_to_cursor(ev.x, ev.y, e.x, e.y);
                e.cursor_line = line;
                e.cursor_col = col;
                if click_count == 2 {
                    let text = &e.lines[line];
                    let start = word_start(text, col);
                    let end = word_end(text, col);
                    e.selection_anchor = Some((line, start));
                    e.cursor_col = end;
                } else if click_count >= 3 {
                    e.selection_anchor = Some((line, 0));
                    e.cursor_col = e.lines[line].chars().count();
                } else {
                    e.selection_anchor = Some((line, col));
                }
            }
            ui.capture_mouse(handle);
            start_blink(ui, handle);
        });

        ui.listen(handle, move |ev: &MouseMove, ui: &mut Ui| {
            let left_pressed = ui.input.mouse.left.pressed;
            if let Some(e) = ui.get_mut(handle) {
                if e.resizing {
                    let dx = ev.x - e.resize_start_mouse_x;
                    let dy = ev.y - e.resize_start_mouse_y;
                    e.w = (e.resize_start_w + dx).max(100.0);
                    e.h = (e.resize_start_h + dy).max(60.0);
                    e.width = Size::Fixed(e.w);
                    e.height = Size::Fixed(e.h);
                    ui.request_layout(handle);
                    ui.request_update(handle);
                    ui.request_redraw();
                    return;
                }
                let near_corner = e.near_resize_corner(ev.x, ev.y);
                let new_col = if left_pressed {
                    Some(e.pos_to_cursor(ev.x, ev.y, e.x, e.y))
                } else {
                    None
                };
                if let Some((line, col)) = new_col {
                    e.cursor_line = line;
                    e.cursor_col = col;
                }
                if near_corner {
                    ui.set_cursor(CursorIcon::ResizeNwSe);
                } else {
                    ui.set_cursor(CursorIcon::Text);
                }
            }
            ui.request_update(handle);
            ui.request_redraw();
        });

        ui.listen(handle, move |_: &MouseUp, ui: &mut Ui| {
            if let Some(e) = ui.get_mut(handle) {
                if e.resizing {
                    e.resizing = false;
                    ui.release_mouse();
                    return;
                }
                if e.selection_anchor == Some((e.cursor_line, e.cursor_col)) {
                    e.selection_anchor = None;
                }
            }
            ui.release_mouse();
        });

        ui.listen(handle, move |ev: &MouseScroll, ui: &mut Ui| {
            if let Some(e) = ui.get_mut(handle) {
                let inner_h = e.h - e.padding * 2.0;
                let max_scroll = (e.total_visual_rows() as f32 * e.line_height - inner_h).max(0.0);
                e.scroll_y = (e.scroll_y - ev.y * e.line_height * 3.0).clamp(0.0, max_scroll);
                if !e.wrap {
                    let inner_w = e.w - e.padding * 2.0;
                    let max_scroll_x = (e.max_line_width - inner_w).max(0.0);
                    e.scroll_x = (e.scroll_x - ev.x * 20.0).clamp(0.0, max_scroll_x);
                }
                ui.request_redraw();
            }
        });
    }

    fn update(&mut self, measurer: &mut TextMeasurer) {
        let inner_w = self.w - self.padding * 2.0;

        if (inner_w - self.cached_inner_w).abs() > 0.1 {
            self.cached_inner_w = inner_w;
            self.line_dirty.iter_mut().for_each(|d| *d = true);
            self.max_line_width = 0.0;
            self.max_line_width_dirty = false;
        }

        self.sync_cache_len();

        let mut cursor_result = None;

        for i in 0..self.lines.len() {
            if !self.line_dirty[i] {
                continue;
            }
            let result = measurer.measure_reuse(
                self.id,
                TextMeasureRequest {
                    text: if self.lines[i].is_empty() {
                        " "
                    } else {
                        &self.lines[i]
                    },
                    font_family: &self.font_family,
                    size: self.font_size,
                    weight: 400,
                    italic: false,
                    letter_spacing: 0.0,
                    line_height: Some(self.line_height),
                    tab_width: self.tab_width as u16,
                    max_width: if self.wrap { Some(inner_w) } else { None },
                    weight_ranges: &[],
                    italic_ranges: &[],
                    font_family_ranges: &[],
                },
            );
            self.line_visual_rows[i] = result.line_count.max(1);
            self.all_line_glyph_positions[i] = result.line_glyph_positions.clone();
            self.all_line_start_chars[i] = result.line_start_chars.clone();
            self.line_dirty[i] = false;
            self.line_widths[i] = result.width;
            if !self.max_line_width_dirty {
                self.max_line_width = self.max_line_width.max(result.width);
            }
            if i == self.cursor_line {
                cursor_result = Some(result);
            }
        }

        let cursor_line = self.cursor_line;
        if self.max_line_width_dirty {
            self.max_line_width = self.line_widths.iter().cloned().fold(0.0, f32::max);
            self.max_line_width_dirty = false;
        }
        let result = cursor_result.unwrap_or_else(|| {
            measurer.measure_reuse(
                self.id,
                TextMeasureRequest {
                    text: if self.lines[cursor_line].is_empty() {
                        " "
                    } else {
                        &self.lines[cursor_line]
                    },
                    font_family: &self.font_family,
                    size: self.font_size,
                    weight: 400,
                    italic: false,
                    letter_spacing: 0.0,
                    line_height: Some(self.line_height),
                    tab_width: self.tab_width as u16,
                    max_width: if self.wrap { Some(inner_w) } else { None },
                    weight_ranges: &[],
                    italic_ranges: &[],
                    font_family_ranges: &[],
                },
            )
        });

        // always refresh cursor line positions from the result
        let cl = self.cursor_line;
        self.all_line_glyph_positions[cl] = result.line_glyph_positions.clone();
        self.all_line_start_chars[cl] = result.line_start_chars.clone();

        let mut visual_in_logical = result.line_start_chars.len().saturating_sub(1);
        for (vi, &start) in result.line_start_chars.iter().enumerate() {
            let next = result
                .line_start_chars
                .get(vi + 1)
                .copied()
                .unwrap_or(usize::MAX);
            if self.cursor_col >= start
                && (self.cursor_col < next || vi + 1 == result.line_start_chars.len())
            {
                visual_in_logical = vi;
                break;
            }
        }
        self.current_visual_line_in_logical = visual_in_logical;

        let col_in_visual = self.cursor_col
            - result
                .line_start_chars
                .get(visual_in_logical)
                .copied()
                .unwrap_or(0);

        self.cursor_x = result
            .line_glyph_positions
            .get(visual_in_logical)
            .and_then(|p| p.get(col_in_visual).or_else(|| p.last()))
            .copied()
            .unwrap_or(0.0);

        let max_scroll_x = (self.max_line_width - inner_w).max(0.0);
        let cursor_screen_x = self.cursor_x - self.scroll_x;
        if cursor_screen_x < 0.0 {
            self.scroll_x = self.cursor_x;
        } else if cursor_screen_x > inner_w {
            self.scroll_x = self.cursor_x - inner_w;
        }
        self.scroll_x = self.scroll_x.clamp(0.0, max_scroll_x);

        self.cursor_visual_row = self.visual_row_of_line(self.cursor_line) + visual_in_logical;

        let inner_h = self.h - self.padding * 2.0;
        let top = self.cursor_visual_row as f32 * self.line_height;
        let bottom = top + self.line_height;
        if top < self.scroll_y {
            self.scroll_y = top;
        } else if bottom > self.scroll_y + inner_h {
            self.scroll_y = bottom - inner_h;
        }
        let max_scroll = (self.total_visual_rows() as f32 * self.line_height - inner_h).max(0.0);
        self.scroll_y = self.scroll_y.clamp(0.0, max_scroll);
    }

    fn render(&self, canvas: &mut Canvas) {
        // background + border
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
        let sel = self.selection_range();
        let mut vrow = 0usize;

        for (li, line) in self.lines.iter().enumerate() {
            let rows = self.line_visual_rows[li];
            let top = canvas.y + self.padding + vrow as f32 * self.line_height - self.scroll_y;
            let bot = top + rows as f32 * self.line_height;

            // skip fully off-screen lines
            if bot >= canvas.y && top <= canvas.y + self.h {
                let background_ranges = sel
                    .and_then(|((sl, sc), (el, ec))| {
                        if li < sl || li > el {
                            return None;
                        }
                        let from = if li == sl { sc } else { 0 };
                        let to = if li == el { ec } else { line.chars().count() };
                        // selection color
                        Some(vec![DecorationRange {
                            start: from,
                            end: to,
                            color: [0.196, 0.592, 0.992, 1.0],
                        }])
                    })
                    .unwrap_or_default();

                // skip pushing a TextDraw entirely for empty lines that have no selection on them
                if line.is_empty() && background_ranges.is_empty() {
                    vrow += rows;
                    continue;
                }

                canvas.draw_list.push_text(TextDraw {
                    x: canvas.x + self.padding - self.scroll_x,
                    y: top,
                    w: inner_w + self.scroll_x,
                    h: self.line_height * rows as f32,
                    text: line.clone(),
                    size: self.font_size,
                    color: self.color,
                    weight: 400,
                    italic: false,
                    font_family: self.font_family.clone(),
                    max_width: if self.wrap { Some(inner_w) } else { None },
                    line_height: Some(self.line_height),
                    tab_width: self.tab_width as u16,
                    letter_spacing: 0.0,
                    align: TextAlign::Left,
                    opacity: canvas.opacity,
                    clip,
                    rotate: canvas.rotate,
                    scale_x: canvas.scale_x,
                    scale_y: canvas.scale_y,
                    z: canvas.z + 1,
                    color_ranges: vec![],
                    background_ranges,
                    underline_ranges: vec![],
                    strikethrough_ranges: vec![],
                    weight_ranges: vec![],
                    italic_ranges: vec![],
                    font_family_ranges: vec![],
                });
            }

            vrow += rows;
        }

        // cursor
        if self.focused && self.cursor_visible {
            let cy = canvas.y + self.padding + self.cursor_visual_row as f32 * self.line_height
                - self.scroll_y;
            canvas.draw_list.push_rect(RectDraw {
                x: (canvas.x + self.padding + self.cursor_x - self.scroll_x).floor(),
                y: cy,
                w: 1.0,
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
                z: canvas.z + 3,
            });
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
}

fn char_to_byte(text: &str, char_idx: usize) -> usize {
    text.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(text.len())
}

fn find_col_at_x(positions: &[f32], target_x: f32) -> usize {
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

fn word_start(text: &str, col: usize) -> usize {
    let chars: Vec<char> = text.chars().collect();
    if col == 0 || chars.is_empty() {
        return 0;
    }
    let mut i = col.min(chars.len()).saturating_sub(1);
    if !chars[i].is_alphanumeric() && chars[i] != '_' {
        return col;
    }
    while i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_') {
        i -= 1;
    }
    i
}

fn word_end(text: &str, col: usize) -> usize {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return 0;
    }
    let mut i = col.min(chars.len());
    if i >= chars.len() {
        return chars.len();
    }
    if !chars[i].is_alphanumeric() && chars[i] != '_' {
        return col;
    }
    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
        i += 1;
    }
    i
}
