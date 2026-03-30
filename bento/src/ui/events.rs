use crate::widget::Handle;
use std::any::Any;
use std::collections::HashMap;

// Event trait 

pub trait Event: Any + 'static {
    fn stop_propagation(&mut self);
    fn stop_default(&mut self);
    fn is_propagation_stopped(&self) -> bool;
    fn is_default_stopped(&self) -> bool;

    /// whether this event bubbles up the tree by default
    fn bubbles() -> bool
    where
        Self: Sized,
    {
        true
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// macro to implement Event on a struct that has propagation_stopped and default_stopped fields
macro_rules! impl_event {
    ($t:ty) => {
        impl Event for $t {
            fn stop_propagation(&mut self) {
                self.propagation_stopped = true;
            }
            fn stop_default(&mut self) {
                self.default_stopped = true;
            }
            fn is_propagation_stopped(&self) -> bool {
                self.propagation_stopped
            }
            fn is_default_stopped(&self) -> bool {
                self.default_stopped
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }
    };
}

// builtin event types 

#[derive(Clone, Debug)]
pub struct Click {
    pub x: f32,
    pub y: f32,
    propagation_stopped: bool,
    default_stopped: bool,
}
impl Click {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(Click);

#[derive(Clone, Debug)]
pub struct RightClick {
    pub x: f32,
    pub y: f32,
    propagation_stopped: bool,
    default_stopped: bool,
}
impl RightClick {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(RightClick);

#[derive(Clone, Debug)]
pub struct DoubleClick {
    pub x: f32,
    pub y: f32,
    propagation_stopped: bool,
    default_stopped: bool,
}
impl DoubleClick {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(DoubleClick);

#[derive(Clone, Debug)]
pub struct Press {
    pub x: f32,
    pub y: f32,
    propagation_stopped: bool,
    default_stopped: bool,
}
impl Press {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(Press);

#[derive(Clone, Debug)]
pub struct Release {
    pub x: f32,
    pub y: f32,
    propagation_stopped: bool,
    default_stopped: bool,
}
impl Release {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(Release);

#[derive(Clone, Debug)]
pub struct MouseMove {
    pub x: f32,
    pub y: f32,
    propagation_stopped: bool,
    default_stopped: bool,
}
impl MouseMove {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(MouseMove);

#[derive(Clone, Debug)]
pub struct Scroll {
    pub x: f32,
    pub y: f32,
    propagation_stopped: bool,
    default_stopped: bool,
}
impl Scroll {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(Scroll);

#[derive(Clone, Debug)]
pub struct Hover {
    propagation_stopped: bool,
    default_stopped: bool,
}
impl Hover {
    pub fn new() -> Self {
        Self {
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(Hover);

#[derive(Clone, Debug)]
pub struct HoverEnd {
    propagation_stopped: bool,
    default_stopped: bool,
}
impl HoverEnd {
    pub fn new() -> Self {
        Self {
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(HoverEnd);

#[derive(Clone, Debug)]
pub struct FocusGained {
    propagation_stopped: bool,
    default_stopped: bool,
}
impl FocusGained {
    pub fn new() -> Self {
        Self {
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(FocusGained);

#[derive(Clone, Debug)]
pub struct FocusLost {
    propagation_stopped: bool,
    default_stopped: bool,
}
impl FocusLost {
    pub fn new() -> Self {
        Self {
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(FocusLost);

#[derive(Clone, Debug)]
pub struct KeyPress {
    pub key: crate::input::Key,
    pub text: Option<char>,
    pub modifiers: crate::input::Modifiers,
    propagation_stopped: bool,
    default_stopped: bool,
}
impl KeyPress {
    pub fn new(
        key: crate::input::Key,
        text: Option<char>,
        modifiers: crate::input::Modifiers,
    ) -> Self {
        Self {
            key,
            text,
            modifiers,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(KeyPress);

#[derive(Clone, Debug)]
pub struct KeyRelease {
    pub key: crate::input::Key,
    propagation_stopped: bool,
    default_stopped: bool,
}
impl KeyRelease {
    pub fn new(key: crate::input::Key) -> Self {
        Self {
            key,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(KeyRelease);

#[derive(Clone, Debug)]
pub struct Change {
    pub value: String,
    propagation_stopped: bool,
    default_stopped: bool,
}
impl Change {
    pub fn new(value: String) -> Self {
        Self {
            value,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl_event!(Change);

// connection storage 

pub(crate) struct Connection {
    pub id: u32,
    pub callback: Box<dyn FnMut(&mut super::Ui, &mut dyn Event)>,
}

pub(crate) struct ConnectionList {
    pub external: Vec<Connection>,
    pub internal: Vec<Connection>,
}

impl ConnectionList {
    pub fn new() -> Self {
        Self {
            external: Vec::new(),
            internal: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.external.is_empty() && self.internal.is_empty()
    }
}

// event queue 

pub(crate) struct QueuedEvent {
    pub handle: Handle<()>,
    pub event: Box<dyn Event>,
    pub remaining_chain: Vec<Handle<()>>,
}

// event system 

pub struct EventSystem {
    pub(crate) connections: HashMap<Handle<()>, ConnectionList>,
    pub(crate) event_queue: Vec<QueuedEvent>,
    pub(crate) next_connection_id: u32,
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            event_queue: Vec::new(),
            next_connection_id: 0,
        }
    }

    pub(crate) fn connect_external<T>(
        &mut self,
        handle: Handle<T>,
        callback: impl FnMut(&mut super::Ui, &mut dyn Event) + 'static,
    ) -> u32 {
        let id = self.next_connection_id;
        self.next_connection_id += 1;
        self.connections
            .entry(handle.untyped())
            .or_insert_with(ConnectionList::new)
            .external
            .push(Connection {
                id,
                callback: Box::new(callback),
            });
        id
    }

    pub(crate) fn connect_internal<T>(
        &mut self,
        handle: Handle<T>,
        callback: impl FnMut(&mut super::Ui, &mut dyn Event) + 'static,
    ) -> u32 {
        let id = self.next_connection_id;
        self.next_connection_id += 1;
        self.connections
            .entry(handle.untyped())
            .or_insert_with(ConnectionList::new)
            .internal
            .push(Connection {
                id,
                callback: Box::new(callback),
            });
        id
    }

    pub fn disconnect(&mut self, connection_id: u32) {
        for list in self.connections.values_mut() {
            list.external.retain(|c| c.id != connection_id);
            list.internal.retain(|c| c.id != connection_id);
        }
    }

    pub fn has_connections(&self, handle: Handle<()>) -> bool {
        self.connections
            .get(&handle)
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }

    pub fn emit<E: Event>(&mut self, handle: Handle<()>, event: E) {
        self.event_queue.push(QueuedEvent {
            handle,
            event: Box::new(event),
            remaining_chain: Vec::new(),
        });
    }

    pub fn emit_bubbling<E: Event>(&mut self, event: E, chain: Vec<Handle<()>>) {
        if chain.is_empty() {
            return;
        }
        let mut remaining = chain;
        let first = remaining.remove(0);
        self.event_queue.push(QueuedEvent {
            handle: first,
            event: Box::new(event),
            remaining_chain: remaining,
        });
    }
}
