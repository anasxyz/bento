use super::{Ui, slot::Slot};
use crate::widget::{Handle, HasBase, Widget};
use bento_wgpu::SceneNodeId;

impl Ui {
    pub fn add<W: Widget>(&mut self, mut widget: W) -> Handle<W> {
        let layout = widget.base().layout.clone();
        let has_measure = widget.has_measure();

        // call build to create scene nodes eagerly, track which were created
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

        // find which new nodes are already children of another new node
        // those are internal nodes and should NOT be attached to root
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

        // only top level nodes, that are not already someones child, attach to root
        let scene_nodes: Vec<bento_wgpu::SceneNodeId> = all_new
            .iter()
            .filter(|id| !child_nodes.contains(&id.0))
            .copied()
            .collect();

        for &node_id in &scene_nodes {
            self.scene.add_child(self.scene.root, node_id);
        }

        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                let generation = 0;
                let h = Handle::<()>::new(i as u32, generation);
                if has_measure {
                    self.layout.add_with_measure(h, &layout);
                } else {
                    self.layout.add(h, &layout);
                }
                *slot = Some(Slot {
                    widget: Box::new(widget),
                    generation,
                    children: Vec::new(),
                    parent: None,
                    scene_nodes,
                });
                return Handle::new(i as u32, generation);
            }
        }

        let id = self.slots.len() as u32;
        let generation = 0;
        let h = Handle::<()>::new(id, generation);
        if has_measure {
            self.layout.add_with_measure(h, &layout);
        } else {
            self.layout.add(h, &layout);
        }
        self.slots.push(Some(Slot {
            widget: Box::new(widget),
            generation,
            children: Vec::new(),
            parent: None,
            scene_nodes,
        }));
        Handle::new(id, generation)
    }

    pub fn remove<T>(&mut self, handle: Handle<T>) {
        let handle = handle.untyped();

        // detach from parent
        if let Some(parent) = self.parent(handle) {
            if let Some(Some(slot)) = self.slots.get_mut(parent.id as usize) {
                slot.children.retain(|c| *c != handle);
            }
            let children = self.children(parent).to_vec();
            self.layout.set_children(parent, &children);
        }

        // detach scene nodes from scene graph
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
        self.events.event_queue.retain(|(h, _)| *h != handle);

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

        // widget tree
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

        // scene graph
        // move childs nodes from root to parents attachment node
        let child_nodes: Vec<SceneNodeId> = self
            .slots
            .get(child.id as usize)
            .and_then(|s| s.as_ref())
            .map(|s| s.scene_nodes.clone())
            .unwrap_or_default();

        let attachment = self.get_attachment_node(parent);

        for node_id in child_nodes {
            // remove from scene root
            self.scene.remove_child(self.scene.root, node_id);
            // attach to parents attachment node
            self.scene.add_child(attachment, node_id);
        }
    }

    /// get the scene node that children of this widget should attach to
    /// uses children_attachment_node() if provided, otherwise the widgets first scene node,
    /// otherwise the scene root
    pub(crate) fn get_attachment_node(&self, handle: Handle<()>) -> SceneNodeId {
        if let Some(Some(slot)) = self.slots.get(handle.id as usize) {
            if let Some(node) = slot.widget.children_attachment_node() {
                return node;
            }
            // check if first scene node is a group node
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
