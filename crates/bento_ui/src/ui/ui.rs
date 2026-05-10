use crate::widget::{Widget, WidgetHandle};
use bento_shared::BentoEvent;
use bento_shared::{TextMeasurer, scene::Scene};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Slot {
    pub widget: Box<dyn Widget>,
    pub generation: u32,
}

struct EventQueue {
    sender: Option<Arc<dyn Fn(u64) + Send + Sync>>,
    callbacks: HashMap<u64, Box<dyn FnOnce(&mut Ui)>>,
    next_id: u64,
    pending: Vec<(u64, f32)>,
}

impl EventQueue {
    fn new() -> Self {
        Self {
            sender: None,
            callbacks: HashMap::new(),
            next_id: 0,
            pending: Vec::new(),
        }
    }
}

pub struct Ui {
    pub scene: Scene,
    slots: Vec<Option<Slot>>,
    events: EventQueue,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            slots: Vec::new(),
            events: EventQueue::new(),
        }
    }

    pub fn add<W: Widget + 'static>(&mut self, mut widget: W) -> WidgetHandle<W> {
        widget.build(&mut self.scene);
        let generation = 0;
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(Slot {
                    widget: Box::new(widget),
                    generation,
                });
                return WidgetHandle::new(i as u32, generation);
            }
        }
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

    pub fn with<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>, f: impl FnOnce(&mut W)) {
        if let Some(widget) = self.get_mut(handle) {
            f(widget);
        }
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

    pub fn set_sender(&mut self, sender: Arc<dyn Fn(u64) + Send + Sync>) {
        self.events.sender = Some(sender.clone());
        for (id, duration) in self.events.pending.drain(..) {
            let sender = sender.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs_f32(duration));
                sender(id);
            });
        }
    }

    pub fn timer(&mut self, duration: f32, callback: impl FnOnce(&mut Ui) + 'static) {
        let id = self.events.next_id;
        self.events.next_id += 1;
        self.events.callbacks.insert(id, Box::new(callback));
        self.events.pending.push((id, duration));
    }

    pub fn fire_callback(&mut self, id: u64) {
        if let Some(callback) = self.events.callbacks.remove(&id) {
            callback(self);
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}
