use std::any::{Any, TypeId};
use std::collections::HashMap;

use bento_shared::{CosmicTextMeasurer, MeasureCache, Scene, SceneNodeId};
use cosmic_text::FontSystem;

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

    pub measurer: CosmicTextMeasurer,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            input: InputState::new(),
            asyncs: AsyncEventQueue::new(),

            slots: Vec::new(),

            needs_redraw: false,
            
            measurer: CosmicTextMeasurer::new(),
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    pub fn add<W: Widget + 'static>(&mut self, mut widget: W) -> WidgetHandle<W> {
        let index = self.slots.len();
        widget.set_id(index);
        self.slots.push(None);
        widget.build(self);
        // any append() calls during build will have pushed to pending_children
        let children: Vec<usize> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().filter(|s| s.parent == Some(index)).map(|_| i))
            .collect();
        self.slots[index] = Some(Slot {
            widget: Box::new(widget),
            generation: 0,
            children,
            parent: None,
        });
        self.request_redraw();
        WidgetHandle::from_id(index)
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
        self.remove_id(handle.id);
        self.request_redraw();
    }

    fn remove_id(&mut self, id: usize) {
        let children = self
            .slots
            .get(id)
            .and_then(|s| s.as_ref())
            .map(|s| s.children.clone())
            .unwrap_or_default();
        for child_id in children {
            self.remove_id(child_id);
        }
        if let Some(Some(mut slot)) = self.slots.get_mut(id).map(|s| s.take()) {
            slot.widget.remove(self);
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
        // Iterates forwards through slots. This works correctly because children
        // always have higher slot indices than their parents, which is guaranteed by add_child.
        // If I manually append a widget created before its parent, it might miss a frame.
        for i in 0..self.slots.len() {
            if let Some(s) = self.slots[i].as_ref() {
                if !s.widget.is_dirty() {
                    continue;
                }
            } else {
                continue;
            }
            if let Some(mut slot) = self.slots[i].take() {
                slot.widget.update(self);
                println!("updating widget id: {}, name: {}", slot.widget.id(), slot.widget.name());
                slot.widget.set_dirty(false);
                self.slots[i] = Some(slot);
                self.request_redraw();
            }
        }
    }

    pub fn process_input(&mut self) {
        for (k, _) in self.input.keyboard.just_pressed() {
            if *k == Key::D {
                self.print_slots();
            }

            if *k == Key::S {
                self.scene.print_tree();
            }
        }
    }
}

impl Ui {
    pub fn print_slots(&self) {
        println!("[Slots]");
        if self.slots.iter().all(|s| s.is_none()) {
            println!("No slots");
            println!("[Slots]");
            return;
        }
        for (i, slot) in self.slots.iter().enumerate() {
            if let Some(s) = slot {
                println!(
                    "[{}] {} parent={:?} children={:?} {:?}",
                    i,
                    s.widget.name(),
                    s.parent,
                    s.children,
                    s.widget.hitbox(),
                );
            }
        }
        println!("[Slots]");
    }
}
