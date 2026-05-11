use bento_shared::Scene;

use super::EventQueue;
use crate::Widget;

pub struct Slot {
    pub widget: Box<dyn Widget>,
    pub generation: u32,
}

pub struct Ui {
    pub scene: Scene,
    pub slots: Vec<Option<Slot>>,
    pub events: EventQueue,
}

impl Ui {
    fn new() -> Self {
        Self {
            scene: Scene::new(),
            slots: Vec::new(),
            events: EventQueue::new(),
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    /// looks up a completed async callback by id and runs it with &mut Ui on the main thread
    pub fn fire_callback(&mut self, id: u64) {
        if let Some(callback) = self.events.callbacks.remove(&id) {
            callback(self);
        } else {
            let callback = self.events.async_callbacks.lock().unwrap().remove(&id);
            if let Some(callback) = callback {
                callback(self);
            }
        }
    }
}
