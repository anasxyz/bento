use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;

use bento_shared::{Scene, TextMeasurer};

use super::AsyncEventQueue;
use super::{
    Click, FocusGained, FocusLost, HoverEnter, HoverLeave, KeyPress, KeyRelease, MouseDown,
    MouseEnter, MouseLeave, MouseMove, MouseScroll, MouseUp,
};
use crate::{Input, Key, MouseButton, Widget, WidgetHandle, WidgetId};

/// Slot in the UI where a widget lives.
pub struct Slot {
    pub widget: Box<dyn Widget>,
    pub generation: u32,
    pub children: Vec<u32>,
    pub parent: Option<u32>,
}

/// A single registered listener.
struct Listener {
    id: u64,
    type_id: TypeId,
    f: Box<dyn FnMut(&dyn Any, &mut Ui) -> bool>,
}

/// A pending event waiting to be dispatched.
struct PendingEvent {
    // None = global
    target: Option<u32>,
    event: Box<dyn Any>,
    type_id: TypeId,
}

/// Handle returned from listen calls, used to unsubscribe.
#[derive(Clone, Copy)]
pub struct ListenerHandle {
    target: Option<u32>,
    id: u64,
}

/// Main orchestrator of anything UI / Event related.
pub struct Ui {
    scene: Scene,
    slots: Vec<Option<Slot>>,
    focused: Option<u32>,

    listeners: HashMap<Option<u32>, Vec<Listener>>,
    next_listener_id: u64,
    pending_events: Vec<PendingEvent>,
    pending_event_removals: Vec<u64>,
    pending_widget_removals: Vec<u32>,

    pub input: Input,
    pub asyncs: AsyncEventQueue,

    pub needs_redraw: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            slots: Vec::new(),
            focused: None,
            listeners: HashMap::new(),
            next_listener_id: 0,
            pending_events: Vec::new(),
            pending_widget_removals: Vec::new(),
            pending_event_removals: Vec::new(),
            input: Input::new(),
            asyncs: AsyncEventQueue::new(),
            needs_redraw: false,
        }
    }

    /// Adds a widget to the UI.
    /// Returns a handle to the widget.
    pub fn add<W: Widget + 'static>(&mut self, mut widget: W) -> WidgetHandle<W> {
        let index = self.slots.len();
        self.slots.push(None);

        widget.set_handle(index as u32, 0);
        widget.build(self);

        self.slots[index] = Some(Slot {
            widget: Box::new(widget),
            generation: 0,
            children: Vec::new(),
            parent: None,
        });

        WidgetHandle::new(index as u32, 0)
    }

    /// Removes a widget from the UI
    /// Deferred to end of frame.
    pub fn remove<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) {
        let Some(Some(s)) = self.slots.get(handle.id as usize) else {
            return;
        };
        if s.generation == handle.generation {
            self.pending_widget_removals.push(handle.id);
        }
    }

    /// Returns a reference to a widget.
    pub fn get<W: Widget + 'static>(&self, handle: WidgetHandle<W>) -> Option<&W> {
        self.slots
            .get(handle.id as usize)?
            .as_ref()
            .filter(|s| s.generation == handle.generation)?
            .widget
            .as_any()
            .downcast_ref::<W>()
    }

    /// Returns a mutable reference to a widget.
    pub fn get_mut<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) -> Option<&mut W> {
        self.slots
            .get_mut(handle.id as usize)?
            .as_mut()
            .filter(|s| s.generation == handle.generation)?
            .widget
            .as_any_mut()
            .downcast_mut::<W>()
    }

    /// Updates all dirty widgets.
    pub fn update(&mut self, measurer: &mut dyn TextMeasurer) {
        let dirty: Vec<usize> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().filter(|s| s.widget.is_dirty()).map(|_| i))
            .collect();

        for i in dirty {
            let mut slot = self.slots[i].take().unwrap();
            slot.widget.update(self, measurer);
            slot.widget.set_dirty(false);
            self.slots[i] = Some(slot);
        }
    }

    /// Sets the children of a widget, reparenting their scene roots.
    pub fn set_children<W: Widget + 'static>(
        &mut self,
        handle: WidgetHandle<W>,
        children: impl IntoIterator<Item = impl WidgetId>,
    ) {
        let child_ids: Vec<u32> = children.into_iter().map(|c| c.id()).collect();
        let group_id = self
            .slots
            .get(handle.id as usize)
            .and_then(|s| s.as_ref())
            .and_then(|s| s.widget.scene_root());

        for &child_id in &child_ids {
            let child_root = self
                .slots
                .get(child_id as usize)
                .and_then(|s| s.as_ref())
                .and_then(|s| s.widget.scene_root());

            if let (Some(group_id), Some(child_root)) = (group_id, child_root) {
                self.scene.reparent(child_root, group_id);
            }

            if let Some(Some(child_slot)) = self.slots.get_mut(child_id as usize) {
                child_slot.parent = Some(handle.id);
            }
        }

        if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
            slot.children = child_ids;
        }
    }

    /// Returns true if any widget is dirty.
    pub fn any_dirty(&self) -> bool {
        self.slots.iter().flatten().any(|s| s.widget.is_dirty())
    }

    /// Returns a reference to the scene.
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    /// Returns a mutable reference to the scene.
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}

/// Listener registration and event emission
impl Ui {
    /// Registers a listener.
    /// Used internally.
    /// Returns a handle to the listener.
    fn register(
        &mut self,
        target: Option<u32>,
        type_id: TypeId,
        f: Box<dyn FnMut(&dyn Any, &mut Ui) -> bool>,
    ) -> ListenerHandle {
        let id = self.next_listener_id;
        self.next_listener_id += 1;

        self.listeners
            .entry(target)
            .or_default()
            .push(Listener { id, type_id, f });

        ListenerHandle { target, id }
    }

    /// Listen for event E on a specific widget.
    /// Returns a handle to the listener.
    pub fn listen<W: Widget + 'static, E: 'static>(
        &mut self,
        handle: WidgetHandle<W>,
        mut f: impl FnMut(&E, &mut Ui) + 'static,
    ) -> ListenerHandle {
        self.register(
            Some(handle.id),
            TypeId::of::<E>(),
            Box::new(move |event, ui| {
                if let Some(e) = event.downcast_ref::<E>() {
                    f(e, ui);
                }
                true
            }),
        )
    }

    /// Listen for event E on a specific widget, once.
    /// Returns a handle to the listener.
    pub fn listen_once<W: Widget + 'static, E: 'static>(
        &mut self,
        handle: WidgetHandle<W>,
        mut f: impl FnMut(&E, &mut Ui) + 'static,
    ) -> ListenerHandle {
        self.register(
            Some(handle.id),
            TypeId::of::<E>(),
            Box::new(move |event, ui| {
                if let Some(e) = event.downcast_ref::<E>() {
                    f(e, ui);
                }
                false
            }),
        )
    }

    /// Listen for event E on a specific widget while the closure returns true.
    /// Returns a handle to the listener.
    pub fn listen_while<W: Widget + 'static, E: 'static>(
        &mut self,
        handle: WidgetHandle<W>,
        mut f: impl FnMut(&E, &mut Ui) -> bool + 'static,
    ) -> ListenerHandle {
        self.register(
            Some(handle.id),
            TypeId::of::<E>(),
            Box::new(move |event, ui| {
                if let Some(e) = event.downcast_ref::<E>() {
                    return f(e, ui);
                }
                true
            }),
        )
    }

    /// Listen for event E globally.
    /// Returns a handle to the listener.
    pub fn listen_global<E: 'static>(
        &mut self,
        mut f: impl FnMut(&E, &mut Ui) + 'static,
    ) -> ListenerHandle {
        self.register(
            None,
            TypeId::of::<E>(),
            Box::new(move |event, ui| {
                if let Some(e) = event.downcast_ref::<E>() {
                    f(e, ui);
                }
                true
            }),
        )
    }

    /// Listen for event E globally, once.
    /// Returns a handle to the listener.
    pub fn listen_global_once<E: 'static>(
        &mut self,
        mut f: impl FnMut(&E, &mut Ui) + 'static,
    ) -> ListenerHandle {
        self.register(
            None,
            TypeId::of::<E>(),
            Box::new(move |event, ui| {
                if let Some(e) = event.downcast_ref::<E>() {
                    f(e, ui);
                }
                false
            }),
        )
    }

    /// Listen for event E globally while the closure returns true.
    /// Returns a handle to the listener.
    pub fn listen_global_while<E: 'static>(
        &mut self,
        mut f: impl FnMut(&E, &mut Ui) -> bool + 'static,
    ) -> ListenerHandle {
        self.register(
            None,
            TypeId::of::<E>(),
            Box::new(move |event, ui| {
                if let Some(e) = event.downcast_ref::<E>() {
                    return f(e, ui);
                }
                true
            }),
        )
    }

    /// Unsubscribe a listener.
    pub fn listen_off(&mut self, handle: ListenerHandle) {
        self.pending_event_removals.push(handle.id);
    }

    /// Broadcasts an event to listeners on a specific widget.
    pub fn send_to<W: Widget + 'static, E: Any>(&mut self, handle: WidgetHandle<W>, event: E) {
        self.pending_events.push(PendingEvent {
            target: Some(handle.id),
            type_id: TypeId::of::<E>(),
            event: Box::new(event),
        });
    }

    /// Broadcasts an event globally.
    pub fn send_global<E: Any>(&mut self, event: E) {
        self.pending_events.push(PendingEvent {
            target: None,
            type_id: TypeId::of::<E>(),
            event: Box::new(event),
        });
    }

    /// Drain the pending event queue and dispatch all events.
    /// Called once per frame after input processing.
    fn flush_events(&mut self) {
        let mut i = 0;
        while i < self.pending_events.len() {
            let pending = self.pending_events.remove(i);
            self.dispatch_one(pending);
            // no increment of i bc an element was removed meaning next item is now at i
        }

        // process deferred widget removals
        let mut removals = std::mem::take(&mut self.pending_widget_removals);
        removals.dedup();
        for id in &removals {
            if let Some(Some(slot)) = self.slots.get_mut(*id as usize).map(|s| s.take()) {
                let mut slot = slot;
                slot.widget.remove(self);
            }
        }

        if !removals.is_empty() {
            println!("setting needs_redraw, removals={:?}", removals);
            self.needs_redraw = true;
        }
    }

    /// Dispatches a single event.
    fn dispatch_one(&mut self, pending: PendingEvent) {
        let listeners = self.listeners.remove(&pending.target).unwrap_or_default();
        let mut remaining = Vec::new();

        for mut listener in listeners {
            if listener.type_id == pending.type_id {
                let keep = (listener.f)(pending.event.as_ref(), self);
                if keep {
                    remaining.push(listener);
                }
            } else {
                remaining.push(listener);
            }
        }

        remaining.retain(|l| !self.pending_event_removals.contains(&l.id));
        self.pending_event_removals.clear();

        self.listeners
            .entry(pending.target)
            .or_default()
            .extend(remaining.drain(..));
    }
}

/// Focus helpers
impl Ui {
    /// Sets focus on a widget.
    fn set_focus_by_id(&mut self, slot_id: u32) {
        if let Some(prev) = self.focused {
            if let Some(Some(slot)) = self.slots.get_mut(prev as usize) {
                slot.widget.set_focused(false);
            }
            self.pending_events.push(PendingEvent {
                target: Some(prev),
                type_id: TypeId::of::<FocusLost>(),
                event: Box::new(FocusLost),
            });
        }
        self.focused = Some(slot_id);
        if let Some(Some(slot)) = self.slots.get_mut(slot_id as usize) {
            slot.widget.set_focused(true);
        }
        self.pending_events.push(PendingEvent {
            target: Some(slot_id),
            type_id: TypeId::of::<FocusGained>(),
            event: Box::new(FocusGained),
        });
    }

    /// Clears focus from the currently focused widget.
    fn clear_focus(&mut self) {
        if let Some(prev) = self.focused {
            if let Some(Some(slot)) = self.slots.get_mut(prev as usize) {
                slot.widget.set_focused(false);
            }
            self.pending_events.push(PendingEvent {
                target: Some(prev),
                type_id: TypeId::of::<FocusLost>(),
                event: Box::new(FocusLost),
            });
        }
        self.focused = None;
    }
}

/// Input processing
impl Ui {
    pub fn process_input(&mut self) {
        let events = self.collect_events();
        if self
            .input
            .keyboard
            .just_pressed()
            .iter()
            .any(|(k, _)| *k == Key::D)
        {
            println!("{}", self);
        }
        self.queue_input_events(&events);
        self.flush_events();
    }

    fn collect_events(&self) -> InputEvents {
        let key_presses = self
            .input
            .keyboard
            .just_pressed()
            .iter()
            .map(|(key, ch)| KeyPress { key: *key, ch: *ch })
            .collect();

        let key_releases = self
            .input
            .keyboard
            .just_released()
            .iter()
            .map(|key| KeyRelease { key: *key })
            .collect();

        let mouse_move = if self.input.mouse.dx != 0.0 || self.input.mouse.dy != 0.0 {
            Some(MouseMove {
                x: self.input.mouse.x,
                y: self.input.mouse.y,
                dx: self.input.mouse.dx,
                dy: self.input.mouse.dy,
            })
        } else {
            None
        };

        let mut mouse_downs = Vec::new();
        let mut mouse_ups = Vec::new();
        let mut clicks = Vec::new();
        for btn in [MouseButton::Left, MouseButton::Middle, MouseButton::Right] {
            let state = match btn {
                MouseButton::Left => &self.input.mouse.left,
                MouseButton::Right => &self.input.mouse.right,
                MouseButton::Middle => &self.input.mouse.middle,
            };
            if state.just_pressed {
                mouse_downs.push(MouseDown {
                    x: self.input.mouse.x,
                    y: self.input.mouse.y,
                    button: btn,
                });
            }
            if state.just_released {
                mouse_ups.push(MouseUp {
                    x: self.input.mouse.x,
                    y: self.input.mouse.y,
                    button: btn,
                });
                clicks.push(Click {
                    x: self.input.mouse.x,
                    y: self.input.mouse.y,
                    button: btn,
                });
            }
        }

        let mouse_scroll = if self.input.mouse.scroll_x != 0.0 || self.input.mouse.scroll_y != 0.0 {
            Some(MouseScroll {
                x: self.input.mouse.scroll_x,
                y: self.input.mouse.scroll_y,
            })
        } else {
            None
        };

        InputEvents {
            key_presses,
            key_releases,
            mouse_move,
            mouse_downs,
            mouse_ups,
            clicks,
            mouse_scroll,
            mouse_enter: self.input.mouse.just_entered,
            mouse_leave: self.input.mouse.just_left,
        }
    }

    fn queue_input_events(&mut self, events: &InputEvents) {
        // keyboard -> focused widget and global
        if let Some(focused_id) = self.focused {
            for e in &events.key_presses {
                self.pending_events.push(PendingEvent {
                    target: Some(focused_id),
                    type_id: TypeId::of::<KeyPress>(),
                    event: Box::new(*e),
                });
            }
            for e in &events.key_releases {
                self.pending_events.push(PendingEvent {
                    target: Some(focused_id),
                    type_id: TypeId::of::<KeyRelease>(),
                    event: Box::new(*e),
                });
            }
        }

        // mouse -> per widget based on hit test
        let slot_ids: Vec<u32> = self.listeners.keys().filter_map(|k| *k).collect();
        for slot_id in &slot_ids {
            let hit = if let Some(Some(slot)) = self.slots.get(*slot_id as usize) {
                let (x, y, w, h) = slot.widget.bounds();
                let scene_root = slot.widget.scene_root();

                let (sx, sy, sw, sh, clip) = if let Some(root_id) = scene_root {
                    self.scene.screen_bounds(root_id, x, y, w, h)
                } else {
                    (x, y, w, h, None)
                };

                let in_bounds = self.input.mouse.x >= sx
                    && self.input.mouse.x <= sx + sw
                    && self.input.mouse.y >= sy
                    && self.input.mouse.y <= sy + sh;

                let in_clip = clip
                    .map(|[cx, cy, cw, ch]| {
                        self.input.mouse.x >= cx
                            && self.input.mouse.x <= cx + cw
                            && self.input.mouse.y >= cy
                            && self.input.mouse.y <= cy + ch
                    })
                    .unwrap_or(true);

                in_bounds && in_clip
            } else {
                false
            };

            if hit {
                if let Some(Some(slot)) = self.slots.get_mut(*slot_id as usize) {
                    if slot.widget.hoverable() && !slot.widget.is_hovered() {
                        slot.widget.set_hovered(true);
                        self.pending_events.push(PendingEvent {
                            target: Some(*slot_id),
                            type_id: TypeId::of::<HoverEnter>(),
                            event: Box::new(HoverEnter),
                        });
                    }
                }
                if let Some(e) = &events.mouse_move {
                    self.pending_events.push(PendingEvent {
                        target: Some(*slot_id),
                        type_id: TypeId::of::<MouseMove>(),
                        event: Box::new(*e),
                    });
                }
                for e in &events.mouse_downs {
                    self.pending_events.push(PendingEvent {
                        target: Some(*slot_id),
                        type_id: TypeId::of::<MouseDown>(),
                        event: Box::new(*e),
                    });
                }
                for e in &events.mouse_ups {
                    self.pending_events.push(PendingEvent {
                        target: Some(*slot_id),
                        type_id: TypeId::of::<MouseUp>(),
                        event: Box::new(*e),
                    });
                }
                for e in &events.clicks {
                    self.pending_events.push(PendingEvent {
                        target: Some(*slot_id),
                        type_id: TypeId::of::<Click>(),
                        event: Box::new(*e),
                    });
                }
            } else if let Some(Some(slot)) = self.slots.get_mut(*slot_id as usize) {
                if slot.widget.hoverable() && slot.widget.is_hovered() {
                    slot.widget.set_hovered(false);
                    self.pending_events.push(PendingEvent {
                        target: Some(*slot_id),
                        type_id: TypeId::of::<HoverLeave>(),
                        event: Box::new(HoverLeave),
                    });
                }
            }

            if let Some(e) = &events.mouse_scroll {
                self.pending_events.push(PendingEvent {
                    target: Some(*slot_id),
                    type_id: TypeId::of::<MouseScroll>(),
                    event: Box::new(*e),
                });
            }
            if events.mouse_enter {
                self.pending_events.push(PendingEvent {
                    target: Some(*slot_id),
                    type_id: TypeId::of::<MouseEnter>(),
                    event: Box::new(MouseEnter),
                });
            }
            if events.mouse_leave {
                self.pending_events.push(PendingEvent {
                    target: Some(*slot_id),
                    type_id: TypeId::of::<MouseLeave>(),
                    event: Box::new(MouseLeave),
                });
            }

            // auto focus on click
            if hit && !events.clicks.is_empty() {
                if let Some(Some(slot)) = self.slots.get(*slot_id as usize) {
                    if slot.widget.focusable() {
                        self.set_focus_by_id(*slot_id);
                    }
                }
            }
        }

        // clear focus if clicked outside all focusable widgets
        if !events.clicks.is_empty() {
            let click_hit_any = slot_ids.iter().any(|slot_id| {
                if let Some(Some(slot)) = self.slots.get(*slot_id as usize) {
                    let (x, y, w, h) = slot.widget.bounds();
                    slot.widget.focusable()
                        && self.input.mouse.x >= x
                        && self.input.mouse.x <= x + w
                        && self.input.mouse.y >= y
                        && self.input.mouse.y <= y + h
                } else {
                    false
                }
            });
            if !click_hit_any {
                self.clear_focus();
            }
        }

        // global events
        for e in &events.key_presses {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<KeyPress>(),
                event: Box::new(*e),
            });
        }
        for e in &events.key_releases {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<KeyRelease>(),
                event: Box::new(*e),
            });
        }
        if let Some(e) = &events.mouse_move {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<MouseMove>(),
                event: Box::new(*e),
            });
        }
        for e in &events.mouse_downs {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<MouseDown>(),
                event: Box::new(*e),
            });
        }
        for e in &events.mouse_ups {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<MouseUp>(),
                event: Box::new(*e),
            });
        }
        for e in &events.clicks {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<Click>(),
                event: Box::new(*e),
            });
        }
        if let Some(e) = &events.mouse_scroll {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<MouseScroll>(),
                event: Box::new(*e),
            });
        }
        if events.mouse_enter {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<MouseEnter>(),
                event: Box::new(MouseEnter),
            });
        }
        if events.mouse_leave {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<MouseLeave>(),
                event: Box::new(MouseLeave),
            });
        }
    }
}

/// Collected input events for a single frame
struct InputEvents {
    key_presses: Vec<KeyPress>,
    key_releases: Vec<KeyRelease>,
    mouse_move: Option<MouseMove>,
    mouse_downs: Vec<MouseDown>,
    mouse_ups: Vec<MouseUp>,
    clicks: Vec<Click>,
    mouse_scroll: Option<MouseScroll>,
    mouse_enter: bool,
    mouse_leave: bool,
}

impl fmt::Display for Ui {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Ui ({} slots):", self.slots.len())?;

        // print only root nodes (no parent), recursing into children
        for (i, slot) in self.slots.iter().enumerate() {
            if let Some(s) = slot {
                if s.parent.is_none() {
                    self.print_node(f, i, 0)?;
                }
            }
        }
        Ok(())
    }
}

impl Ui {
    fn print_node(&self, f: &mut fmt::Formatter<'_>, index: usize, depth: usize) -> fmt::Result {
        let indent = "  ".repeat(depth);
        if let Some(Some(s)) = self.slots.get(index) {
            writeln!(f, "{}[{}] {}", indent, index, s.widget.name())?;
            for &child_id in &s.children {
                self.print_node(f, child_id as usize, depth + 1)?;
            }
        }
        Ok(())
    }
}
