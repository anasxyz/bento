// ui/mod.rs
//
// Ui owns the widget tree and scene graph for one window.
// The user builds their UI here, then passes it to app.open_window().

use bento_wgpu::SceneGraph;
use std::collections::HashMap;

use crate::layout::LayoutEngine;
use crate::widget::{AnyWidget, Handle, HasBase, Widget};

const GLOBAL_ID: u32 = u32::MAX;

pub struct Slot {
    pub(crate) widget: AnyWidget,
    pub(crate) generation: u32,
    pub(crate) children: Vec<Handle<()>>,
    pub(crate) parent: Option<Handle<()>>,
}

pub struct Ui {
    pub(crate) slots: Vec<Option<Slot>>,
    pub(crate) layout: LayoutEngine,
    pub(crate) scene: SceneGraph,
    root: Option<Handle<()>>,
    pub window_width: u32,
    pub window_height: u32,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            layout: LayoutEngine::new(),
            scene: SceneGraph::new(),
            root: None,
            window_width: 0,
            window_height: 0,
        }
    }

    pub fn global(&self) -> Handle<()> {
        Handle::new(GLOBAL_ID, 0)
    }

    /// Add a widget. Returns a typed handle.
    pub fn add<W: Widget>(&mut self, widget: W) -> Handle<W> {
        let layout = widget.base().layout.clone();

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
            slot.children.push(child);
        }
        // update layout children
        let children: Vec<Handle<()>> = self
            .slots
            .get(parent.id as usize)
            .and_then(|s| s.as_ref())
            .map(|s| s.children.clone())
            .unwrap_or_default();
        self.layout.set_children(parent, &children);
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

    pub fn children(&self, handle: Handle<()>) -> &[Handle<()>] {
        self.slots
            .get(handle.id as usize)
            .and_then(|s| s.as_ref())
            .map(|s| s.children.as_slice())
            .unwrap_or(&[])
    }

    /// Run layout and sync all widgets to the scene graph.
    pub fn update(&mut self) {
        let Some(root) = self.root else { return };
        let w = self.window_width as f32;
        let h = self.window_height as f32;

        // push current layout styles into taffy before computing
        let handles: Vec<(Handle<()>, crate::layout::Layout)> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                s.as_ref().map(|s| {
                    (
                        Handle::new(i as u32, s.generation),
                        s.widget.base().layout.clone(),
                    )
                })
            })
            .collect();
        for (handle, layout) in &handles {
            self.layout.set_layout(*handle, layout);
        }

        // compute layout
        self.layout
            .compute(root, w, h, |_handle, _max_w, _max_h| (0.0, 0.0));

        // sync each widget's computed rect to its scene nodes
        let handles: Vec<Handle<()>> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().map(|s| Handle::new(i as u32, s.generation)))
            .collect();

        for handle in handles {
            if let Some((x, y, w, h)) = self.layout.get_rect(handle) {
                // temporarily take the widget out to avoid borrow conflict
                if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
                    slot.widget.sync(&mut self.scene, x, y, w, h);
                }
            }
        }
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}
