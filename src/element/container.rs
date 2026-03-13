use crate::Ui;
use crate::element::element::Element;
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::element::values::FlexDirection;
use crate::fonts::Fonts;
use std::any::Any;

pub struct Container {
    pub layout: Layout,
    pub children: Vec<Handle<()>>,
}

impl Container {
    pub fn new(ui: &mut Ui) -> Handle<Self> {
        ui.add(Self {
            layout: Layout::default(),
            children: Vec::new(),
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
        let h = ui.add(Container {
            layout: Layout::default(),
            children: Vec::new(),
        });
        ui[h].layout.flex_direction = FlexDirection::Row;
        h
    }
}

pub struct Column;
impl Column {
    pub fn new(ui: &mut Ui) -> Handle<Container> {
        let h = ui.add(Container {
            layout: Layout::default(),
            children: Vec::new(),
        });
        ui[h].layout.flex_direction = FlexDirection::Col;
        h
    }
}
