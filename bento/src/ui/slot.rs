use crate::widget::{AnyWidget, Handle};
use bento_wgpu::SceneNodeId;

pub struct Slot {
    pub widget: AnyWidget,
    pub generation: u32,
    pub children: Vec<Handle<()>>,
    pub parent: Option<Handle<()>>,
    pub scene_nodes: Vec<SceneNodeId>,
}
