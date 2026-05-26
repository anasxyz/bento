use crate::events::types::{FocusGained, FocusLost, KeyPress, MouseDown, MouseMove, MouseUp};
use crate::layout::Size;
use crate::ui::TimerHandle;
use crate::widget::{Canvas, Widget, WidgetHandle};
use crate::{CursorIcon, HoverEnter, Key, Ui};
use bento_wgpu::{
    DecorationRange, RectDraw, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer,
};

pub struct LineInput {
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
    selection_anchor: Option<usize>,
    focused: bool,
    text_w: f32,
    text_h: f32,
    cursor_x: f32,
    scroll_offset: f32,
    blink_handle: Option<TimerHandle>,
    cursor_visible: bool,
    glyph_positions: Vec<f32>,
    screen_x: f32,
    screen_y: f32,
}

impl LineInput {
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
            selection_anchor: None,
            focused: false,
            text_w: 0.0,
            text_h: 0.0,
            cursor_x: 0.0,
            scroll_offset: 0.0,
            blink_handle: None,
            cursor_visible: true,
            glyph_positions: Vec::new(),
            screen_x: 0.0,
            screen_y: 0.0,
        }
    }

    fn selection_range(&self) -> Option<(usize, usize)> {
        let anchor = self.selection_anchor?;
        if anchor == self.cursor {
            return None;
        }
        Some(if anchor < self.cursor {
            (anchor, self.cursor)
        } else {
            (self.cursor, anchor)
        })
    }

    fn delete_selection(&mut self) -> bool {
        let Some((start, end)) = self.selection_range() else {
            return false;
        };
        let b0 = char_to_byte(&self.value, start);
        let b1 = char_to_byte(&self.value, end);
        self.value.drain(b0..b1);
        self.cursor = start;
        self.selection_anchor = None;
        true
    }

    fn pos_to_col(&self, mx: f32) -> usize {
        let rel_x = mx - self.screen_x - self.padding + self.scroll_offset;
        find_col_at_x(&self.glyph_positions, rel_x)
    }

    fn handle_key(&mut self, e: &KeyPress, shift: bool, ctrl: bool) -> bool {
        if ctrl && e.key == Key::A {
            self.selection_anchor = Some(0);
            self.cursor = self.value.chars().count();
            return true;
        }

        let anchor = self.cursor;

        match e.key {
            Key::Backspace => {
                if self.delete_selection() {
                    return true;
                }
                if self.cursor > 0 {
                    let b0 = char_to_byte(&self.value, self.cursor - 1);
                    let b1 = char_to_byte(&self.value, self.cursor);
                    self.value.drain(b0..b1);
                    self.cursor -= 1;
                    return true;
                }
            }
            Key::Delete => {
                if self.delete_selection() {
                    return true;
                }
                let len = self.value.chars().count();
                if self.cursor < len {
                    let b0 = char_to_byte(&self.value, self.cursor);
                    let b1 = char_to_byte(&self.value, self.cursor + 1);
                    self.value.drain(b0..b1);
                    return true;
                }
            }
            Key::Left => {
                if !shift {
                    if let Some((start, _)) = self.selection_range() {
                        self.cursor = start;
                        self.selection_anchor = None;
                        return true;
                    }
                    self.selection_anchor = None;
                } else if self.selection_anchor.is_none() {
                    self.selection_anchor = Some(anchor);
                }
                if self.cursor > 0 {
                    self.cursor -= 1;
                    return true;
                }
            }
            Key::Right => {
                if !shift {
                    if let Some((_, end)) = self.selection_range() {
                        self.cursor = end;
                        self.selection_anchor = None;
                        return true;
                    }
                    self.selection_anchor = None;
                } else if self.selection_anchor.is_none() {
                    self.selection_anchor = Some(anchor);
                }
                let len = self.value.chars().count();
                if self.cursor < len {
                    self.cursor += 1;
                    return true;
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
                if self.cursor != 0 {
                    self.cursor = 0;
                    return true;
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
                let len = self.value.chars().count();
                if self.cursor != len {
                    self.cursor = len;
                    return true;
                }
            }
            _ => {
                if let Some(ch) = e.ch {
                    if !ch.is_control() {
                        self.delete_selection();
                        let byte_idx = char_to_byte(&self.value, self.cursor);
                        self.value.insert(byte_idx, ch);
                        self.cursor += 1;
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn blink_tick(ui: &mut Ui, handle: WidgetHandle<LineInput>) {
    let h = ui.asyncs.timer(0.53, move |ui| {
        if let Some(input) = ui.get_mut(handle) {
            if input.focused {
                input.cursor_visible = !input.cursor_visible;
                ui.request_redraw();
                blink_tick(ui, handle);
            }
        }
    });
    if let Some(input) = ui.get_mut(handle) {
        input.blink_handle = Some(h);
    }
}

fn start_blink(ui: &mut Ui, handle: WidgetHandle<LineInput>) {
    if let Some(input) = ui.get_mut(handle) {
        input.cursor_visible = true;
        if let Some(h) = input.blink_handle.take() {
            h.cancel();
        }
    }
    ui.request_update(handle);
    ui.request_redraw();
    blink_tick(ui, handle);
}

impl Widget for LineInput {
    fn name(&self) -> &str {
        "LineInput"
    }

    fn build(&mut self, ui: &mut Ui, handle: WidgetHandle<()>) {
        let handle = handle.typed::<LineInput>();

        ui.listen(handle, move |_: &FocusGained, ui: &mut Ui| {
            if let Some(input) = ui.get_mut(handle) {
                input.focused = true;
                input.cursor_visible = true;
                if let Some(h) = input.blink_handle.take() {
                    h.cancel();
                }
            }
            blink_tick(ui, handle);
        });

        ui.listen(handle, move |_: &FocusLost, ui: &mut Ui| {
            if let Some(input) = ui.get_mut(handle) {
                input.focused = false;
                input.cursor_visible = true;
                input.selection_anchor = None;
                if let Some(h) = input.blink_handle.take() {
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
            if let Some(input) = ui.get_mut(handle) {
                if input.handle_key(ev, shift, ctrl) {
                    start_blink(ui, handle);
                }
            }
        });

        ui.listen(handle, move |ev: &MouseDown, ui: &mut Ui| {
            let click_count = ui.input.mouse.left.click_count;
            if let Some(input) = ui.get_mut(handle) {
                let col = input.pos_to_col(ev.x);
                if click_count >= 3 {
                    input.selection_anchor = Some(0);
                    input.cursor = input.value.chars().count();
                } else if click_count == 2 {
                    let start = word_start(&input.value, col);
                    let end = word_end(&input.value, col);
                    input.selection_anchor = Some(start);
                    input.cursor = end;
                } else {
                    input.cursor = col;
                    input.selection_anchor = Some(col);
                }
            }
            ui.capture_mouse(handle);
            start_blink(ui, handle);
        });

        ui.listen(handle, move |ev: &MouseMove, ui: &mut Ui| {
            let left_pressed = ui.input.mouse.left.pressed;
            if let Some(input) = ui.get_mut(handle) {
                if left_pressed {
                    input.cursor = input.pos_to_col(ev.x);
                }
            }
            ui.request_update(handle);
            ui.request_redraw();
        });

        ui.listen(handle, move |_: &MouseUp, ui: &mut Ui| {
            if let Some(input) = ui.get_mut(handle) {
                if input.selection_anchor == Some(input.cursor) {
                    input.selection_anchor = None;
                }
            }
            ui.release_mouse();
        });
    }

    fn update(&mut self, measurer: &mut TextMeasurer) {
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
            tab_width: 4,
            max_width: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        self.text_h = result.height;
        self.text_w = result.width;
        self.glyph_positions = result.glyph_positions.clone();

        if matches!(self.height, Size::Auto) {
            self.h = self.text_h + self.padding * 2.0;
        }
        if matches!(self.width, Size::Auto) {
            self.w = (result.width + self.padding * 2.0).max(100.0);
        }

        self.cursor_x = result
            .glyph_positions
            .get(self.cursor)
            .copied()
            .unwrap_or(result.width);

        let inner_w = self.w - self.padding * 2.0;
        let cursor_local = self.cursor_x - self.scroll_offset;
        if cursor_local < 0.0 {
            self.scroll_offset = self.cursor_x;
        } else if cursor_local > inner_w {
            self.scroll_offset = self.cursor_x - inner_w;
        }
        let max_scroll = (self.text_w - inner_w).max(0.0);
        self.scroll_offset = self.scroll_offset.clamp(0.0, max_scroll);
    }

    fn render(&mut self, canvas: &mut Canvas) {
        self.screen_x = canvas.x;
        self.screen_y = canvas.y;

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

        let sel = self.selection_range();
        let background_ranges = sel
            .map(|(start, end)| {
                vec![DecorationRange {
                    start,
                    end,
                    color: [0.196, 0.592, 0.992, 1.0],
                }]
            })
            .unwrap_or_default();

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
            tab_width: 4,
            letter_spacing: 0.0,
            align: TextAlign::Left,
            opacity: canvas.opacity,
            clip: Some([canvas.x, canvas.y, self.w, self.h]),
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

        if self.focused && self.cursor_visible {
            canvas.draw_list.push_rect(RectDraw {
                x: (canvas.x + self.padding + self.cursor_x - self.scroll_offset).floor(),
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
