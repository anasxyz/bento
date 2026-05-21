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
use crate::widget::{AnyWidget, Widget, WidgetHandle};

pub struct Slot {
    pub widget: Box<dyn AnyWidget>,
    pub generation: usize,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
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

    pub fn add<W: Widget + 'static>(&mut self, mut widget: W) -> WidgetHandle<W> {
        let index = self.slots.len();
        widget.set_id(index);
        widget.build(self);
        self.slots.push(Some(Slot {
            widget: Box::new(widget),
            generation: 0,
            children: Vec::new(),
            parent: None,
        }));
        WidgetHandle::new(index, 0)
    }

    pub fn add_child<P: Widget + 'static, C: Widget + 'static>(
        &mut self,
        parent: &P,
        child: C,
    ) -> WidgetHandle<C> {
        let child_handle = self.add(child);
        let parent_handle = WidgetHandle::<P>::from_id(parent.id());
        self.append(parent_handle, child_handle);
        child_handle
    }

    pub fn remove<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) {
        if let Some(slot) = self.slots.get_mut(handle.id) {
            *slot = None;
        }
    }

    pub fn get_mut<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) -> Option<&mut W> {
        let id = handle.id;
        self.slots
            .get_mut(id)?
            .as_mut()?
            .widget
            .as_any_mut()
            .downcast_mut::<W>()
    }

    pub fn append<W: Widget + 'static, C: Widget + 'static>(
        &mut self,
        handle: WidgetHandle<W>,
        child: WidgetHandle<C>,
    ) {
        // check if child is parent
        if handle.id == child.id {
            println!("[ERROR] Cannot append widget to itself");
            return;
        }

        // check if child is already a child of parent
        if let Some(Some(parent_slot)) = self.slots.get(handle.id) {
            if parent_slot.children.contains(&child.id) {
                println!("[ERROR] Cannot append, widget is already child of parent");
                return;
            }
        }

        if let Some(Some(parent_slot)) = self.slots.get_mut(handle.id) {
            parent_slot.children.push(child.id);
        }
        if let Some(Some(child_slot)) = self.slots.get_mut(child.id) {
            child_slot.parent = Some(handle.id);
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.slots.len() {
            if let Some(mut slot) = self.slots[i].take() {
                slot.widget.update(self);
                self.slots[i] = Some(slot);
            }
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
                    "[{}] {} parent={:?} children={:?} {:?}",
                    i,
                    s.widget.name(),
                    s.generation,
                    s.parent,
                    s.widget.hitbox(),
                );
            }
        }
    }
}
