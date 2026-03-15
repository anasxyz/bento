use crate::color::Color;
use crate::element::element::Element;
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::fonts::Fonts;
use crate::ui::Ui;
use std::any::Any;
use std::cell::Cell;

pub struct Button {
    pub layout: Layout,
    pub text: String,
    pub color: Color,
    pub border_radius: f32,
    pub font_size: f32,
    pub font_weight: u16,
    pub(crate) hovered: bool,
    pub(crate) pressed: bool,
    pub(crate) text_w: Cell<f32>,
    pub(crate) text_h: Cell<f32>,
}

impl Button {
    pub const CLICKED: u32 = 0;
    pub const HOVERED: u32 = 1;
    pub const HOVER_END: u32 = 2;
    pub const PRESSED: u32 = 3;

    pub fn new(ui: &mut Ui, text: &str) -> Handle<Self> {
        ui.add(Self {
            layout: Layout::default(),
            text: text.to_string(),
            color: Color::rgb(70, 70, 200),
            border_radius: 6.0,
            font_size: 16.0,
            font_weight: 600,
            hovered: false,
            pressed: false,
            text_w: Cell::new(0.0),
            text_h: Cell::new(0.0),
        })
    }
}

impl Element for Button {
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
        let (tw, th) = fonts.measure_sized(
            &self.text,
            "sans-serif",
            self.font_size,
            self.font_weight,
            false,
            max_width,
        );
        self.text_w.set(tw);
        self.text_h.set(th);
        Some((tw + 32.0, th + 16.0))
    }
    fn on_mouse_enter(&mut self) -> Option<u32> {
        self.hovered = true;
        Some(Button::HOVERED)
    }
    fn on_mouse_leave(&mut self) -> Option<u32> {
        self.hovered = false;
        self.pressed = false;
        Some(Button::HOVER_END)
    }
    fn on_press(&mut self) -> Option<u32> {
        self.pressed = true;
        Some(Button::PRESSED)
    }
    fn on_release(&mut self) -> Option<u32> {
        self.pressed = false;
        Some(Button::CLICKED)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
