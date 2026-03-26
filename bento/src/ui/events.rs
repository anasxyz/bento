use crate::widget::Handle;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Event {
    Click { x: f32, y: f32 },
    RightClick { x: f32, y: f32 },
    DoubleClick { x: f32, y: f32 },
    Press { x: f32, y: f32 },
    Release { x: f32, y: f32 },
    MouseMove { x: f32, y: f32 },
    Scroll { x: f32, y: f32 },
    Hover,
    HoverEnd,
    FocusGained,
    FocusLost,
    KeyPress { key: String, text: Option<char> },
    KeyRelease { key: String },
    Change(String),
    Custom(u32),
}

pub struct Connection {
    pub id: u32,
    pub callback: Box<dyn FnMut(&mut super::Ui, &Event)>,
}

pub struct EventSystem {
    // handle => list of connections
    pub(crate) connections: HashMap<Handle<()>, Vec<Connection>>,
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

    pub fn connect<T>(
        &mut self,
        handle: Handle<T>,
        callback: impl FnMut(&mut super::Ui, &Event) + 'static,
    ) -> u32 {
        let id = self.next_connection_id;
        self.next_connection_id += 1;
        self.connections
            .entry(handle.untyped())
            .or_default()
            .push(Connection {
                id,
                callback: Box::new(callback),
            });
        id
    }

    pub fn disconnect(&mut self, connection_id: u32) {
        for conns in self.connections.values_mut() {
            conns.retain(|c| c.id != connection_id);
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
