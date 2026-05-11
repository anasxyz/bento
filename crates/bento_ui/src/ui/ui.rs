use std::fmt;

use bento_shared::Scene;

use super::EventQueue;
use crate::{Widget, WidgetHandle};

/// Slot in the UI tree where a widget lives.
pub struct Slot {
    pub widget: Box<dyn Widget>,
    pub generation: u32,
}

/// Main orchestrator of anything UI / Event related.
pub struct Ui {
    pub scene: Scene,
    pub slots: Vec<Option<Slot>>,
    pub events: EventQueue,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            slots: Vec::new(),
            events: EventQueue::new(),
        }
    }

    /// Adds a widget to the ui and returns a handle to it.
    ///
    /// Iterates through slots until it finds a none/empty/free slot to be reused and returns
    /// its position/index as Option<usize>. If no slot is found, return the end of the slots vector as the index.
    /// Once index is found, creates a new slot at the index and adds the widget to it.
    ///
    /// Returns a WidgetHandle to the added the widget.
    pub fn add<W: Widget + 'static>(&mut self, widget: W) -> WidgetHandle<W> {
        let index = self
            .slots
            .iter()
            .position(|s| s.is_none())
            .unwrap_or(self.slots.len());

        let slot = Slot {
            widget: Box::new(widget),
            generation: 0,
        };

        if index == self.slots.len() {
            self.slots.push(Some(slot));
        } else {
            self.slots[index] = Some(slot);
        }

        WidgetHandle::new(index as u32, 0)
    }

    /// Returns a reference to the scene.
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    /// Returns a mutable reference to the scene.
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    /// Looks up a completed async callback by id and runs it with &mut Ui on the main thread.
    /// 
    /// Completely ignore for this now, it just has to be here and can't be on EventQueue because
    /// of the self parameter in the callbac.
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

// For debug
impl fmt::Display for Ui {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Ui ({} slots):", self.slots.len())?;
        for (i, slot) in self.slots.iter().enumerate() {
            match slot {
                Some(s) => writeln!(f, "  [{}] {} gen={}", i, s.widget.name(), s.generation)?,
                None => writeln!(f, "  [{}] empty", i)?,
            }
        }
        Ok(())
    }
}
