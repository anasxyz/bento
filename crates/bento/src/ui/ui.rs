use bento_shared::{RectNode, Scene};

pub struct Ui {
    pub scene: Scene,
}

impl Ui {
    pub fn new() -> Self {
        let mut scene = Scene::new();

        let mut rect = RectNode::new(0.0, 0.0, 100.0, 100.0);
        rect.color([0.0, 0.0, 0.0, 1.0]);
        scene.add_rect(rect);

        Self { scene }
    }
}
