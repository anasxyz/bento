use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;

use bento_shared::Scene;

use super::EventQueue;
use super::{KeyPress, KeyRelease};
use crate::Input;
use crate::{Widget, WidgetHandle};

/// Slot in the UI where a widget lives.
pub struct Slot {
    pub widget: Box<dyn Widget>,
    pub generation: u32,
}

/// Main orchestrator of anything UI / Event related.
pub struct Ui {
    scene: Scene,
    slots: Vec<Option<Slot>>,
    connections: HashMap<Option<u32>, Vec<(u64, TypeId, Box<dyn Fn(&dyn Any, &mut Ui)>)>>,
    next_connection_id: u64,
    pending_removals: Vec<(Option<u32>, u64)>,

    // Input stuff
    // Controls state for Mouse, Keyboard, etc.
    pub input: Input,

    // Async stuff
    pub events: EventQueue,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            slots: Vec::new(),
            connections: HashMap::new(),
            next_connection_id: 0,
            pending_removals: Vec::new(),

            input: Input::new(),

            events: EventQueue::new(),
        }
    }

    /// Adds a widget to the UI and returns a handle to it.
    ///
    /// Iterates through slots until it finds a none/empty/free slot to be reused and returns
    /// its position/index as Option<usize>. If no slot is found, return the end of the slots vector as the index.
    /// Once index is found, creates a new slot at the index and adds the widget to it.
    ///
    /// Returns a WidgetHandle to the added the widget.
    pub fn add<W: Widget + 'static>(&mut self, mut widget: W) -> WidgetHandle<W> {
        // Iterate through slots until it finds a none/empty/free slot to be reused
        // and return its position/index as Option<usize>
        // If no slot is found, return the end of the slots vector as the index
        let index = self
            .slots
            .iter()
            .position(|s| s.is_none())
            .unwrap_or(self.slots.len());

        // Build the widget using its iternal build() method
        widget.build(&mut self.scene);

        // Create a new slot at the index and add the widget to it
        let slot = Slot {
            widget: Box::new(widget),
            generation: 0,
        };

        // If the index is the end of the slots vector, add the slot
        // Otherwise, replace the slot at the index with the new slot
        if index == self.slots.len() {
            self.slots.push(Some(slot));
        } else {
            self.slots[index] = Some(slot);
        }

        // Widget ID is the index of the slot
        WidgetHandle::new(index as u32, 0)
    }

    /// Removes a widget from the UI.
    ///
    /// Gets widget's slot using its ID, which was assigned by the add() method, and sets its slot
    /// None. This allows the slot to be reused by the next widget added to the UI.
    ///
    /// Returns if provided invalid WidgetHandle or if slot is already None.
    pub fn remove<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) {
        // Return if the WidgetHandle provided by user is invalid
        let Some(slot) = self.slots.get_mut(handle.id as usize) else {
            return;
        };

        // Return if slot is None
        let Some(s) = slot.as_mut() else { return };

        // Return if the widget's generation matches the slot's generation
        if s.generation == handle.generation {
            // Call widget's internal remove method
            s.widget.remove(&mut self.scene);

            // Set slot to None to be reused
            *slot = None;
        }
    }

    /// Returns a reference to a widget.
    pub fn get<W: Widget + 'static>(&self, handle: WidgetHandle<W>) -> Option<&W> {
        // Return None if the WidgetHandle provided by user is invalid
        let Some(slot) = self.slots.get(handle.id as usize) else {
            return None;
        };

        // Return None if slot is None
        let Some(s) = slot.as_ref() else { return None };

        // If generations match, downcast to the widget type as a reference
        // Otherwise, return None
        if s.generation == handle.generation {
            s.widget.as_any().downcast_ref::<W>()
        } else {
            None
        }
    }

    /// Returns a mutable reference to a widget.
    pub fn get_mut<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) -> Option<&mut W> {
        // Return None if the WidgetHandle provided by user is invalid
        let Some(slot) = self.slots.get_mut(handle.id as usize) else {
            return None;
        };

        // Return None if slot is None
        let Some(s) = slot.as_mut() else { return None };

        // If generations match, downcast to the widget type as mutable
        // Otherwise, return None
        if s.generation == handle.generation {
            s.widget.as_any_mut().downcast_mut::<W>()
        } else {
            None
        }
    }

    /// Per frame tick.
    ///
    /// Goes through all widgets and calls their update() method.
    /// Processes input.
    ///
    /// TODO: add dirty tacking to only update widgets that have changed.
    pub fn update(&mut self) {
        for slot in self.slots.iter_mut() {
            if let Some(s) = slot.as_mut() {
                s.widget.update(&mut self.scene);
            }
        }

        self.process_input();
    }

    /// Returns a reference to the scene.
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    /// Returns a mutable reference to the scene.
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    /// Looks up a completed async callback by id and runs it with &mut Ui on the main thread.
    ///
    /// Completely ignore for this now, it just has to be here and can't be on EventQueue because
    /// of the self parameter in the callbac.
    pub fn fire_callback(&mut self, id: u64) {
        if let Some(callback) = self.events.callbacks.remove(&id) {
            callback(self);
        } else {
            let callback = self.events.async_callbacks.lock().unwrap().remove(&id);
            if let Some(callback) = callback {
                callback(self);
            }
        }
    }
}

/// Connection handle for managing connections
pub struct ConnectionHandle {
    slot_id: Option<u32>,
    id: u64,
}

/// Event stuff
/// Moved to separate impl block purely for organisation
impl Ui {
    fn process_input(&mut self) {
        let key_presses: Vec<KeyPress> = self
            .input
            .keyboard
            .just_pressed()
            .iter()
            .map(|(key, ch)| KeyPress { key: *key, ch: *ch })
            .collect();

        let key_releases: Vec<KeyRelease> = self
            .input
            .keyboard
            .just_released()
            .iter()
            .map(|key| KeyRelease { key: *key })
            .collect();

        let slot_ids: Vec<u32> = self.connections.keys().filter_map(|k| *k).collect();

        for slot_id in &slot_ids {
            for event in &key_presses {
                self.dispatch_by_id(*slot_id, event);
            }
            for event in &key_releases {
                self.dispatch_by_id(*slot_id, event);
            }
        }

        for event in &key_presses {
            self.dispatch_global(event);
        }
        for event in &key_releases {
            self.dispatch_global(event);
        }
    }

    /// Listens for events of type E on widget with handle.
    pub fn listen<W: Widget + 'static, E: 'static>(
        &mut self,
        handle: WidgetHandle<W>,
        f: impl Fn(&E, &mut Ui) + 'static,
    ) -> ConnectionHandle {
        // Assign a unique id to this connection
        let id = self.next_connection_id;
        self.next_connection_id += 1;

        self.connections
            // Use Some(handle.id) as key to associate this handler with a specific widget
            .entry(Some(handle.id))
            .or_insert_with(Vec::new)
            .push((
                id,
                // Store the TypeId of E so dispatch can filter by event type
                TypeId::of::<E>(),
                // Wrap the closure in a type erased box
                // Downcast &dyn Any back to &E before calling the user's closure
                Box::new(move |event, ui| {
                    if let Some(e) = event.downcast_ref::<E>() {
                        f(e, ui);
                    }
                }),
            ));
        ConnectionHandle {
            slot_id: Some(handle.id),
            id,
        }
    }

    /// Listens for events of type E on widget with handle only once.
    ///
    /// After the first event is fired, the connection is removed.
    pub fn listen_once<W: Widget + 'static, E: 'static>(
        &mut self,
        handle: WidgetHandle<W>,
        f: impl Fn(&E, &mut Ui) + 'static,
    ) -> ConnectionHandle {
        let id = self.next_connection_id;
        self.next_connection_id += 1;

        let slot_id = Some(handle.id);
        self.connections
            .entry(slot_id)
            .or_insert_with(Vec::new)
            .push((
                id,
                TypeId::of::<E>(),
                Box::new(move |event, ui| {
                    if let Some(e) = event.downcast_ref::<E>() {
                        f(e, ui);
                        // Remove this connection after the first event is fired
                        ui.listen_off(ConnectionHandle { slot_id, id });
                    }
                }),
            ));
        ConnectionHandle { slot_id, id }
    }

    /// Listens for events of type E on any widget.
    pub fn listen_any<E: 'static>(
        &mut self,
        f: impl Fn(&E, &mut Ui) + 'static,
    ) -> ConnectionHandle {
        // Assign a unique id to this connection
        let id = self.next_connection_id;
        self.next_connection_id += 1;

        self.connections
            // Use None as key to signify this is a global handler
            // dispatch_global will pick these up
            .entry(None)
            .or_insert_with(Vec::new)
            .push((
                id,
                // Store the TypeId of E so dispatch can filter by event type
                TypeId::of::<E>(),
                // Wrap the closure in a type erased box
                // Downcast &dyn Any back to &E before calling the user's closure
                Box::new(move |event, ui| {
                    if let Some(e) = event.downcast_ref::<E>() {
                        f(e, ui);
                    }
                }),
            ));
        ConnectionHandle { slot_id: None, id }
    }

    /// Listens for events of type E on any widget only once.
    ///
    /// After the first event is fired, the connection is removed.
    pub fn listen_any_once<E: 'static>(
        &mut self,
        f: impl Fn(&E, &mut Ui) + 'static,
    ) -> ConnectionHandle {
        let id = self.next_connection_id;
        self.next_connection_id += 1;

        self.connections.entry(None).or_insert_with(Vec::new).push((
            id,
            TypeId::of::<E>(),
            Box::new(move |event, ui| {
                if let Some(e) = event.downcast_ref::<E>() {
                    f(e, ui);
                    // Remove this connection after the first event is fired
                    ui.listen_off(ConnectionHandle { slot_id: None, id });
                }
            }),
        ));
        ConnectionHandle { slot_id: None, id }
    }

    /// Removes a connection from the UI.
    pub fn listen_off(&mut self, handle: ConnectionHandle) {
        self.pending_removals.push((handle.slot_id, handle.id));
    }

    /// Fires correct handlers for widget and event type.
    ///
    /// Searches the UI's connections for handlers registered to the widget with
    /// the provided slot_id
    /// If found any, it checks if the type_id of the event matches the type_id
    /// of the handler's type parameter, meaning it's the exact same event type.
    /// This skips the handlers that don't match the event being disptached at the moment.
    /// Finally iterate over and call the handlers.
    fn dispatch_by_id(&mut self, slot_id: u32, event: &dyn Any) {
        let connections = std::mem::take(&mut self.connections);

        // Type id of this event's struct
        let type_id = event.type_id();

        // Get widget specific handlers
        if let Some(handlers) = connections.get(&Some(slot_id)) {
            // Iterate over all handlers registered to the widget
            for (_, tid, f) in handlers {
                // Filter out handlers that don't match the event type
                if *tid == type_id {
                    f(event, self);
                }
            }
        }

        self.connections = connections;

        // Apply any pending removals requested during dispatch
        for (slot_id, id) in self.pending_removals.drain(..) {
            if let Some(vec) = self.connections.get_mut(&slot_id) {
                vec.retain(|(cid, _, _)| *cid != id);
            }
        }
    }

    /// Fires handlers registered globally.
    ///
    /// Works similarly to Ui::dispatch_by_id(), but instead of searching for a widget with the provided
    /// slot_id, it searches for a handler registered to no widget.
    fn dispatch_global(&mut self, event: &dyn Any) {
        let connections = std::mem::take(&mut self.connections);

        // Type id of this event's struct
        let type_id = event.type_id();

        // Get handlers registered to no widget, which are meant for broadcasting
        if let Some(handlers) = connections.get(&None) {
            for (_, tid, f) in handlers {
                if *tid == type_id {
                    f(event, self);
                }
            }
        }

        self.connections = connections;

        // Apply any pending removals requested during dispatch
        for (slot_id, id) in self.pending_removals.drain(..) {
            if let Some(vec) = self.connections.get_mut(&slot_id) {
                vec.retain(|(cid, _, _)| *cid != id);
            }
        }
    }

    /// Emits an event from a widget.
    ///
    /// Purely for convenience.
    /// This is the same as calling Ui::dispatch_by_id() with the widget's id.
    pub fn emit<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>, event: &dyn Any) {
        self.dispatch_by_id(handle.id, event);
    }
}

// For debug
impl fmt::Display for Ui {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Ui ({} slots):", self.slots.len())?;
        for (i, slot) in self.slots.iter().enumerate() {
            match slot {
                Some(s) => writeln!(f, "  [{}] {} gen={}", i, s.widget.name(), s.generation)?,
                None => writeln!(f, "  [{}] empty", i)?,
            }
        }
        Ok(())
    }
}
