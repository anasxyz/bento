use crate::ui::Ui;

pub struct Callbacks {
    pub on_click: Option<Box<dyn Fn(&mut Ui)>>,
    pub on_hover: Option<Box<dyn Fn(&mut Ui)>>,
    pub on_hover_end: Option<Box<dyn Fn(&mut Ui)>>,
}

impl Callbacks {
    pub fn new() -> Self {
        Self {
            on_click: None,
            on_hover: None,
            on_hover_end: None,
        }
    }

    pub fn has_click(&self) -> bool {
        self.on_click.is_some()
    }

    pub fn has_hover(&self) -> bool {
        self.on_hover.is_some()
    }
}
