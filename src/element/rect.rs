use crate::color::Color;
use crate::element::element::Element;
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::fonts::Fonts;
use crate::ui::Ui;
use std::any::Any;

pub struct Rect {
    pub layout: Layout,
    pub bg_color: Color,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,
    pub border_thickness: f32,
    pub focused: bool,
}

impl Rect {
    pub const CLICKED: u32 = 0;
    pub const HOVERED: u32 = 1;
    pub const HOVER_END: u32 = 2;
    pub const PRESSED: u32 = 3;
    pub const RIGHT_CLICKED: u32 = 4;
    pub const MIDDLE_CLICKED: u32 = 5;
    pub const DOUBLE_CLICKED: u32 = 6;
    pub const FOCUS_GAINED: u32 = 7;
    pub const FOCUS_LOST: u32 = 8;

    pub fn new(ui: &mut Ui) -> Handle<Self> {
        ui.add(Self {
            layout: Layout::default(),
            bg_color: Color::new(0.0, 0.0, 0.0, 1.0),
            border_color: None,
            border_radius: None,
            border_thickness: 0.0,
            focused: false,
        })
    }
}

impl Element for Rect {
    fn layout(&self) -> &Layout {
        &self.layout
    }
    fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }
    fn has_measure(&self) -> bool {
        false
    }
    fn measure(&self, _fonts: &mut Fonts, _max_width: Option<f32>) -> Option<(f32, f32)> {
        None
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn on_mouse_enter(&mut self) -> Option<u32> {
        Some(Rect::HOVERED)
    }
    fn on_mouse_leave(&mut self) -> Option<u32> {
        Some(Rect::HOVER_END)
    }
    fn on_left_press(&mut self) -> Option<u32> {
        Some(Rect::PRESSED)
    }
    fn on_left_release(&mut self) -> Option<u32> {
        Some(Rect::CLICKED)
    }
    fn on_left_click(&mut self) -> Option<u32> {
        Some(Rect::CLICKED)
    }
    fn on_left_double_click(&mut self) -> Option<u32> {
        Some(Rect::DOUBLE_CLICKED)
    }
    fn on_right_click(&mut self) -> Option<u32> {
        Some(Rect::RIGHT_CLICKED)
    }
    fn on_middle_click(&mut self) -> Option<u32> {
        Some(Rect::MIDDLE_CLICKED)
    }
    fn on_focus_gained(&mut self) -> Option<u32> {
        self.focused = true;
        Some(Rect::FOCUS_GAINED)
    }
    fn on_focus_lost(&mut self) -> Option<u32> {
        self.focused = false;
        Some(Rect::FOCUS_LOST)
    }
}
