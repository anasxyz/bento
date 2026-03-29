use crate::input::Key;
use crate::widget::Handle;
use std::collections::HashMap;

// typed event structs

#[derive(Clone, Debug)]
pub struct ClickEvent {
    pub x: f32,
    pub y: f32,
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct RightClickEvent {
    pub x: f32,
    pub y: f32,
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct DoubleClickEvent {
    pub x: f32,
    pub y: f32,
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct PressEvent {
    pub x: f32,
    pub y: f32,
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct ReleaseEvent {
    pub x: f32,
    pub y: f32,
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct MouseMoveEvent {
    pub x: f32,
    pub y: f32,
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct ScrollEvent {
    pub x: f32,
    pub y: f32,
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct HoverEvent {
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct HoverEndEvent {
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct FocusEvent {
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct BlurEvent {
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct KeyPressEvent {
    pub key: Key,
    pub text: Option<char>,
    pub modifiers: crate::input::Modifiers,
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct KeyReleaseEvent {
    pub key: Key,
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

#[derive(Clone, Debug)]
pub struct ChangeEvent {
    pub value: String,
    pub(crate) propagation_stopped: bool,
    pub(crate) default_stopped: bool,
}

// propagation control — implemented on all event structs via macro

macro_rules! impl_event {
    ($t:ty) => {
        impl $t {
            pub fn stop_propagation(&mut self) {
                self.propagation_stopped = true;
            }
            pub fn stop_default(&mut self) {
                self.default_stopped = true;
            }
            pub fn is_propagation_stopped(&self) -> bool {
                self.propagation_stopped
            }
            pub fn is_default_stopped(&self) -> bool {
                self.default_stopped
            }
        }
    };
}

impl_event!(ClickEvent);
impl_event!(RightClickEvent);
impl_event!(DoubleClickEvent);
impl_event!(PressEvent);
impl_event!(ReleaseEvent);
impl_event!(MouseMoveEvent);
impl_event!(ScrollEvent);
impl_event!(HoverEvent);
impl_event!(HoverEndEvent);
impl_event!(FocusEvent);
impl_event!(BlurEvent);
impl_event!(KeyPressEvent);
impl_event!(KeyReleaseEvent);
impl_event!(ChangeEvent);

// constructors

impl ClickEvent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl RightClickEvent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl DoubleClickEvent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl PressEvent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl ReleaseEvent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl MouseMoveEvent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl ScrollEvent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl HoverEvent {
    pub fn new() -> Self {
        Self {
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl HoverEndEvent {
    pub fn new() -> Self {
        Self {
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl FocusEvent {
    pub fn new() -> Self {
        Self {
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl BlurEvent {
    pub fn new() -> Self {
        Self {
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl KeyPressEvent {
    pub fn new(key: Key, text: Option<char>, modifiers: crate::input::Modifiers) -> Self {
        Self {
            key,
            text,
            modifiers,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl KeyReleaseEvent {
    pub fn new(key: Key) -> Self {
        Self {
            key,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}
impl ChangeEvent {
    pub fn new(value: String) -> Self {
        Self {
            value,
            propagation_stopped: false,
            default_stopped: false,
        }
    }
}

// the raw Event enum kept for internal use and connect() 

#[derive(Clone, Debug)]
pub enum Event {
    Click(ClickEvent),
    RightClick(RightClickEvent),
    DoubleClick(DoubleClickEvent),
    Press(PressEvent),
    Release(ReleaseEvent),
    MouseMove(MouseMoveEvent),
    Scroll(ScrollEvent),
    Hover(HoverEvent),
    HoverEnd(HoverEndEvent),
    Focus(FocusEvent),
    Blur(BlurEvent),
    KeyPress(KeyPressEvent),
    KeyRelease(KeyReleaseEvent),
    Change(ChangeEvent),
    Custom(u32),
}

pub struct Connection {
    pub id: u32,
    pub callback: Box<dyn FnMut(&mut super::Ui, &mut Event)>,
}

pub struct ConnectionList {
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

    fn is_empty(&self) -> bool {
        self.external.is_empty() && self.internal.is_empty()
    }
}

pub struct EventSystem {
    pub(crate) connections: HashMap<Handle<()>, ConnectionList>,
    pub(crate) event_queue: Vec<(Handle<()>, Event)>,
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

    pub fn connect_external<T>(
        &mut self,
        handle: Handle<T>,
        callback: impl FnMut(&mut super::Ui, &mut Event) + 'static,
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

    pub fn connect_internal<T>(
        &mut self,
        handle: Handle<T>,
        callback: impl FnMut(&mut super::Ui, &mut Event) + 'static,
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

    pub fn emit<T>(&mut self, handle: Handle<T>, event: Event) {
        self.event_queue.push((handle.untyped(), event));
    }
}
