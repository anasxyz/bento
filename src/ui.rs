use crate::element::element::Element;
use crate::element::handle::Handle;
use crate::signals::Signal;
use std::ops::{Index, IndexMut};

struct Slot {
    element: Box<dyn Element + 'static>,
    generation: u32,
    children: Vec<Handle<()>>,
    parent: Option<Handle<()>>,
}

pub struct Connection {
    pub handle: Handle<()>,
    pub signal: Signal,
    pub callback: Box<dyn Fn(&mut Ui)>,
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
    pub interaction: InteractionState,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            root: None,
            connections: Vec::new(),
            interaction: InteractionState::new(),
        }
    }

    pub fn add<T: Element + 'static>(&mut self, element: T) -> Handle<T> {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                let handle = Handle::new(i as u32, 0);
                *slot = Some(Slot {
                    element: Box::new(element),
                    generation: 0,
                    children: Vec::new(),
                    parent: None,
                });
                return handle;
            }
        }
        let id = self.slots.len() as u32;
        let handle = Handle::new(id, 0);
        self.slots.push(Some(Slot {
            element: Box::new(element),
            generation: 0,
            children: Vec::new(),
            parent: None,
        }));
        handle
    }

    pub fn remove<T>(&mut self, handle: impl Into<Handle<T>>) {
        let handle = handle.into();
        let erased = Handle::new(handle.id, handle.generation);

        let parent = self
            .slots
            .get(handle.id as usize)
            .and_then(|s| s.as_ref())
            .and_then(|s| s.parent);

        if let Some(parent_handle) = parent {
            if let Some(Some(parent_slot)) = self.slots.get_mut(parent_handle.id as usize) {
                parent_slot.children.retain(|c| *c != erased);
            }
        }

        if let Some(slot) = self.slots.get_mut(handle.id as usize) {
            if let Some(s) = slot {
                if s.generation == handle.generation {
                    *slot = None;
                }
            }
        }

        self.connections.retain(|c| c.handle != erased);

        if self.interaction.hovered == Some(erased) {
            self.interaction.hovered = None;
        }
        if self.interaction.pressed == Some(erased) {
            self.interaction.pressed = None;
        }
        if self.interaction.focused == Some(erased) {
            self.interaction.focused = None;
        }
    }

    pub fn remove_children<T>(&mut self, handle: impl Into<Handle<T>>) {
        let handle = handle.into();
        let erased = Handle::new(handle.id, handle.generation);
        let children = self.children(erased).to_vec();
        for child in children {
            self.remove(child);
        }
        if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
            slot.children.clear();
        }
    }

    pub fn append<P, C>(&mut self, parent: impl Into<Handle<P>>, child: impl Into<Handle<C>>) {
        let parent = parent.into();
        let child = child.into();
        let parent_erased = Handle::new(parent.id, parent.generation);
        let child_erased: Handle<()> = Handle::new(child.id, child.generation);
        if let Some(Some(child_slot)) = self.slots.get_mut(child.id as usize) {
            child_slot.parent = Some(parent_erased);
        }
        if let Some(Some(parent_slot)) = self.slots.get_mut(parent.id as usize) {
            parent_slot.children.push(child_erased);
        }
    }

    pub fn set_root<T>(&mut self, handle: impl Into<Handle<T>>) {
        let handle = handle.into();
        self.root = Some(Handle::new(handle.id, handle.generation));
    }

    pub fn root(&self) -> Option<Handle<()>> {
        self.root
    }

    pub fn get<T: Element + 'static>(&self, handle: Handle<T>) -> Option<&T> {
        self.slots.get(handle.id as usize)?.as_ref().and_then(|s| {
            if s.generation == handle.generation {
                s.element.as_any().downcast_ref::<T>()
            } else {
                None
            }
        })
    }

    pub fn get_mut<T: Element + 'static>(&mut self, handle: Handle<T>) -> Option<&mut T> {
        self.slots
            .get_mut(handle.id as usize)?
            .as_mut()
            .and_then(|s| {
                if s.generation == handle.generation {
                    s.element.as_any_mut().downcast_mut::<T>()
                } else {
                    None
                }
            })
    }

    pub fn get_dyn(&self, handle: Handle<()>) -> Option<&dyn Element> {
        self.slots.get(handle.id as usize)?.as_ref().and_then(|s| {
            if s.generation == handle.generation {
                Some(s.element.as_ref())
            } else {
                None
            }
        })
    }

    pub fn get_dyn_mut(&mut self, handle: Handle<()>) -> Option<&mut (dyn Element + 'static)> {
        self.slots
            .get_mut(handle.id as usize)?
            .as_mut()
            .and_then(|s| {
                if s.generation == handle.generation {
                    Some(s.element.as_mut())
                } else {
                    None
                }
            })
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

    pub fn connect<T>(
        &mut self,
        handle: impl Into<Handle<T>>,
        signal: Signal,
        callback: impl Fn(&mut Ui) + 'static,
    ) {
        let handle = handle.into();
        self.connections.push(Connection {
            handle: Handle::new(handle.id, handle.generation),
            signal,
            callback: Box::new(callback),
        });
    }

    pub fn disconnect<T>(&mut self, handle: impl Into<Handle<T>>, signal: Signal) {
        let handle = handle.into();
        let erased = Handle::new(handle.id, handle.generation);
        self.connections
            .retain(|c| !(c.handle == erased && c.signal == signal));
    }

    pub fn take_connections(&mut self) -> Vec<Connection> {
        std::mem::take(&mut self.connections)
    }

    pub fn restore_connections(&mut self, connections: Vec<Connection>) {
        self.connections = connections;
    }
}

impl<T: Element + 'static> Index<Handle<T>> for Ui {
    type Output = T;
    fn index(&self, handle: Handle<T>) -> &T {
        self.get(handle).expect("stale handle")
    }
}

impl<T: Element + 'static> IndexMut<Handle<T>> for Ui {
    fn index_mut(&mut self, handle: Handle<T>) -> &mut T {
        self.get_mut(handle).expect("stale handle")
    }
}
