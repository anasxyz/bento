use std::rc::Rc;

use bento_wgpu::DrawCommand;

use crate::{
    layout::{CrossAxis, Direction, MainAxis, Size},
    reactive::{owner::Owner, runtime::SubscriberId},
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
    pub(crate) handler: Rc<dyn Fn(&dyn std::any::Any)>,
}

pub(crate) struct Node {
    pub(crate) view: Box<dyn View>,
    pub(crate) parent: Option<ViewId>,
    pub(crate) children: Vec<ViewId>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
    pub(crate) handlers: Vec<EventHandler>,
    pub(crate) owners: Vec<Owner>,

    pub(crate) paint_dirty: bool,
    pub(crate) cache: Vec<DrawCommand>,
    pub(crate) paint_subscriber: Option<SubscriberId>,

    pub(crate) layout_dirty: bool,

    pub(crate) width: Size,
    pub(crate) height: Size,

    pub(crate) last_available_w: f32,
    pub(crate) last_available_h: f32,
}
