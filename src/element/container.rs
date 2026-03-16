use crate::color::Color;
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::element::values::FlexDirection;
use crate::fonts::Fonts;
use crate::ui::Ui;
use std::any::Any;

pub struct Container {
    pub layout: Layout,
    pub bg_color: Option<Color>,
    pub border_radius: Option<f32>,
    pub border_thickness: f32,
    pub border_color: Option<Color>,
    pub focused: bool,
}

impl Container {
    pub const FOCUS_GAINED: u32 = 0;
    pub const FOCUS_LOST: u32 = 1;

    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
            bg_color: None,
            border_radius: None,
            border_thickness: 0.0,
            border_color: None,
            focused: false,
        }
    }

    pub fn on_focus_gained(&mut self) -> Option<u32> {
        self.focused = true;
        Some(Self::FOCUS_GAINED)
    }

    pub fn on_focus_lost(&mut self) -> Option<u32> {
        self.focused = false;
        Some(Self::FOCUS_LOST)
    }
}


pub struct Row;
impl Row {
    pub fn new() -> Container {
        let mut h = Container::new();
        h.layout.flex_direction = FlexDirection::Row;
        h
    }
}

pub struct Column;
impl Column {
    pub fn new() -> Container {
        let mut h = Container::new();
        h.layout.flex_direction = FlexDirection::Col;
        h
    }
}
