use crate::widget::Button;
use bento_shared::{measure::TextMeasurer, scene::Scene};

pub struct Ui {
    pub scene: Scene,
    buttons: Vec<Button>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            buttons: Vec::new(),
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
        self.buttons.push(button);
    }

    pub fn build(&mut self, measurer: &mut dyn TextMeasurer) {
        for button in &self.buttons {
            button.build(&mut self.scene, measurer);
        }
    }
}
