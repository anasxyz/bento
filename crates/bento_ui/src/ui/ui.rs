use crate::widget::Button;
use bento_shared::scene::Scene;

pub struct Ui {
    pub scene: Scene,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
        }
    }

    // returns reference to the scene
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    // returns mutable reference to the scene
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn add_button(&mut self, button: Button) {
        println!("add button called");
        button.build(&mut self.scene);
        println!("scene nodes after build: {}", self.scene.nodes.len());
    }
}
