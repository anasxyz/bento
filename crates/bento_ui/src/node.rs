use crate::{
    view::{View, ViewId},
    widgets::Text,
};

pub(crate) struct TextNode {
    pub(crate) text: Box<dyn Fn() -> String>,
}

pub(crate) enum NodeType {
    Text(Text),
}

pub(crate) struct EventHandler {
    pub(crate) type_id: std::any::TypeId,
    pub(crate) handler: Box<dyn Fn(&dyn std::any::Any)>,
}

pub(crate) struct Node {
    pub(crate) view: Box<dyn View>,
    pub(crate) children: Vec<ViewId>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
    pub(crate) handlers: Vec<EventHandler>,
}
