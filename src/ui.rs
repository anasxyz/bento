use crate::element::element::AnyElement;
use crate::element::handle::Handle;
use crate::fonts::Fonts;
use crate::keyboard::{Key, Modifiers};
use crate::mouse::MouseState;
use std::ops::{Index, IndexMut};

struct Slot {
    element: AnyElement,
    generation: u32,
    children: Vec<Handle<()>>,
    parent: Option<Handle<()>>,
}

pub struct Connection {
    pub handle: Handle<()>,
    pub signal: u32,
    pub callback: Box<dyn Fn(&mut Ui)>,
}

pub struct KeyConnection {
    pub handle: Option<Handle<()>>,
    pub callback: Box<dyn Fn(&mut Ui, Key, Modifiers, Option<char>)>,
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
    key_connections: Vec<KeyConnection>,
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
            key_connections: Vec::new(),
            interaction: InteractionState::new(),
            fonts: Some(Fonts::new()),
            mouse: MouseState::default(),
            window_width: 0,
            window_height: 0,
        }
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

    // internal — returns the AnyElement directly, used by draw/events/layout
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
        signal: u32,
        callback: impl Fn(&mut Ui) + 'static,
    ) {
        self.connections.push(Connection {
            handle: handle.untyped(),
            signal,
            callback: Box::new(callback),
        });
    }

    pub fn disconnect<T>(&mut self, handle: Handle<T>, signal: u32) {
        let handle = handle.untyped();
        self.connections
            .retain(|c| !(c.handle == handle && c.signal == signal));
    }

    pub fn emit(&mut self, handle: Handle<()>, signal: u32) {
        let indices: Vec<usize> = self
            .connections
            .iter()
            .enumerate()
            .filter(|(_, c)| c.handle == handle && c.signal == signal)
            .map(|(i, _)| i)
            .collect();
        for i in indices {
            let mut connections = std::mem::take(&mut self.connections);
            let cb_ptr: *const dyn Fn(&mut Ui) = connections[i].callback.as_ref();
            self.connections = connections;
            unsafe { (*cb_ptr)(self) };
        }
    }

    pub fn emit_bubbling(&mut self, handle: Handle<()>, signal: u32) {
        let mut chain = vec![handle];
        let mut current = self.parent(handle);
        while let Some(p) = current {
            chain.push(p);
            current = self.parent(p);
        }
        for ancestor in chain {
            self.emit(ancestor, signal);
        }
    }

    pub fn broadcast(&mut self, signal: u32) {
        let handles: Vec<Handle<()>> = self
            .connections
            .iter()
            .filter(|c| c.signal == signal)
            .map(|c| c.handle)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        for handle in handles {
            self.emit(handle, signal);
        }
    }

    pub fn connect_key<T>(
        &mut self,
        handle: Handle<T>,
        callback: impl Fn(&mut Ui, Key, Modifiers, Option<char>) + 'static,
    ) {
        self.key_connections.push(KeyConnection {
            handle: Some(handle.untyped()),
            callback: Box::new(callback),
        });
    }

    pub fn connect_key_global(
        &mut self,
        callback: impl Fn(&mut Ui, Key, Modifiers, Option<char>) + 'static,
    ) {
        self.key_connections.push(KeyConnection {
            handle: None,
            callback: Box::new(callback),
        });
    }

    pub fn fire_key(
        &mut self,
        focused: Option<Handle<()>>,
        key: Key,
        modifiers: Modifiers,
        text: Option<char>,
    ) {
        let mut key_connections = std::mem::take(&mut self.key_connections);
        for conn in &key_connections {
            let should_fire = match conn.handle {
                None => true,
                Some(h) => Some(h) == focused,
            };
            if should_fire {
                let cb_ptr: *const dyn Fn(&mut Ui, Key, Modifiers, Option<char>) =
                    conn.callback.as_ref();
                unsafe { (*cb_ptr)(self, key.clone(), modifiers.clone(), text) };
            }
        }
        self.key_connections = key_connections;
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

// downcast helpers — match AnyElement to get &T or &mut T
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
