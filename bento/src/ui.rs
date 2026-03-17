use crate::element::element::{AnyElement, Element};
use crate::element::handle::Handle;
use crate::event::Event;
use crate::fonts::Fonts;
use crate::input::MouseState;
use std::collections::HashMap;
use taffy::prelude::{NodeId, TaffyTree};

const GLOBAL_ID: u32 = u32::MAX;

pub struct Slot {
    pub(crate) element: AnyElement,
    pub(crate) generation: u32,
    children: Vec<Handle<()>>,
    parent: Option<Handle<()>>,
}

pub struct Connection {
    pub id: u32,
    pub handle: Handle<()>,
    pub callback: Box<dyn FnMut(&mut Ui, &Event)>,
}

pub struct InteractionState {
    pub hovered: Option<Handle<()>>,
    pub pressed: Option<Handle<()>>,
    pub focused: Option<Handle<()>>,
}

impl InteractionState {
    pub fn new() -> Self {
        Self {
            hovered: None,
            pressed: None,
            focused: None,
        }
    }
}

pub struct Ui {
    pub(crate) slots: Vec<Option<Slot>>,
    root: Option<Handle<()>>,
    connections: Vec<Connection>,
    next_connection_id: u32,
    event_queue: Vec<(Handle<()>, Event)>,
    pub interaction: InteractionState,
    pub fonts: Option<Fonts>,
    pub mouse: MouseState,
    pub window_width: u32,
    pub window_height: u32,
    pub(crate) taffy: Option<TaffyTree<Handle<()>>>,
    pub(crate) taffy_nodes: HashMap<Handle<()>, NodeId>,
    pub(crate) taffy_root: Option<NodeId>,
    pub draw_list_dirty: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            root: None,
            connections: Vec::new(),
            next_connection_id: 0,
            event_queue: Vec::new(),
            interaction: InteractionState::new(),
            fonts: Some(Fonts::new()),
            mouse: MouseState::default(),
            window_width: 0,
            window_height: 0,
            taffy: Some(TaffyTree::new()),
            taffy_nodes: HashMap::new(),
            taffy_root: None,
            draw_list_dirty: false,
        }
    }

    pub fn global(&self) -> Handle<()> {
        Handle::new(GLOBAL_ID, 0)
    }

    pub fn add<T: Element>(&mut self, element: T) -> Handle<T> {
        let any: AnyElement = Box::new(element);
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(Slot {
                    element: any,
                    generation: 0,
                    children: Vec::new(),
                    parent: None,
                });
                crate::layout::invalidate_layout(self);
                self.draw_list_dirty = true;
                return Handle::new(i as u32, 0);
            }
        }
        let id = self.slots.len() as u32;
        self.slots.push(Some(Slot {
            element: any,
            generation: 0,
            children: Vec::new(),
            parent: None,
        }));
        crate::layout::invalidate_layout(self);
        self.draw_list_dirty = true;
        Handle::new(id, 0)
    }

    // Returns the concrete type directly — no enum, no match needed by caller.
    pub fn get_mut<T: Element>(&mut self, handle: Handle<T>) -> Option<&mut T> {
        let slot = self.slots.get_mut(handle.id as usize)?.as_mut()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.element.as_any_mut().downcast_mut::<T>()
    }

    pub fn get<T: Element>(&self, handle: Handle<T>) -> Option<&T> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.element.as_any().downcast_ref::<T>()
    }

    pub(crate) fn get_any(&self, handle: Handle<()>) -> Option<&dyn Element> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        Some(slot.element.as_ref())
    }

    pub(crate) fn get_any_mut(&mut self, handle: Handle<()>) -> Option<&mut dyn Element> {
        let slot = self.slots.get_mut(handle.id as usize)?.as_mut()?;
        if slot.generation != handle.generation {
            return None;
        }
        Some(slot.element.as_mut())
    }

    pub fn set_root<T>(&mut self, handle: Handle<T>) {
        self.root = Some(handle.untyped());
    }

    pub fn root(&self) -> Option<Handle<()>> {
        self.root
    }

    pub fn append<P, C>(&mut self, parent: Handle<P>, child: Handle<C>) {
        let parent = parent.untyped();
        let child = child.untyped();
        if let Some(Some(slot)) = self.slots.get_mut(child.id as usize) {
            slot.parent = Some(parent);
        }
        if let Some(Some(slot)) = self.slots.get_mut(parent.id as usize) {
            slot.children.push(child);
        }
        crate::layout::invalidate_layout(self);
        self.draw_list_dirty = true;
    }

    pub fn children(&self, handle: Handle<()>) -> &[Handle<()>] {
        self.slots
            .get(handle.id as usize)
            .and_then(|s| s.as_ref())
            .map(|s| s.children.as_slice())
            .unwrap_or(&[])
    }

    pub fn parent(&self, handle: Handle<()>) -> Option<Handle<()>> {
        self.slots
            .get(handle.id as usize)?
            .as_ref()
            .and_then(|s| s.parent)
    }

    pub fn remove<T>(&mut self, handle: Handle<T>) {
        let handle = handle.untyped();
        if let Some(parent) = self
            .slots
            .get(handle.id as usize)
            .and_then(|s| s.as_ref())
            .and_then(|s| s.parent)
        {
            if let Some(Some(slot)) = self.slots.get_mut(parent.id as usize) {
                slot.children.retain(|c| *c != handle);
            }
        }
        if let Some(slot) = self.slots.get_mut(handle.id as usize) {
            if let Some(s) = slot {
                if s.generation == handle.generation {
                    *slot = None;
                }
            }
        }
        self.connections.retain(|c| c.handle != handle);
        self.event_queue.retain(|(h, _)| *h != handle);
        if self.interaction.hovered == Some(handle) {
            self.interaction.hovered = None;
        }
        if self.interaction.pressed == Some(handle) {
            self.interaction.pressed = None;
        }
        if self.interaction.focused == Some(handle) {
            self.interaction.focused = None;
        }
        crate::layout::invalidate_layout(self);
        self.draw_list_dirty = true;
    }

    pub fn remove_children<T>(&mut self, handle: Handle<T>) {
        let handle = handle.untyped();
        let children = self.children(handle).to_vec();
        for child in children {
            self.remove(child);
        }
        if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
            slot.children.clear();
        }
        crate::layout::invalidate_layout(self);
        self.draw_list_dirty = true;
    }

    pub fn connect<T>(
        &mut self,
        handle: Handle<T>,
        callback: impl FnMut(&mut Ui, &Event) + 'static,
    ) -> u32 {
        let id = self.next_connection_id;
        self.next_connection_id += 1;
        self.connections.push(Connection {
            id,
            handle: handle.untyped(),
            callback: Box::new(callback),
        });
        id
    }

    pub fn disconnect(&mut self, connection_id: u32) {
        self.connections.retain(|c| c.id != connection_id);
    }

    pub fn has_connections(&self, handle: Handle<()>) -> bool {
        self.connections.iter().any(|c| c.handle == handle)
    }

    pub fn emit<T>(&mut self, handle: Handle<T>, event: Event) {
        self.event_queue.push((handle.untyped(), event));
    }

    pub fn emit_bubbling<T>(&mut self, handle: Handle<T>, event: Event) {
        let handle = handle.untyped();
        let mut chain = vec![handle];
        let mut current = self.parent(handle);
        while let Some(p) = current {
            chain.push(p);
            current = self.parent(p);
        }
        chain.push(self.global());
        for ancestor in chain {
            self.event_queue.push((ancestor, event.clone()));
        }
    }

    pub fn drain_events(&mut self) {
        while !self.event_queue.is_empty() {
            let queue = std::mem::take(&mut self.event_queue);
            for (handle, event) in queue {
                let ids: Vec<u32> = self
                    .connections
                    .iter()
                    .filter(|c| c.handle == handle)
                    .map(|c| c.id)
                    .collect();
                for id in ids {
                    let i = match self.connections.iter().position(|c| c.id == id) {
                        Some(i) => i,
                        None => continue,
                    };
                    let mut connections = std::mem::take(&mut self.connections);
                    let cb_ptr: *mut dyn FnMut(&mut Ui, &Event) = connections[i].callback.as_mut();
                    self.connections = connections;
                    unsafe { (*cb_ptr)(self, &event) };
                }
            }
        }
    }

    pub fn any_dirty(&self) -> bool {
        self.slots
            .iter()
            .filter_map(|s| s.as_ref())
            .any(|s| s.element.is_dirty())
    }

    pub fn clear_dirty(&mut self) {
        for slot in self.slots.iter_mut().filter_map(|s| s.as_mut()) {
            slot.element.set_dirty(false);
        }
    }

    pub fn mark_all_dirty(&mut self) {
        for slot in self.slots.iter_mut().filter_map(|s| s.as_mut()) {
            slot.element.set_dirty(true);
        }
    }

    pub fn dirty_region(&self) -> Option<[f32; 4]> {
        let pad = 4.0;
        let mut region: Option<[f32; 4]> = None;
        for slot in self.slots.iter().filter_map(|s| s.as_ref()) {
            if !slot.element.is_dirty() {
                continue;
            }
            let l = slot.element.layout();
            if l.w > 0.0 || l.h > 0.0 {
                let rect = [l.x - pad, l.y - pad, l.x + l.w + pad, l.y + l.h + pad];
                region = Some(union_rect(region, rect));
            }
            if l.prev_w > 0.0 || l.prev_h > 0.0 {
                let rect = [
                    l.prev_x - pad,
                    l.prev_y - pad,
                    l.prev_x + l.prev_w + pad,
                    l.prev_y + l.prev_h + pad,
                ];
                region = Some(union_rect(region, rect));
            }
        }
        region
    }

    pub fn mouse_x(&self) -> f32 {
        self.mouse.x
    }
    pub fn mouse_y(&self) -> f32 {
        self.mouse.y
    }
    pub fn mouse_down(&self) -> bool {
        self.mouse.left_pressed
    }
    pub fn mouse_just_pressed(&self) -> bool {
        self.mouse.left_just_pressed
    }
    pub fn mouse_just_released(&self) -> bool {
        self.mouse.left_just_released
    }
    pub fn right_mouse_down(&self) -> bool {
        self.mouse.right_pressed
    }
    pub fn right_mouse_just_pressed(&self) -> bool {
        self.mouse.right_just_pressed
    }
    pub fn right_mouse_just_released(&self) -> bool {
        self.mouse.right_just_released
    }
}

fn union_rect(region: Option<[f32; 4]>, rect: [f32; 4]) -> [f32; 4] {
    match region {
        None => rect,
        Some([ax, ay, ax2, ay2]) => [
            ax.min(rect[0]),
            ay.min(rect[1]),
            ax2.max(rect[2]),
            ay2.max(rect[3]),
        ],
    }
}
