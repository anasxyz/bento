use super::{Ui, slot::Slot};
use crate::widget::{Handle, HasBase, Widget};

impl Ui {
    /// add a widget to the tree
    /// returns a typed handle
    pub fn add<W: Widget>(&mut self, widget: W) -> Handle<W> {
        let layout = widget.base().layout.clone();

        // reuse a free slot if one exists
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                let generation = 0;
                let h = Handle::<()>::new(i as u32, generation);
                self.layout.add(h, &layout);
                *slot = Some(Slot {
                    widget: Box::new(widget),
                    generation,
                    children: Vec::new(),
                    parent: None,
                });
                return Handle::new(i as u32, generation);
            }
        }

        let id = self.slots.len() as u32;
        let generation = 0;
        let h = Handle::<()>::new(id, generation);
        self.layout.add(h, &layout);
        self.slots.push(Some(Slot {
            widget: Box::new(widget),
            generation,
            children: Vec::new(),
            parent: None,
        }));
        Handle::new(id, generation)
    }

    /// remove a widget
    /// increments generation so stale handles become invalid
    /// also removes all connections and clears focus/hover if needed
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

        // remove from layout engine
        self.layout.remove(handle);

        // remove all connections for this handle
        self.events.connections.remove(&handle);
        self.events.event_queue.retain(|(h, _)| *h != handle);

        // clear interaction state
        if self.interaction.hovered == Some(handle) {
            self.interaction.hovered = None;
        }
        if self.interaction.pressed == Some(handle) {
            self.interaction.pressed = None;
        }
        if self.interaction.focused == Some(handle) {
            self.interaction.focused = None;
        }

        // invalidate slot by incrementing generation
        if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
            slot.generation = slot.generation.wrapping_add(1);
        }
        if let Some(slot) = self.slots.get_mut(handle.id as usize) {
            *slot = None;
        }
    }

    /// set the root widget
    /// the top of the layout tree
    pub fn set_root<W>(&mut self, handle: Handle<W>) {
        self.root = Some(handle.untyped());
    }

    pub fn root(&self) -> Option<Handle<()>> {
        self.root
    }

    /// append child to parent in both the widget tree and layout engine
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

    /// get a typed reference to a widget
    /// returns none if handle is stale
    pub fn get<W: Widget>(&self, handle: Handle<W>) -> Option<&W> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any().downcast_ref::<W>()
    }

    /// get a typed mutable reference to a widget
    pub fn get_mut<W: Widget>(&mut self, handle: Handle<W>) -> Option<&mut W> {
        let slot = self.slots.get_mut(handle.id as usize)?.as_mut()?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.widget.as_any_mut().downcast_mut::<W>()
    }

    /// get an untyped reference
    /// used internally by dispatch
    pub(crate) fn get_any(&self, handle: Handle<()>) -> Option<&dyn Widget> {
        let slot = self.slots.get(handle.id as usize)?.as_ref()?;
        if slot.generation != handle.generation {
            return None;
        }
        Some(slot.widget.as_ref())
    }

    /// temporarily remove widget from slot, call f, put it back
    /// avoids borrow conflicts when callbacks need &mut Ui
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
