// ui/mod.rs
//
// Ui owns the widget tree, layout engine, scene graph, and event system
// for one window. The user builds their UI here and passes it to
// app.open_window().

mod events;
mod slot;
mod tree;
mod update;

pub use events::Event;

use crate::layout::LayoutEngine;
use crate::widget::Handle;
use bento_wgpu::SceneGraph;

use events::EventSystem;
use slot::Slot;

const GLOBAL_ID: u32 = u32::MAX;

/// Tracks which widget currently has hover/press/focus.
pub struct InteractionState {
    pub hovered: Option<Handle<()>>,
    pub pressed: Option<Handle<()>>,
    pub focused: Option<Handle<()>>,
}

impl InteractionState {
    fn new() -> Self {
        Self {
            hovered: None,
            pressed: None,
            focused: None,
        }
    }
}

pub struct Ui {
    pub(crate) slots: Vec<Option<Slot>>,
    pub(crate) layout: LayoutEngine,
    pub(crate) scene: SceneGraph,
    pub(crate) events: EventSystem,
    pub(crate) interaction: InteractionState,
    pub(crate) root: Option<Handle<()>>,
    pub window_width: u32,
    pub window_height: u32,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            layout: LayoutEngine::new(),
            scene: SceneGraph::new(),
            events: EventSystem::new(),
            interaction: InteractionState::new(),
            root: None,
            window_width: 0,
            window_height: 0,
        }
    }

    pub fn global(&self) -> Handle<()> {
        Handle::new(GLOBAL_ID, 0)
    }

    // ── event API (delegates to EventSystem) ─────────────────────────────────

    pub fn connect<T>(
        &mut self,
        handle: Handle<T>,
        callback: impl FnMut(&mut Ui, &Event) + 'static,
    ) -> u32 {
        self.events.connect(handle, callback)
    }

    pub fn disconnect(&mut self, connection_id: u32) {
        self.events.disconnect(connection_id);
    }

    pub fn has_connections(&self, handle: Handle<()>) -> bool {
        self.events.has_connections(handle)
    }

    pub fn emit<T>(&mut self, handle: Handle<T>, event: Event) {
        self.events.emit(handle, event);
    }

    pub fn emit_bubbling<T>(&mut self, handle: Handle<T>, event: Event) {
        let global = self.global();
        let handle = handle.untyped();
        // build ancestor chain first to avoid borrow conflict
        let mut chain = vec![handle];
        let mut current = self.parent(handle);
        while let Some(p) = current {
            chain.push(p);
            current = self.parent(p);
        }
        chain.push(global);
        for ancestor in chain {
            self.events.event_queue.push((ancestor, event.clone()));
        }
    }

    pub fn drain_events(&mut self) {
        while !self.events.event_queue.is_empty() {
            let queue = std::mem::take(&mut self.events.event_queue);
            for (handle, event) in queue {
                // take connections for this handle out to avoid borrow conflict
                let Some(mut conns) = self.events.connections.remove(&handle) else {
                    continue;
                };
                for conn in &mut conns {
                    (conn.callback)(self, &event);
                }
                // put connections back — merge in case new ones were added during callback
                let entry = self.events.connections.entry(handle).or_default();
                // prepend existing conns back (callbacks added during drain go at end)
                let new_conns = std::mem::take(entry);
                *entry = conns;
                entry.extend(new_conns);
            }
        }
    }

    // ── mouse helpers ─────────────────────────────────────────────────────────

    pub fn hovered(&self) -> Option<Handle<()>> {
        self.interaction.hovered
    }
    pub fn pressed(&self) -> Option<Handle<()>> {
        self.interaction.pressed
    }
    pub fn focused(&self) -> Option<Handle<()>> {
        self.interaction.focused
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}
