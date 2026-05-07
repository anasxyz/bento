use crate::widget::Widget;
use bento_shared::{TextMeasurer, scene::Scene};

pub struct Ui {
    pub scene: Scene,
    widgets: Vec<Box<dyn Widget>>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            widgets: Vec::new(),
        }
    }

    pub fn add<W: Widget + 'static>(&mut self, mut widget: W) {
        widget.build(&mut self.scene);
        self.widgets.push(Box::new(widget));
    }

    pub fn build(&mut self) {
        for widget in &mut self.widgets {
            widget.build(&mut self.scene);
        }
    }

    pub fn update(&mut self, measurer: &mut dyn TextMeasurer) {
        for widget in &mut self.widgets {
            widget.update(&mut self.scene, measurer);
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}
