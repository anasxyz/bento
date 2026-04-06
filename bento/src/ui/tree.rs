use super::{Ui, slot::Slot};
use crate::widget::{Handle, HasBase, Widget};
use bento_wgpu::SceneNodeId;

impl Ui {
    pub fn add<W: Widget>(&mut self, mut widget: W) -> Handle<W> {
        let layout = widget.base().layout.clone();
        let has_measure = widget.has_measure();

        // build scene nodes
        let before: std::collections::HashSet<usize> =
            self.scene.nodes.iter().map(|(i, _)| i).collect();
        widget.build(&mut self.scene);
        let all_new: Vec<bento_wgpu::SceneNodeId> = self
            .scene
            .nodes
            .iter()
            .map(|(i, _)| bento_wgpu::SceneNodeId(i))
            .filter(|id| !before.contains(&id.0))
            .collect();

        let mut child_nodes: std::collections::HashSet<usize> = std::collections::HashSet::new();
        for &node_id in &all_new {
            match &self.scene.nodes[node_id.0] {
                bento_wgpu::SceneNode::Transform(n) => {
                    for c in &n.children {
                        child_nodes.insert(c.0);
                    }
                }
                bento_wgpu::SceneNode::Clip(n) => {
                    for c in &n.children {
                        child_nodes.insert(c.0);
                    }
                }
                bento_wgpu::SceneNode::Opacity(n) => {
                    for c in &n.children {
                        child_nodes.insert(c.0);
                    }
                }
                _ => {}
            }
        }

        let scene_nodes: Vec<bento_wgpu::SceneNodeId> = all_new
            .iter()
            .filter(|id| !child_nodes.contains(&id.0))
            .copied()
            .collect();

        for &node_id in &scene_nodes {
            self.scene.add_child(self.scene.root, node_id);
        }

        // find a free slot or push a new one
        let (id, generation) = self.alloc_slot();
        let h = Handle::<()>::new(id, generation);

        if has_measure {
            self.layout.add_with_measure(h, &layout);
        } else {
            self.layout.add(h, &layout);
        }

        // insert into slot
        let slot_ref = if (id as usize) < self.slots.len() {
            &mut self.slots[id as usize]
        } else {
            self.slots.push(None);
            self.slots.last_mut().unwrap()
        };
        *slot_ref = Some(Slot {
            widget: Box::new(widget),
            generation,
            children: Vec::new(),
            parent: None,
            scene_nodes,
        });

        // register internal connections now that slot exists
        let typed_handle = Handle::<W>::new(id, generation);
        {
            let Some(slot) = self.slots.get_mut(id as usize) else {
                return typed_handle;
            };
            let Some(mut s) = slot.take() else {
                return typed_handle;
            };
            self.registering = true;
            s.widget.base_mut().handle = h;
            s.widget.register(self);
            self.registering = false;
            if let Some(slot) = self.slots.get_mut(id as usize) {
                *slot = Some(s);
            }
        }

        typed_handle
    }

    /// find a free slot index and return (id, generation)
    fn alloc_slot(&self) -> (u32, u32) {
        for (i, slot) in self.slots.iter().enumerate() {
            if slot.is_none() {
                return (i as u32, 0);
            }
        }
        (self.slots.len() as u32, 0)
    }

    pub fn add_to<W: Widget, P>(&mut self, parent: Handle<P>, widget: W) -> Handle<W> {
        let handle = self.add(widget);
        self.append(parent, handle);
        handle
    }

    pub fn remove<T>(&mut self, handle: Handle<T>) {
        let handle = handle.untyped();

        if let Some(parent) = self.parent(handle) {
            if let Some(Some(slot)) = self.slots.get_mut(parent.id as usize) {
                slot.children.retain(|c| *c != handle);
            }
            let children = self.children(parent).to_vec();
            self.layout.set_children(parent, &children);
        }

        if let Some(Some(slot)) = self.slots.get(handle.id as usize) {
            let nodes = slot.scene_nodes.clone();
            let parent_scene_node = self.get_attachment_node(handle);
            for node_id in &nodes {
                self.scene.remove_child(parent_scene_node, *node_id);
                self.scene.remove_node(*node_id);
            }
        }

        self.layout.remove(handle);
        self.events.connections.remove(&handle);
        self.events.event_queue.retain(|q| q.handle != handle);

        if self.interaction.hovered == Some(handle) {
            self.interaction.hovered = None;
        }
        if self.interaction.pressed == Some(handle) {
            self.interaction.pressed = None;
        }
        if self.interaction.focused == Some(handle) {
            self.interaction.focused = None;
        }

        if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
            slot.generation = slot.generation.wrapping_add(1);
        }
        if let Some(slot) = self.slots.get_mut(handle.id as usize) {
            *slot = None;
        }
    }

    pub fn set_root<W>(&mut self, handle: Handle<W>) {
        self.root = Some(handle.untyped());
    }

    pub fn root(&self) -> Option<Handle<()>> {
        self.root
    }

    pub fn append<P, C>(&mut self, parent: Handle<P>, child: Handle<C>) {
        let parent = parent.untyped();
        let child = child.untyped();

        if let Some(Some(slot)) = self.slots.get_mut(child.id as usize) {
            slot.parent = Some(parent);
        }
        if let Some(Some(slot)) = self.slots.get_mut(parent.id as usize) {
            if !slot.children.contains(&child) {
                slot.children.push(child);
            }
        }
        let children = self.children(parent).to_vec();
        self.layout.set_children(parent, &children);

        let child_nodes: Vec<SceneNodeId> = self
            .slots
            .get(child.id as usize)
            .and_then(|s| s.as_ref())
            .map(|s| s.scene_nodes.clone())
            .unwrap_or_default();

        let attachment = self.get_attachment_node(parent);

        for node_id in child_nodes {
            self.scene.remove_child(self.scene.root, node_id);
            self.scene.add_child(attachment, node_id);
        }
    }

    pub(crate) fn get_attachment_node(&self, handle: Handle<()>) -> SceneNodeId {
        if let Some(Some(slot)) = self.slots.get(handle.id as usize) {
            if let Some(node) = slot.widget.children_attachment_node() {
                return node;
            }
            if let Some(&first) = slot.scene_nodes.first() {
                match &self.scene.nodes[first.0] {
                    bento_wgpu::SceneNode::Transform(_)
                    | bento_wgpu::SceneNode::Clip(_)
                    | bento_wgpu::SceneNode::Opacity(_) => return first,
                    _ => {}
                }
            }
        }
        self.scene.root
    }

    pub fn parent(&self, handle: Handle<()>) -> Option<Handle<()>> {
        self.slots.get(handle.id as usize)?.as_ref()?.parent
    }

    pub fn children(&self, handle: Handle<()>) -> &[Handle<()>] {
        self.slots
            .get(handle.id as usize)
            .and_then(|s| s.as_ref())
            .map(|s| s.children.as_slice())
            .unwrap_or(&[])
    }

    pub fn get<W: Widget>(&self, handle: Handle<W>) -> Option<&W> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any().downcast_ref::<W>()
    }

    pub fn get_mut<W: Widget>(&mut self, handle: Handle<W>) -> Option<&mut W> {
        let slot = self.slots.get_mut(handle.id as usize)?.as_mut()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any_mut().downcast_mut::<W>()
    }

    pub(crate) fn get_any(&self, handle: Handle<()>) -> Option<&dyn Widget> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        Some(slot.widget.as_ref())
    }

    pub(crate) fn with_widget<F>(&mut self, handle: Handle<()>, f: F)
    where
        F: FnOnce(&mut dyn Widget, &mut Ui),
    {
        let Some(slot) = self.slots.get_mut(handle.id as usize) else {
            return;
        };
        let Some(mut s) = slot.take() else { return };
        f(s.widget.as_mut(), self);
        if let Some(slot) = self.slots.get_mut(handle.id as usize) {
            *slot = Some(s);
        }
    }
}
