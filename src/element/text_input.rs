use crate::color::Color;
use crate::element::element::Element;
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::element::values::Size;
use crate::fonts::Fonts;
use crate::keyboard::{Key, Modifiers};
use crate::mouse::MouseButton;
use crate::ui::Ui;
use std::any::Any;
use std::cell::Cell;

pub struct TextInput {
    pub layout: Layout,
    pub text: String,
    pub placeholder: String,
    pub color: Color,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub border_color: Color,
    pub focused_border_color: Color,
    pub border_radius: f32,
    pub font_size: f32,
    pub font_weight: u16,
    pub font_family: String,
    pub focused: bool,
    pub cursor_pos: Cell<usize>,
    pub cursor_x: Cell<f32>,
    pub scroll_offset: Cell<f32>,
    pub text_h: Cell<f32>,
    pub inner_w: Cell<f32>,
    pub pending_click_x: Cell<f32>,
    pub has_pending_click: Cell<bool>,
}

impl TextInput {
    pub const TEXT_CHANGED: u32 = 0;
    pub const SUBMITTED: u32 = 1;
    pub const FOCUS_GAINED: u32 = 2;
    pub const FOCUS_LOST: u32 = 3;

    pub fn new(ui: &mut Ui, placeholder: &str) -> Handle<Self> {
        let mut layout = Layout::default();
        layout.padding = [8.0, 12.0, 8.0, 12.0];
        layout.width = Size::Fixed(200.0);
        ui.add(Self {
            layout,
            text: String::new(),
            placeholder: placeholder.to_string(),
            color: Color::hex("313244"),
            text_color: Color::hex("cdd6f4"),
            placeholder_color: Color::hex("6c7086"),
            border_color: Color::hex("45475a"),
            focused_border_color: Color::hex("89b4fa"),
            border_radius: 6.0,
            font_size: 16.0,
            font_weight: 400,
            font_family: "sans-serif".to_string(),
            focused: false,
            cursor_pos: Cell::new(0),
            cursor_x: Cell::new(0.0),
            scroll_offset: Cell::new(0.0),
            text_h: Cell::new(0.0),
            inner_w: Cell::new(0.0),
            pending_click_x: Cell::new(0.0),
            has_pending_click: Cell::new(false),
        })
    }
}

impl Element for TextInput {
    fn layout(&self) -> &Layout {
        &self.layout
    }
    fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }
    fn has_measure(&self) -> bool {
        true
    }

    fn measure(&self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)> {
        let pad = &self.layout.padding;
        let w = match self.layout.width {
            Size::Fixed(w) => w,
            _ => max_width.unwrap_or(200.0),
        };
        let inner_w = w - pad[1] - pad[3];
        self.inner_w.set(inner_w);

        let (_, th) = fonts.measure_sized(
            "A",
            &self.font_family,
            self.font_size,
            self.font_weight,
            false,
            None,
        );
        self.text_h.set(th);

        // measure cursor x position
        let cursor_text: String = self.text.chars().take(self.cursor_pos.get()).collect();
        let cx = if cursor_text.is_empty() {
            0.0
        } else {
            let (cw, _) = fonts.measure_sized(
                &cursor_text,
                &self.font_family,
                self.font_size,
                self.font_weight,
                false,
                None,
            );
            cw
        };
        self.cursor_x.set(cx);

        // update scroll offset to keep cursor visible
        let mut offset = self.scroll_offset.get();
        if cx - offset > inner_w - 4.0 {
            offset = cx - inner_w + 4.0;
        }
        if cx - offset < 0.0 {
            offset = cx;
        }
        self.scroll_offset.set(offset.max(0.0));

        // process pending click — find cursor position from click x
        if self.has_pending_click.get() {
            self.has_pending_click.set(false);
            let cx = self.pending_click_x.get();
            let mut best_pos = 0;
            let mut best_dist = f32::MAX;
            for i in 0..=self.text.chars().count() {
                let substr: String = self.text.chars().take(i).collect();
                let (w2, _) = fonts.measure_sized(
                    &substr,
                    &self.font_family,
                    self.font_size,
                    self.font_weight,
                    false,
                    None,
                );
                let dist = (w2 - cx).abs();
                if dist < best_dist {
                    best_dist = dist;
                    best_pos = i;
                }
            }
            self.cursor_pos.set(best_pos);
        }

        Some((w, th + pad[0] + pad[2]))
    }

    fn on_focus_gained(&mut self) -> Option<u32> {
        self.focused = true;
        self.cursor_pos.set(self.text.chars().count());
        Some(Self::FOCUS_GAINED)
    }

    fn on_focus_lost(&mut self) -> Option<u32> {
        self.focused = false;
        Some(Self::FOCUS_LOST)
    }

    fn on_mouse_press(&mut self, x: f32, _y: f32, button: MouseButton) -> Option<u32> {
        if let MouseButton::Left = button {
            let relative_x = x - self.layout.x - self.layout.padding[3] + self.scroll_offset.get();
            self.pending_click_x.set(relative_x);
            self.has_pending_click.set(true);
        }
        None
    }

    fn on_key_press(&mut self, key: Key, _modifiers: Modifiers, text: Option<char>) -> Option<u32> {
        match key {
            Key::Backspace => {
                if self.cursor_pos.get() > 0 {
                    let byte_pos = self
                        .text
                        .char_indices()
                        .nth(self.cursor_pos.get() - 1)
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    self.text.remove(byte_pos);
                    self.cursor_pos.set(self.cursor_pos.get() - 1);
                    return Some(Self::TEXT_CHANGED);
                }
                None
            }
            Key::Delete => {
                if self.cursor_pos.get() < self.text.chars().count() {
                    let byte_pos = self
                        .text
                        .char_indices()
                        .nth(self.cursor_pos.get())
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    self.text.remove(byte_pos);
                    return Some(Self::TEXT_CHANGED);
                }
                None
            }
            Key::Left => {
                if self.cursor_pos.get() > 0 {
                    self.cursor_pos.set(self.cursor_pos.get() - 1);
                }
                None
            }
            Key::Right => {
                if self.cursor_pos.get() < self.text.chars().count() {
                    self.cursor_pos.set(self.cursor_pos.get() + 1);
                }
                None
            }
            Key::Home => {
                self.cursor_pos.set(0);
                None
            }
            Key::End => {
                self.cursor_pos.set(self.text.chars().count());
                None
            }
            Key::Enter => Some(Self::SUBMITTED),
            _ => {
                if let Some(ch) = text {
                    if !ch.is_control() {
                        let byte_pos = self
                            .text
                            .char_indices()
                            .nth(self.cursor_pos.get())
                            .map(|(i, _)| i)
                            .unwrap_or(self.text.len());
                        self.text.insert(byte_pos, ch);
                        self.cursor_pos.set(self.cursor_pos.get() + 1);
                        return Some(Self::TEXT_CHANGED);
                    }
                }
                None
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
