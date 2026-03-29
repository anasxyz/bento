mod events;
mod slot;
mod tree;
mod update;

pub use events::{
    Blur, Change, Click, DoubleClick, Event, Focus, Hover, HoverEnd, KeyPress, KeyRelease,
    MouseMove, Press, Release, RightClick, Scroll,
};

use crate::layout::LayoutEngine;
use crate::widget::{Handle, Widget};
use bento_wgpu::SceneGraph;

use events::{ConnectionList, EventSystem, QueuedEvent};
use slot::Slot;

const GLOBAL_ID: u32 = u32::MAX;

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
    pub(crate) registering: bool,
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
            registering: false,
            window_width: 0,
            window_height: 0,
        }
    }

    pub fn global(&self) -> Handle<()> {
        Handle::new(GLOBAL_ID, 0)
    }

    pub fn has_focused_text_input(&self) -> bool {
        if let Some(focused) = self.interaction.focused {
            if let Some(Some(slot)) = self.slots.get(focused.id as usize) {
                return slot
                    .widget
                    .as_any()
                    .downcast_ref::<crate::widgets::TextInput>()
                    .is_some();
            }
        }
        false
    }

    pub fn toggle_cursor_blink(&mut self) {
        if let Some(focused) = self.interaction.focused {
            if let Some(Some(slot)) = self.slots.get_mut(focused.id as usize) {
                if let Some(input) = slot
                    .widget
                    .as_any_mut()
                    .downcast_mut::<crate::widgets::TextInput>()
                {
                    input.toggle_blink();
                }
            }
        }
    }

    // unified on<E> 
    // works for any type that implements Event, builtin or user defined
    // when called during register() goes to internal list, otherwise external

    pub fn on<W, E>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut E) + 'static,
    ) -> u32
    where
        W: Widget,
        E: Event,
    {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut dyn Event| {
            if let Some(e) = event.as_any_mut().downcast_mut::<E>() {
                let Some(mut slot) = ui.slots.get_mut(h.id as usize).and_then(|s| s.take()) else {
                    return;
                };
                if let Some(w) = slot.widget.as_any_mut().downcast_mut::<W>() {
                    callback(ui, w, e);
                }
                if let Some(s) = ui.slots.get_mut(h.id as usize) {
                    *s = Some(slot);
                }
            }
        };
        if self.registering {
            self.events.connect_internal(handle, cb)
        } else {
            self.events.connect_external(handle, cb)
        }
    }

    pub fn disconnect(&mut self, connection_id: u32) {
        self.events.disconnect(connection_id);
    }

    pub fn has_connections(&self, handle: Handle<()>) -> bool {
        self.events.has_connections(handle)
    }

    // emit 

    pub fn emit<T, E: Event>(&mut self, handle: Handle<T>, event: E) {
        self.events.emit(handle.untyped(), event);
    }

    pub fn emit_bubbling<T, E: Event + Clone>(&mut self, handle: Handle<T>, event: E) {
        let global = self.global();
        let handle = handle.untyped();
        let mut chain = vec![handle];
        let mut current = self.parent(handle);
        while let Some(p) = current {
            chain.push(p);
            current = self.parent(p);
        }
        chain.push(global);
        for (i, ancestor) in chain.iter().enumerate() {
            // clone for all but last
            if i < chain.len() - 1 {
                self.events.emit(*ancestor, event.clone());
            } else {
                self.events.emit(*ancestor, event.clone());
            }
        }
    }

    // drain 

    pub fn drain_events(&mut self) {
        while !self.events.event_queue.is_empty() {
            let queue = std::mem::take(&mut self.events.event_queue);
            for mut queued in queue {
                let handle = queued.handle;
                let event = &mut *queued.event;

                let Some(mut list) = self.events.connections.remove(&handle) else {
                    continue;
                };

                // external first
                for conn in &mut list.external {
                    if event.is_propagation_stopped() {
                        break;
                    }
                    (conn.callback)(self, event);
                }

                // internal only if default not stopped
                if !event.is_default_stopped() {
                    for conn in &mut list.internal {
                        if event.is_propagation_stopped() {
                            break;
                        }
                        (conn.callback)(self, event);
                    }
                }

                // put list back, merging any new connections added during drain
                let entry = self
                    .events
                    .connections
                    .entry(handle)
                    .or_insert_with(ConnectionList::new);
                let new_list = std::mem::replace(entry, list);
                entry.external.extend(new_list.external);
                entry.internal.extend(new_list.internal);
            }
        }
    }

    // interaction helpers 

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
