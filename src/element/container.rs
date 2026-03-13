use crate::element::element::Element;
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::element::values::FlexDirection;
use crate::fonts::Fonts;
use crate::ui::Ui;
use std::any::Any;

pub struct Container {
    pub layout: Layout,
}

impl Container {
    pub fn new(ui: &mut Ui) -> Handle<Self> {
        ui.add(Self {
            layout: Layout::default(),
        })
    }
}

impl Element for Container {
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
}

pub struct Row;
impl Row {
    pub fn new(ui: &mut Ui) -> Handle<Container> {
        let h = Container::new(ui);
        ui.get_mut(h).unwrap().layout.flex_direction = FlexDirection::Row;
        h
    }
}

pub struct Column;
impl Column {
    pub fn new(ui: &mut Ui) -> Handle<Container> {
        let h = Container::new(ui);
        ui.get_mut(h).unwrap().layout.flex_direction = FlexDirection::Col;
        h
    }
}

