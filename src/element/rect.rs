use crate::element::layout::Layout;
use crate::color::Color;

pub struct Rect {
    pub layout: Layout,
    pub bg_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,
    pub border_thickness: f32,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            layout: Layout::default(),
            bg_color: None,
            border_color: None,
            border_radius: None,
            border_thickness: 0.0,
        }
    }
}
