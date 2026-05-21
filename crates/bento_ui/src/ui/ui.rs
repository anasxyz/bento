use std::any::{Any, TypeId};
use std::collections::HashMap;

use bento_shared::{Scene, SceneNodeId};

use crate::Key;
use crate::events::types::{
    Click, KeyPress, KeyRelease, MouseDown, MouseEnter, MouseLeave, MouseMove, MouseScroll, MouseUp,
};
use crate::input::InputState;
use crate::input::mouse::MouseButton;
use crate::ui::asyncs::AsyncEventQueue;
use crate::widget::Widget;

pub struct Slot {
    pub widget: Box<dyn Widget>,
    pub generation: u32,
    pub children: Vec<u32>,
    pub parent: Option<u32>,
}

pub struct Ui {
    pub scene: Scene,
    pub input: InputState,
    pub asyncs: AsyncEventQueue,

    pub slots: Vec<Option<Slot>>,

    pub needs_redraw: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            input: InputState::new(),
            asyncs: AsyncEventQueue::new(),

            slots: Vec::new(),

            needs_redraw: false,
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn add<W: Widget + Clone + 'static>(&mut self, widget: &mut W) {
        let index = self.slots.len();
        widget.set_id(index);
        let mut cloned = widget.clone();
        self.slots.push(Some(Slot {
            widget: Box::new(cloned),
            generation: 0,
            children: Vec::new(),
            parent: None,
        }));
    }

    pub fn remove<W: Widget + 'static>(&mut self, widget: &W) {
        let id = widget.id();

        if let Some(slot) = self.slots.get_mut(id) {
            *slot = None;
        }
    }

    pub fn process_input(&mut self) {
        for (k, _) in self.input.keyboard.just_pressed() {
            if *k == Key::D {
                self.print_slots();
            }
        }
    }
}

impl Ui {
    pub fn print_slots(&self) {
        for (i, slot) in self.slots.iter().enumerate() {
            if let Some(s) = slot {
                println!(
                    "[{}] {} gen={} parent={:?} children={:?}",
                    i,
                    s.widget.name(),
                    s.generation,
                    s.parent,
                    s.children
                );
            }
        }
    }
}
