use crate::element::callbacks::Callbacks;
use crate::element::element::Element;
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::element::values::FlexDirection;
use crate::fonts::Fonts;
use crate::ui::Ui;
use std::any::Any;

pub struct Container {
    pub layout: Layout,
    pub callbacks: Callbacks,
}

impl Container {
    pub fn new(ui: &mut Ui) -> Handle<Self> {
        ui.add(Self {
            layout: Layout::default(),
            callbacks: Callbacks::new(),
        })
    }

    pub fn on_click(&mut self, f: impl Fn(&mut Ui) + 'static) -> &mut Self {
        self.callbacks.on_click = Some(Box::new(f));
        self
    }

    pub fn on_hover(&mut self, f: impl Fn(&mut Ui) + 'static) -> &mut Self {
        self.callbacks.on_hover = Some(Box::new(f));
        self
    }

    pub fn on_hover_end(&mut self, f: impl Fn(&mut Ui) + 'static) -> &mut Self {
        self.callbacks.on_hover_end = Some(Box::new(f));
        self
    }
}

impl Element for Container {
    fn layout(&self) -> &Layout {
        &self.layout
    }
    fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }
    fn callbacks(&self) -> &Callbacks {
        &self.callbacks
    }
    fn callbacks_mut(&mut self) -> &mut Callbacks {
        &mut self.callbacks
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

