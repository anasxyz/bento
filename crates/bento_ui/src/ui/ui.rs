use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;

use bento_shared::Scene;

use crate::input::InputState;
use crate::ui::asyncs::AsyncEventQueue;
use crate::widget::{AnyWidget, Widget, WidgetHandle};

pub struct Slot {
    pub widget: Box<dyn AnyWidget>,
    pub generation: u32,
}

pub struct Ui {
    pub scene: Scene,
    pub input: InputState,
    pub asyncs: AsyncEventQueue,
    pub needs_redraw: bool,
    slots: Vec<Option<Slot>>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            input: InputState::new(),
            asyncs: AsyncEventQueue::new(),
            needs_redraw: false,
            slots: Vec::new(),
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    pub fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    pub fn process_input(&mut self) {}

    pub fn add<W: AnyWidget + 'static>(&mut self, mut widget: W) -> WidgetHandle<W> {
        let index = self.slots.len();
        widget.build(self);
        self.slots.push(Some(Slot {
            widget: Box::new(widget),
            generation: 0,
        }));
        self.request_redraw();
        WidgetHandle::new(index as u32, 0)
    }

    pub fn get<W: AnyWidget + 'static>(&self, handle: WidgetHandle<W>) -> Option<&W> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any().downcast_ref::<W>()
    }

    pub fn get_mut<W: AnyWidget + 'static>(&mut self, handle: WidgetHandle<W>) -> Option<&mut W> {
        let slot = self.slots.get_mut(handle.id as usize)?.as_mut()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any_mut().downcast_mut::<W>()
    }

    pub fn remove<W: AnyWidget + 'static>(&mut self, handle: WidgetHandle<W>) {
        let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) else {
            return;
        };
        if slot.generation != handle.generation {
            return;
        }
        let mut slot = self.slots[handle.id as usize].take().unwrap();
        slot.widget.remove(self);
        self.request_redraw();
    }

    pub fn update(&mut self) {
        for i in 0..self.slots.len() {
            if let Some(slot) = &self.slots[i] {
                if slot.widget.is_dirty() {
                    let mut slot = self.slots[i].take().unwrap();
                    slot.widget.update(self);
                    self.slots[i] = Some(slot);
                }
            }
        }
    }
}
