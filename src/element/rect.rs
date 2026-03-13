use crate::element::layout::Layout;
use crate::color::Color;
use crate::element::element::Element;
use crate::fonts::Fonts;

pub struct Rect {
    pub layout: Layout,
    pub bg_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,
    pub border_thickness: f32,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
            bg_color: None,
            border_color: None,
            border_radius: None,
            border_thickness: 0.0,
        }
    }
}

impl Element for Rect {
    fn layout(&self) -> &Layout {
        &self.layout
    }

    fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }

    fn measure(&self, _fonts: &mut Fonts, _max_width: Option<f32>) -> Option<(f32, f32)> {
        None
    }
}
