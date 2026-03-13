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
}

impl Rect {
    pub fn new(ui: &mut Ui) -> Handle<Self> {
        ui.add(Self {
            layout: Layout::default(),
            bg_color: Color::rgb(0, 0, 0),
            border_color: None,
            border_radius: None,
            border_thickness: 0.0,
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
    fn measure(&self, _fonts: &mut Fonts, _max_width: Option<f32>) -> Option<(f32, f32)> {
        None
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
