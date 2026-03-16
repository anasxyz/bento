use crate::element::element::AnyElement;
use crate::element::handle::Handle;
use crate::event::Event;
use crate::fonts::Fonts;
use crate::mouse::MouseState;
use std::ops::{Index, IndexMut};

const GLOBAL_ID: u32 = u32::MAX;

struct Slot {
    element: AnyElement,
    generation: u32,
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
    slots: Vec<Option<Slot>>,
    root: Option<Handle<()>>,
    connections: Vec<Connection>,
    next_connection_id: u32,
    event_queue: Vec<(Handle<()>, Event)>,
    pub interaction: InteractionState,
    pub fonts: Option<Fonts>,
    pub mouse: MouseState,
    pub window_width: u32,
    pub window_height: u32,
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
        }
    }

    pub fn global(&self) -> Handle<()> {
        Handle::new(GLOBAL_ID, 0)
    }

    pub fn add<T: Into<AnyElement>>(&mut self, element: T) -> Handle<T> {
        let any = element.into();
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(Slot {
                    element: any,
                    generation: 0,
                    children: Vec::new(),
                    parent: None,
                });
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
        Handle::new(id, 0)
    }

    pub fn get<T: 'static>(&self, handle: Handle<T>) -> Option<&T> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        get_inner_ref::<T>(&slot.element)
    }

    pub fn get_mut<T: 'static>(&mut self, handle: Handle<T>) -> Option<&mut T> {
        let slot = self.slots.get_mut(handle.id as usize)?.as_mut()?;
        if slot.generation != handle.generation {
            return None;
        }
        get_inner_mut::<T>(&mut slot.element)
    }

    pub(crate) fn get_any(&self, handle: Handle<()>) -> Option<&AnyElement> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        Some(&slot.element)
    }

    pub(crate) fn get_any_mut(&mut self, handle: Handle<()>) -> Option<&mut AnyElement> {
        let slot = self.slots.get_mut(handle.id as usize)?.as_mut()?;
        if slot.generation != handle.generation {
            return None;
        }
        Some(&mut slot.element)
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

    // pushes event onto the queue — safe to call from inside callbacks
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

    // drains the event queue and fires all callbacks
    // called once per frame by the app loop
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
                        None => continue, // was disconnected
                    };
                    let mut connections = std::mem::take(&mut self.connections);
                    let cb_ptr: *mut dyn FnMut(&mut Ui, &Event) = connections[i].callback.as_mut();
                    self.connections = connections;
                    unsafe { (*cb_ptr)(self, &event) };
                }
            }
        }
    }

    // mouse helpers
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

impl<T: 'static> Index<Handle<T>> for Ui {
    type Output = T;
    fn index(&self, handle: Handle<T>) -> &T {
        self.get(handle).expect("stale handle")
    }
}

impl<T: 'static> IndexMut<Handle<T>> for Ui {
    fn index_mut(&mut self, handle: Handle<T>) -> &mut T {
        self.get_mut(handle).expect("stale handle")
    }
}

fn get_inner_ref<T: 'static>(el: &AnyElement) -> Option<&T> {
    use crate::element::container::Container;
    use crate::element::label::Label;
    use crate::element::rect::Rect;
    use std::any::Any;
    let any: &dyn Any = match el {
        AnyElement::Rect(e) => e,
        AnyElement::Label(e) => e,
        AnyElement::Container(e) => e,
    };
    any.downcast_ref::<T>()
}

fn get_inner_mut<T: 'static>(el: &mut AnyElement) -> Option<&mut T> {
    use crate::element::container::Container;
    use crate::element::label::Label;
    use crate::element::rect::Rect;
    use std::any::Any;
    let any: &mut dyn Any = match el {
        AnyElement::Rect(e) => e,
        AnyElement::Label(e) => e,
        AnyElement::Container(e) => e,
    };
    any.downcast_mut::<T>()
}
