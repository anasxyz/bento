use crate::color::Color;
use crate::element::layout::Layout;
use crate::element::values::FlexDirection;

pub struct Container {
    pub(crate) layout: Layout,
    pub(crate) dirty: bool,
    bg_color: Option<Color>,
    border_radius: Option<f32>,
    border_width: [f32; 4],
    border_color: Option<Color>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
            dirty: true,
            bg_color: None,
            border_radius: None,
            border_width: [0.0; 4],
            border_color: None,
        }
    }

    // getters
    pub fn bg_color(&self) -> Option<Color> {
        self.bg_color
    }
    pub fn border_radius(&self) -> Option<f32> {
        self.border_radius
    }
    pub fn border_widths(&self) -> [f32; 4] {
        self.border_width
    }
    pub fn border_color(&self) -> Option<Color> {
        self.border_color
    }

    // setters
    pub fn set_bg_color(&mut self, color: Option<Color>) -> &mut Self {
        self.bg_color = color;
        self.dirty = true;
        self
    }
    pub fn set_border_radius(&mut self, radius: Option<f32>) -> &mut Self {
        self.border_radius = radius;
        self.dirty = true;
        self
    }
    pub fn set_border_color(&mut self, color: Option<Color>) -> &mut Self {
        self.border_color = color;
        self.dirty = true;
        self
    }
    pub fn set_border(&mut self, widths: [f32; 4]) -> &mut Self {
        self.border_width = widths;
        self.dirty = true;
        self
    }
}

pub struct Row;
impl Row {
    pub fn new() -> Container {
        let mut c = Container::new();
        c.layout.flex_direction = FlexDirection::Row;
        c
    }
}

pub struct Column;
impl Column {
    pub fn new() -> Container {
        let mut c = Container::new();
        c.layout.flex_direction = FlexDirection::Col;
        c
    }
}
