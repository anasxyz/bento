use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;

use bento_shared::{Scene, SceneNodeId};

use crate::events::types::{
    Click, HoverEnter, HoverLeave, KeyPress, KeyRelease, MouseDown, MouseEnter, MouseLeave,
    MouseMove, MouseScroll, MouseUp,
};
use crate::input::InputState;
use crate::input::mouse::MouseButton;
use crate::ui::asyncs::AsyncEventQueue;
use crate::widget::{AnyWidget, Widget, WidgetHandle};

pub struct Slot {
    pub widget: Box<dyn AnyWidget>,
    pub generation: u32,
}

pub struct Ui {
    pub scene: Scene,
    pub input: InputState,
    pub asyncs: AsyncEventQueue,
    pub needs_redraw: bool,
    slots: Vec<Option<Slot>>,

    listeners: HashMap<Option<SceneNodeId>, Vec<Listener>>,
    next_listener_id: u64,
    pending_events: Vec<PendingEvent>,
    pending_removals: Vec<u64>,

    focused: Option<SceneNodeId>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            input: InputState::new(),
            asyncs: AsyncEventQueue::new(),
            needs_redraw: false,
            slots: Vec::new(),
            listeners: HashMap::new(),
            next_listener_id: 0,
            pending_events: Vec::new(),
            pending_removals: Vec::new(),
            focused: None,
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    pub fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    pub fn add<W: AnyWidget + 'static>(&mut self, widget: W) -> WidgetHandle<W> {
        let index = self.slots.len();
        let handle = WidgetHandle::new(index as u32, 0);
        self.slots.push(Some(Slot {
            widget: Box::new(widget),
            generation: 0,
        }));
        let mut slot = self.slots[index].take().unwrap();
        slot.widget.build(self, handle.untyped());
        self.slots[index] = Some(slot);
        self.request_redraw();
        handle
    }

    pub fn get<W: AnyWidget + 'static>(&self, handle: WidgetHandle<W>) -> Option<&W> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any().downcast_ref::<W>()
    }

    pub fn get_mut<W: AnyWidget + 'static>(&mut self, handle: WidgetHandle<W>) -> Option<&mut W> {
        let slot = self.slots.get_mut(handle.id as usize)?.as_mut()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any_mut().downcast_mut::<W>()
    }

    pub fn remove<W: AnyWidget + 'static>(&mut self, handle: WidgetHandle<W>) {
        let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) else {
            return;
        };
        if slot.generation != handle.generation {
            return;
        }
        let mut slot = self.slots[handle.id as usize].take().unwrap();
        slot.widget.remove(self);
        self.request_redraw();
    }

    pub fn append<P: AnyWidget + 'static, C: AnyWidget + 'static>(
        &mut self,
        parent: WidgetHandle<P>,
        child: WidgetHandle<C>,
    ) {
        let parent_id = self.get(parent).unwrap().id().unwrap();
        let child_id = self.get(child).unwrap().id().unwrap();
        self.scene_mut().append(parent_id, child_id);
    }

    pub fn update(&mut self) {
        for i in 0..self.slots.len() {
            if let Some(slot) = &self.slots[i] {
                if slot.widget.is_dirty() {
                    let mut slot = self.slots[i].take().unwrap();
                    slot.widget.update(self);
                    self.slots[i] = Some(slot);
                }
            }
        }
    }
}

struct Listener {
    id: u64,
    type_id: TypeId,
    f: Box<dyn FnMut(&dyn Any, &mut Ui) -> bool>,
}

struct PendingEvent {
    target: Option<SceneNodeId>,
    type_id: TypeId,
    event: Box<dyn Any>,
}

#[derive(Clone, Copy)]
pub struct ListenerHandle {
    target: Option<SceneNodeId>,
    id: u64,
}

pub trait IntoListenTarget {
    fn into_target(self, ui: &Ui) -> SceneNodeId;
}

impl<W: AnyWidget + 'static> IntoListenTarget for WidgetHandle<W> {
    fn into_target(self, ui: &Ui) -> SceneNodeId {
        ui.get(self).unwrap().id().unwrap()
    }
}

impl IntoListenTarget for SceneNodeId {
    fn into_target(self, _ui: &Ui) -> SceneNodeId {
        self
    }
}

impl Ui {
    fn register(
        &mut self,
        target: Option<SceneNodeId>,
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

    pub fn listen<T: IntoListenTarget, E: 'static>(
        &mut self,
        target: T,
        mut f: impl FnMut(&E, &mut Ui) + 'static,
    ) -> ListenerHandle {
        let scene_id = target.into_target(self);
        self.register(
            Some(scene_id),
            TypeId::of::<E>(),
            Box::new(move |event, ui| {
                if let Some(e) = event.downcast_ref::<E>() {
                    f(e, ui);
                }
                true
            }),
        )
    }

    pub fn listen_once<W: AnyWidget + 'static, E: 'static>(
        &mut self,
        handle: WidgetHandle<W>,
        mut f: impl FnMut(&E, &mut Ui) + 'static,
    ) -> ListenerHandle {
        let scene_id = self.get(handle).unwrap().id().unwrap();
        self.register(
            Some(scene_id),
            TypeId::of::<E>(),
            Box::new(move |event, ui| {
                if let Some(e) = event.downcast_ref::<E>() {
                    f(e, ui);
                }
                false
            }),
        )
    }

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

    pub fn unlisten(&mut self, handle: ListenerHandle) {
        self.pending_removals.push(handle.id);
    }

    pub fn send<E: Any>(&mut self, target: SceneNodeId, event: E) {
        self.pending_events.push(PendingEvent {
            target: Some(target),
            type_id: TypeId::of::<E>(),
            event: Box::new(event),
        });
    }

    pub fn send_global<E: Any>(&mut self, event: E) {
        self.pending_events.push(PendingEvent {
            target: None,
            type_id: TypeId::of::<E>(),
            event: Box::new(event),
        });
    }

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
        remaining.retain(|l| !self.pending_removals.contains(&l.id));
        self.pending_removals.clear();
        self.listeners
            .entry(pending.target)
            .or_default()
            .extend(remaining);
    }

    pub fn flush(&mut self) {
        let mut i = 0;
        while i < self.pending_events.len() {
            let pending = self.pending_events.remove(i);
            self.dispatch_one(pending);
        }
    }

    pub fn process_input(&mut self) {
        // collect input snapshot first to avoid borrow issues
        let mx = self.input.mouse.x;
        let my = self.input.mouse.y;
        let mdx = self.input.mouse.dx;
        let mdy = self.input.mouse.dy;
        let msx = self.input.mouse.scroll_x;
        let msy = self.input.mouse.scroll_y;
        let left = (
            self.input.mouse.left.just_pressed,
            self.input.mouse.left.just_released,
        );
        let right = (
            self.input.mouse.right.just_pressed,
            self.input.mouse.right.just_released,
        );
        let middle = (
            self.input.mouse.middle.just_pressed,
            self.input.mouse.middle.just_released,
        );
        let just_entered = self.input.mouse.just_entered;
        let just_left = self.input.mouse.just_left;

        let pressed = [
            (MouseButton::Left, left),
            (MouseButton::Right, right),
            (MouseButton::Middle, middle),
        ];

        // hit test every listener target
        let targets: Vec<SceneNodeId> = self.listeners.keys().filter_map(|k| *k).collect();

        for id in targets {
            let (sx, sy, sw, sh) = self.scene.hitbox(id);
            let hit = mx >= sx && mx <= sx + sw && my >= sy && my <= sy + sh;

            if hit {
                // hover enter
                if let Some(slot) = self
                    .slots
                    .iter()
                    .flatten()
                    .find(|s| s.widget.id() == Some(id))
                {
                    // handled below via events
                }

                if mdx != 0.0 || mdy != 0.0 {
                    self.pending_events.push(PendingEvent {
                        target: Some(id),
                        type_id: TypeId::of::<MouseMove>(),
                        event: Box::new(MouseMove {
                            x: mx,
                            y: my,
                            dx: mdx,
                            dy: mdy,
                        }),
                    });
                }
                for (btn, (just_pressed, just_released)) in pressed {
                    if just_pressed {
                        self.pending_events.push(PendingEvent {
                            target: Some(id),
                            type_id: TypeId::of::<MouseDown>(),
                            event: Box::new(MouseDown {
                                x: mx,
                                y: my,
                                button: btn,
                            }),
                        });
                    }
                    if just_released {
                        self.pending_events.push(PendingEvent {
                            target: Some(id),
                            type_id: TypeId::of::<MouseUp>(),
                            event: Box::new(MouseUp {
                                x: mx,
                                y: my,
                                button: btn,
                            }),
                        });
                    }
                    if just_released {
                        self.pending_events.push(PendingEvent {
                            target: Some(id),
                            type_id: TypeId::of::<Click>(),
                            event: Box::new(Click {
                                x: mx,
                                y: my,
                                button: btn,
                            }),
                        });
                    }
                }
                if msx != 0.0 || msy != 0.0 {
                    self.pending_events.push(PendingEvent {
                        target: Some(id),
                        type_id: TypeId::of::<MouseScroll>(),
                        event: Box::new(MouseScroll { x: msx, y: msy }),
                    });
                }
            }
        }

        // global mouse events
        if mdx != 0.0 || mdy != 0.0 {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<MouseMove>(),
                event: Box::new(MouseMove {
                    x: mx,
                    y: my,
                    dx: mdx,
                    dy: mdy,
                }),
            });
        }
        for (btn, (just_pressed, just_released)) in pressed {
            if just_pressed {
                self.pending_events.push(PendingEvent {
                    target: None,
                    type_id: TypeId::of::<MouseDown>(),
                    event: Box::new(MouseDown {
                        x: mx,
                        y: my,
                        button: btn,
                    }),
                });
            }
            if just_released {
                self.pending_events.push(PendingEvent {
                    target: None,
                    type_id: TypeId::of::<MouseUp>(),
                    event: Box::new(MouseUp {
                        x: mx,
                        y: my,
                        button: btn,
                    }),
                });
            }
            if just_released {
                self.pending_events.push(PendingEvent {
                    target: None,
                    type_id: TypeId::of::<Click>(),
                    event: Box::new(Click {
                        x: mx,
                        y: my,
                        button: btn,
                    }),
                });
            }
        }
        if msx != 0.0 || msy != 0.0 {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<MouseScroll>(),
                event: Box::new(MouseScroll { x: msx, y: msy }),
            });
        }
        if just_entered {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<MouseEnter>(),
                event: Box::new(MouseEnter),
            });
        }
        if just_left {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<MouseLeave>(),
                event: Box::new(MouseLeave),
            });
        }

        // keyboard -> focused widget
        let pressed_keys: Vec<KeyPress> = self
            .input
            .keyboard
            .just_pressed()
            .iter()
            .map(|(k, ch)| KeyPress { key: *k, ch: *ch })
            .collect();
        let released_keys: Vec<KeyRelease> = self
            .input
            .keyboard
            .just_released()
            .iter()
            .map(|k| KeyRelease { key: *k })
            .collect();

        if let Some(focused) = self.focused {
            for e in &pressed_keys {
                self.pending_events.push(PendingEvent {
                    target: Some(focused),
                    type_id: TypeId::of::<KeyPress>(),
                    event: Box::new(*e),
                });
            }
            for e in &released_keys {
                self.pending_events.push(PendingEvent {
                    target: Some(focused),
                    type_id: TypeId::of::<KeyRelease>(),
                    event: Box::new(*e),
                });
            }
        }
        for e in &pressed_keys {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<KeyPress>(),
                event: Box::new(*e),
            });
        }
        for e in &released_keys {
            self.pending_events.push(PendingEvent {
                target: None,
                type_id: TypeId::of::<KeyRelease>(),
                event: Box::new(*e),
            });
        }

        self.flush();
    }
}
