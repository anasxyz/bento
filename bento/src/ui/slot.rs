use crate::widget::{AnyWidget, Handle};

pub struct Slot {
    pub widget: AnyWidget,
    pub generation: u32,
    pub children: Vec<Handle<()>>,
    pub parent: Option<Handle<()>>,
}
