use crate::widget::{Widget, WidgetHandle};
use bento_shared::{TextMeasurer, scene::Scene};

pub struct Slot {
    pub widget: Box<dyn Widget>,
    pub generation: u32,
}

pub struct Ui {
    pub scene: Scene,
    slots: Vec<Option<Slot>>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            slots: Vec::new(),
        }
    }

    pub fn add<W: Widget + 'static>(&mut self, mut widget: W) -> WidgetHandle<W> {
        widget.build(&mut self.scene);

        let generation = 0;

        // reuse empty slot if available
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(Slot {
                    widget: Box::new(widget),
                    generation,
                });
                return WidgetHandle::new(i as u32, generation);
            }
        }

        // otherwise push new slot
        let id = self.slots.len() as u32;
        self.slots.push(Some(Slot {
            widget: Box::new(widget),
            generation,
        }));
        WidgetHandle::new(id, generation)
    }

    pub fn update(&mut self, measurer: &mut dyn TextMeasurer) {
        for slot in self.slots.iter_mut().flatten() {
            slot.widget.update(&mut self.scene, measurer);
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}
