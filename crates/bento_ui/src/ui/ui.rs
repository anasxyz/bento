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

    pub fn get<W: Widget + 'static>(&self, handle: WidgetHandle<W>) -> Option<&W> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any().downcast_ref::<W>()
    }

    pub fn get_mut<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) -> Option<&mut W> {
        let slot = self.slots.get_mut(handle.id as usize)?.as_mut()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any_mut().downcast_mut::<W>()
    }

    pub fn update(&mut self, measurer: &mut dyn TextMeasurer, delta: f32) {
        for slot in self.slots.iter_mut().flatten() {
            if slot.widget.base().dirty {
                slot.widget.base_mut().delta = delta;
                slot.widget.base_mut().dirty = false;
                slot.widget.pre_update();
                slot.widget.update(&mut self.scene, measurer);
            }
        }
    }

    pub fn any_dirty(&self) -> bool {
        self.slots.iter().flatten().any(|s| s.widget.base().dirty)
    }

    pub fn with<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>, f: impl FnOnce(&mut W)) {
        if let Some(widget) = self.get_mut(handle) {
            f(widget);
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}
