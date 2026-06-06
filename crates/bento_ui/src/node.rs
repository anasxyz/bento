use bento_wgpu::DrawCommand;
use std::cell::Cell;
use std::rc::Rc;
use taffy::NodeId as TaffyNodeId;

use crate::layout::LayoutProps;
use crate::reactive::{owner::Owner, runtime::SubscriberId};
use crate::view::{View, ViewId};
use crate::{Signal, state};

pub(crate) struct EventHandler {
    pub(crate) type_id: std::any::TypeId,
    pub(crate) handler: Rc<dyn Fn(&dyn std::any::Any)>,
}

pub(crate) fn placeholder_taffy_id() -> TaffyNodeId {
    TaffyNodeId::new(u64::MAX)
}

#[derive(Clone, Copy)]
pub struct NodeRef(pub(crate) Signal<Option<ViewId>>);

impl NodeRef {
    pub fn get(&self) -> Option<ViewId> {
        self.0.get()
    }

    pub fn set(&self, id: ViewId) {
        self.0.set(Some(id));
    }
}

pub fn node_ref() -> NodeRef {
    NodeRef(state(None))
}

pub(crate) struct Node {
    pub(crate) name: Option<&'static str>,
    pub(crate) view: Box<dyn View>,
    pub(crate) taffy_id: TaffyNodeId,
    pub(crate) parent: Option<ViewId>,
    pub(crate) children: Vec<ViewId>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
    pub(crate) layout: LayoutProps,

    pub(crate) handlers: Vec<EventHandler>,
    pub(crate) owners: Vec<Owner>,

    pub(crate) paint_dirty: bool,
    pub(crate) cache: Vec<DrawCommand>,
    pub(crate) paint_subscriber: Option<SubscriberId>,

    pub(crate) scroll_x: f32,
    pub(crate) scroll_y: f32,
    pub(crate) scrollable: bool,
}
