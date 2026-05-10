use crate::layout::run_layout;
use crate::layout::tree::LayoutTree;
use crate::widget::Group;
use crate::widget::{Widget, WidgetHandle};
use bento_shared::{Scene, SceneNodeId, TextMeasurer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Slot {
    pub widget: Box<dyn Widget>,
    pub generation: u32,
    // scene nodes owned by this widget, removed when the widget is removed
    pub node_ids: Vec<SceneNodeId>,
     // index into Ui::layout_tree
    pub layout_node: usize,
}

struct EventQueue {
    shared_sender: Arc<Mutex<Option<Arc<dyn Fn(u64) + Send + Sync>>>>,
    callbacks: HashMap<u64, Box<dyn FnOnce(&mut Ui)>>,
    async_callbacks: Arc<Mutex<HashMap<u64, Box<dyn FnOnce(&mut Ui) + Send>>>>,
    next_id: u64,
    pending_futures: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>>,
    spawner: Option<
        Arc<dyn Fn(std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>) + Send + Sync>,
    >,
}

impl EventQueue {
    fn new() -> Self {
        Self {
            shared_sender: Arc::new(Mutex::new(None)),
            callbacks: HashMap::new(),
            async_callbacks: Arc::new(Mutex::new(HashMap::new())),
            next_id: 0,
            pending_futures: Vec::new(),
            spawner: None,
        }
    }
}

pub struct Ui {
    pub scene: Scene,
    pub layout_tree: LayoutTree,
    slots: Vec<Option<Slot>>,
    events: EventQueue,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            slots: Vec::new(),
            events: EventQueue::new(),
            layout_tree: LayoutTree::new(),
        }
    }

    pub fn add<W: Widget + 'static>(&mut self, mut widget: W) -> WidgetHandle<W> {
        // track which scene nodes this widget adds during build
        // so we can remove them later if the widget is removed
        self.scene.start_tracking();
        widget.build(&mut self.scene);
        let node_ids = self.scene.stop_tracking();
        let generation = 0;

        let slot_index = self
            .slots
            .iter()
            .position(|s| s.is_none())
            .unwrap_or(self.slots.len());

        let layout_node = self.layout_tree.add(slot_index, None);

        let slot = Slot {
            widget: Box::new(widget),
            generation,
            node_ids,
            layout_node,
        };

        if slot_index == self.slots.len() {
            self.slots.push(Some(slot));
        } else {
            self.slots[slot_index] = Some(slot);
        }

        WidgetHandle::new(slot_index as u32, generation)
    }

    pub fn add_to<W: Widget + 'static, P: Widget + 'static>(
        &mut self,
        parent: WidgetHandle<P>,
        mut widget: W,
    ) -> WidgetHandle<W> {
        // build widget scene nodes
        self.scene.start_tracking();
        widget.build(&mut self.scene);
        let node_ids = self.scene.stop_tracking();
        let generation = 0;

        // get parent layout node and parent scene node id
        let parent_slot = self.slots[parent.id as usize].as_ref().unwrap();
        let parent_layout_node = parent_slot.layout_node;

        // find the parent's group scene node id
        // the parent widget gotta be a Group, get its scene_id via downcast
        let parent_scene_id = parent_slot
            .widget
            .as_any()
            .downcast_ref::<Group>()
            .and_then(|g| g.id);

        // reparent all child scene nodes under the group scene node
        if let Some(group_scene_id) = parent_scene_id {
            for &nid in &node_ids {
                self.scene.reparent(nid, group_scene_id);
            }
        }

        // register slot
        let slot_index = self
            .slots
            .iter()
            .position(|s| s.is_none())
            .unwrap_or(self.slots.len());

        // register layout node as child of parent
        let layout_node = self.layout_tree.add(slot_index, Some(parent_layout_node));

        let slot = Slot {
            widget: Box::new(widget),
            generation,
            node_ids,
            layout_node,
        };

        if slot_index == self.slots.len() {
            self.slots.push(Some(slot));
        } else {
            self.slots[slot_index] = Some(slot);
        }

        WidgetHandle::new(slot_index as u32, generation)
    }

    pub fn remove<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) {
        let slot = match self.slots.get_mut(handle.id as usize) {
            Some(s @ Some(_)) => s,
            _ => return,
        };

        let s = slot.as_ref().unwrap();
        if s.generation != handle.generation {
            return;
        }

        for id in &s.node_ids {
            self.scene.remove(*id);
        }

        self.layout_tree.remove(s.layout_node);
        *slot = None;
    }

    pub fn get<W: Widget + 'static>(&self, handle: WidgetHandle<W>) -> Option<&W> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any().downcast_ref::<W>()
    }

    pub fn get_mut<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>) -> Option<&mut W> {
        let slot = self.slots.get_mut(handle.id as usize)?.as_mut()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any_mut().downcast_mut::<W>()
    }

    pub fn with<W: Widget + 'static>(&mut self, handle: WidgetHandle<W>, f: impl FnOnce(&mut W)) {
        if let Some(widget) = self.get_mut(handle) {
            f(widget);
        }
    }

    pub fn update(&mut self, measurer: &mut dyn TextMeasurer, delta: f32) {
        // mark layout dirty for any widget with dirty_layout
        for (slot_index, slot) in self.slots.iter().enumerate() {
            if let Some(s) = slot {
                if s.widget.base().dirty_layout {
                    self.layout_tree
                        .mark_dirty(self.slots[slot_index].as_ref().unwrap().layout_node);
                }
            }
        }

        // run layout
        if self.layout_tree.any_dirty() {
            let start = std::time::Instant::now();
            run_layout(&mut self.layout_tree, &mut self.slots, measurer);
            println!("layout took {:?}", start.elapsed());

            // write resolved positions into scene graph
            for node in &self.layout_tree.nodes {
                if node.slot == usize::MAX {
                    continue;
                }
                let Some(Some(slot)) = self.slots.get(node.slot) else {
                    continue;
                };
                for &scene_id in &slot.node_ids {
                    if let Some(scene_node) = self.scene.get_mut(scene_id) {
                        scene_node.set_position(
                            node.layout.x,
                            node.layout.y,
                            node.layout.w,
                            node.layout.h,
                        );
                    }
                }
            }
        }

        // update dirty widgets
        // this is visual only
        for slot in self.slots.iter_mut().flatten() {
            if slot.widget.base().dirty {
                slot.widget.base_mut().delta = delta;
                slot.widget.base_mut().dirty = false;
                slot.widget.base_mut().dirty_layout = false;
                slot.widget.pre_update();
                slot.widget.update(&mut self.scene, measurer);
            }
        }
    }

    pub fn set_viewport(&mut self, w: f32, h: f32) {
        for node in &mut self.layout_tree.nodes {
            if node.parent.is_none() && node.slot != usize::MAX {
                node.dirty = true;
                if let Some(Some(slot)) = self.slots.get_mut(node.slot) {
                    slot.widget.base_mut().layout.w = w;
                    slot.widget.base_mut().layout.h = h;
                    slot.widget.base_mut().dirty = true;
                    slot.widget.base_mut().dirty_layout = true;
                }
            }
        }
    }

    pub fn any_dirty(&self) -> bool {
        self.slots.iter().flatten().any(|s| s.widget.base().dirty)
    }

    pub fn set_sender(&mut self, sender: Arc<dyn Fn(u64) + Send + Sync>) {
        *self.events.shared_sender.lock().unwrap() = Some(sender);
    }

    pub fn set_spawner(
        &mut self,
        spawner: Arc<
            dyn Fn(std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>) + Send + Sync,
        >,
    ) {
        self.events.spawner = Some(spawner.clone());
        for fut in self.events.pending_futures.drain(..) {
            spawner(fut);
        }
    }

    pub fn timer(&mut self, duration: f32, callback: impl FnOnce(&mut Ui) + Send + 'static) {
        self.spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs_f32(duration)).await;
            callback
        });
    }

    pub fn spawn<F, C>(&mut self, future: F)
    where
        F: std::future::Future<Output = C> + Send + 'static,
        C: FnOnce(&mut Ui) + Send + 'static,
    {
        let id = self.events.next_id;
        self.events.next_id += 1;

        let async_callbacks = self.events.async_callbacks.clone();
        let shared_sender = self.events.shared_sender.clone();

        let fut = Box::pin(async move {
            let callback = future.await;
            async_callbacks
                .lock()
                .unwrap()
                .insert(id, Box::new(callback));
            if let Some(sender) = shared_sender.lock().unwrap().as_ref() {
                sender(id);
            }
        });

        if let Some(spawner) = &self.events.spawner {
            spawner(fut);
        } else {
            self.events.pending_futures.push(fut);
        }
    }

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

    pub fn set_sender_from_bento(&mut self, sender: Arc<dyn Fn(u64) + Send + Sync>) {
        self.set_sender(sender);
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}
