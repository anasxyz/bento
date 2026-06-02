use crate::{widgets::Text, view::{View, ViewId}};

pub(crate) struct TextNode {
    pub(crate) text: Box<dyn Fn() -> String>,
}

pub(crate) enum NodeType {
    Text(Text),
}

pub(crate) struct Node {
    pub(crate) view: Box<dyn View>,
    pub(crate) children: Vec<ViewId>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
}
