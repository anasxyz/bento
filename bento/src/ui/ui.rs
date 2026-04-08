use bento_wgpu::{
    SceneGraph,
};

pub struct Ui {
    pub(crate) scene: SceneGraph,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: SceneGraph::new(),
        }
    }
}
