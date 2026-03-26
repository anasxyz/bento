use super::Ui;
use crate::fonts::Fonts;
use crate::widget::{Handle, HasBase};
use std::collections::HashMap;

impl Ui {
    pub fn update(&mut self, fonts: &mut Fonts) {
        let Some(root) = self.root else { return };
        let w = self.window_width as f32;
        let h = self.window_height as f32;

        // only sync layout styles for dirty widgets
        let dirty_updates: Vec<(Handle<()>, crate::layout::Layout)> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                s.as_ref().and_then(|s| {
                    if s.widget.base().layout_dirty {
                        Some((
                            Handle::new(i as u32, s.generation),
                            s.widget.base().layout.clone(),
                        ))
                    } else {
                        None
                    }
                })
            })
            .collect();

        for (handle, layout) in &dirty_updates {
            self.layout.set_layout(*handle, layout);
            self.layout.mark_dirty(*handle);
        }

        // clear dirty flags after syncing
        for (handle, _) in &dirty_updates {
            if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
                slot.widget.base_mut().layout_dirty = false;
            }
        }

        // pre measure all widgets that need it
        let mut measured: HashMap<Handle<()>, (f32, f32)> = HashMap::new();
        for (i, slot) in self.slots.iter().enumerate() {
            let Some(slot) = slot.as_ref() else { continue };
            if !slot.widget.has_measure() {
                continue;
            }
            let handle = Handle::new(i as u32, slot.generation);
            let size = slot.widget.measure(fonts, None).unwrap_or((0.0, 0.0));
            measured.insert(handle, size);
        }

        // compute layout
        self.layout.compute(root, w, h, |handle, max_w, _max_h| {
            if let Some(mw) = max_w {
                if measured.contains_key(&handle) {
                    if let Some(Some(slot)) = self.slots.get(handle.id as usize) {
                        return slot.widget.measure(fonts, Some(mw)).unwrap_or((0.0, 0.0));
                    }
                }
            }
            measured.get(&handle).copied().unwrap_or((0.0, 0.0))
        });

        // sync computed rects to scene graph nodes
        let handles: Vec<Handle<()>> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().map(|s| Handle::new(i as u32, s.generation)))
            .collect();

        for handle in handles {
            if let Some((x, y, w, h)) = self.layout.get_rect(handle) {
                if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
                    slot.widget.sync(&mut self.scene, x, y, w, h);
                }
            }
        }
    }
}
