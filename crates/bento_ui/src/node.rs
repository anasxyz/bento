use crate::{widgets::Text, view::ViewId};

pub(crate) struct TextNode {
    pub(crate) text: Box<dyn Fn() -> String>,
}

pub(crate) enum NodeType {
    Text(Text),
}

pub(crate) struct Node {
    pub(crate) ntype: NodeType,
    pub(crate) children: Vec<ViewId>,
}
