mod events;
mod slot;
mod tree;
mod update;

pub use events::{
    BlurEvent, ChangeEvent, ClickEvent, DoubleClickEvent, Event, FocusEvent, HoverEndEvent,
    HoverEvent, KeyPressEvent, KeyReleaseEvent, MouseMoveEvent, PressEvent, ReleaseEvent,
    RightClickEvent, ScrollEvent,
};

use crate::layout::LayoutEngine;
use crate::widget::{Handle, Widget};
use bento_wgpu::SceneGraph;

use events::EventSystem;
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

    // raw connect 
    // TODO: remove in the future

    pub fn connect<T>(
        &mut self,
        handle: Handle<T>,
        callback: impl FnMut(&mut Ui, &mut Event) + 'static,
    ) -> u32 {
        // raw connect always goes to external
        self.events.connect_external(handle, callback)
    }

    pub fn disconnect(&mut self, connection_id: u32) {
        self.events.disconnect(connection_id);
    }

    pub fn has_connections(&self, handle: Handle<()>) -> bool {
        self.events.has_connections(handle)
    }

    // typed on_* methods 
    // when called during register() they go to internal list
    // otherwise they go to external list
    // no priority concept exposed to callers

    pub fn on_click<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut ClickEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::Click(e) = event {
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

    pub fn on_right_click<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut RightClickEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::RightClick(e) = event {
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

    pub fn on_double_click<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut DoubleClickEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::DoubleClick(e) = event {
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

    pub fn on_press<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut PressEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::Press(e) = event {
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

    pub fn on_release<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut ReleaseEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::Release(e) = event {
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

    pub fn on_hover<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut HoverEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::Hover(e) = event {
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

    pub fn on_hover_end<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut HoverEndEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::HoverEnd(e) = event {
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

    pub fn on_focus<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut FocusEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::Focus(e) = event {
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

    pub fn on_blur<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut BlurEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::Blur(e) = event {
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

    pub fn on_key_press<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut KeyPressEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::KeyPress(e) = event {
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

    pub fn on_key_release<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut KeyReleaseEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::KeyRelease(e) = event {
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

    pub fn on_change<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut ChangeEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::Change(e) = event {
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

    pub fn on_scroll<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut ScrollEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::Scroll(e) = event {
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

    pub fn on_mouse_move<W: Widget>(
        &mut self,
        handle: Handle<W>,
        mut callback: impl FnMut(&mut Ui, &mut W, &mut MouseMoveEvent) + 'static,
    ) -> u32 {
        let h = handle.untyped();
        let cb = move |ui: &mut Ui, event: &mut Event| {
            if let Event::MouseMove(e) = event {
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

    // event emission 

    pub fn emit<T>(&mut self, handle: Handle<T>, event: Event) {
        self.events.emit(handle, event);
    }

    pub fn emit_bubbling<T>(&mut self, handle: Handle<T>, event: Event) {
        let global = self.global();
        let handle = handle.untyped();
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
            for (handle, mut event) in queue {
                let Some(mut list) = self.events.connections.remove(&handle) else {
                    continue;
                };

                // drain external connections first
                for conn in &mut list.external {
                    let propagation_stopped = match &event {
                        Event::Click(e) => e.is_propagation_stopped(),
                        Event::RightClick(e) => e.is_propagation_stopped(),
                        Event::DoubleClick(e) => e.is_propagation_stopped(),
                        Event::Press(e) => e.is_propagation_stopped(),
                        Event::Release(e) => e.is_propagation_stopped(),
                        Event::MouseMove(e) => e.is_propagation_stopped(),
                        Event::Scroll(e) => e.is_propagation_stopped(),
                        Event::Hover(e) => e.is_propagation_stopped(),
                        Event::HoverEnd(e) => e.is_propagation_stopped(),
                        Event::Focus(e) => e.is_propagation_stopped(),
                        Event::Blur(e) => e.is_propagation_stopped(),
                        Event::KeyPress(e) => e.is_propagation_stopped(),
                        Event::KeyRelease(e) => e.is_propagation_stopped(),
                        Event::Change(e) => e.is_propagation_stopped(),
                        Event::Custom(_) => false,
                    };
                    if propagation_stopped {
                        break;
                    }
                    (conn.callback)(self, &mut event);
                }

                // drain internal connections only if default not stopped
                let default_stopped = match &event {
                    Event::Click(e) => e.is_default_stopped(),
                    Event::RightClick(e) => e.is_default_stopped(),
                    Event::DoubleClick(e) => e.is_default_stopped(),
                    Event::Press(e) => e.is_default_stopped(),
                    Event::Release(e) => e.is_default_stopped(),
                    Event::MouseMove(e) => e.is_default_stopped(),
                    Event::Scroll(e) => e.is_default_stopped(),
                    Event::Hover(e) => e.is_default_stopped(),
                    Event::HoverEnd(e) => e.is_default_stopped(),
                    Event::Focus(e) => e.is_default_stopped(),
                    Event::Blur(e) => e.is_default_stopped(),
                    Event::KeyPress(e) => e.is_default_stopped(),
                    Event::KeyRelease(e) => e.is_default_stopped(),
                    Event::Change(e) => e.is_default_stopped(),
                    Event::Custom(_) => false,
                };

                if !default_stopped {
                    for conn in &mut list.internal {
                        let propagation_stopped = match &event {
                            Event::Click(e) => e.is_propagation_stopped(),
                            Event::RightClick(e) => e.is_propagation_stopped(),
                            Event::DoubleClick(e) => e.is_propagation_stopped(),
                            Event::Press(e) => e.is_propagation_stopped(),
                            Event::Release(e) => e.is_propagation_stopped(),
                            Event::MouseMove(e) => e.is_propagation_stopped(),
                            Event::Scroll(e) => e.is_propagation_stopped(),
                            Event::Hover(e) => e.is_propagation_stopped(),
                            Event::HoverEnd(e) => e.is_propagation_stopped(),
                            Event::Focus(e) => e.is_propagation_stopped(),
                            Event::Blur(e) => e.is_propagation_stopped(),
                            Event::KeyPress(e) => e.is_propagation_stopped(),
                            Event::KeyRelease(e) => e.is_propagation_stopped(),
                            Event::Change(e) => e.is_propagation_stopped(),
                            Event::Custom(_) => false,
                        };
                        if propagation_stopped {
                            break;
                        }
                        (conn.callback)(self, &mut event);
                    }
                }

                // put list back, merging any new connections added during drain
                let entry = self
                    .events
                    .connections
                    .entry(handle)
                    .or_insert_with(|| events::ConnectionList::new());
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
