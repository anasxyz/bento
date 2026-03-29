use super::Ui;
use crate::fonts::Fonts;
use crate::widget::{Handle, HasBase};
use std::collections::HashMap;

impl Ui {
    pub fn update(&mut self, fonts: &mut Fonts) {
        let Some(root) = self.root else { return };
        let w = self.window_width as f32;
        let h = self.window_height as f32;

        // sync dirty layout styles to taffy
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
        for (handle, _) in &dirty_updates {
            if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
                slot.widget.base_mut().layout_dirty = false;
            }
        }

        // pre-measure widgets that need it
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
            let natural = measured.get(&handle).copied().unwrap_or((0.0, 0.0));
            if let Some(mw) = max_w {
                if mw < natural.0 {
                    if let Some(Some(slot)) = self.slots.get(handle.id as usize) {
                        let (w, h) = slot.widget.measure(fonts, Some(mw)).unwrap_or((0.0, 0.0));
                        return (w + 1.0, h);
                    }
                }
            }
            (natural.0 + 1.0, natural.1)
        });

        // collect handles
        let handles: Vec<Handle<()>> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().map(|s| Handle::new(i as u32, s.generation)))
            .collect();

        // compute content size for widgets that have children
        for &handle in &handles {
            let children = self.children(handle).to_vec();
            if children.is_empty() {
                continue;
            }
            let own = match self.layout.get_rect(handle) {
                Some(r) => r,
                None => continue,
            };
            let mut max_right = own.0;
            let mut max_bottom = own.1;
            for child in children {
                if let Some((cx, cy, cw, ch)) = self.layout.get_rect(child) {
                    max_right = max_right.max(cx + cw);
                    max_bottom = max_bottom.max(cy + ch);
                }
            }
            if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
                slot.widget.base_mut().content_width = max_right - own.0;
                slot.widget.base_mut().content_height = max_bottom - own.1;
            }
        }

        // update cursor offset for text inputs
        for &handle in &handles {
            if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
                if let Some(input) = slot
                    .widget
                    .as_any_mut()
                    .downcast_mut::<crate::widgets::TextInput>()
                {
                    input.update_cursor_offset(fonts);
                }
            }
        }

        // sync computed positions to scene nodes
        for handle in handles {
            if let Some((x, y, w, h)) = self.layout.get_rect(handle) {
                if let Some(Some(slot)) = self.slots.get_mut(handle.id as usize) {
                    // only run sync on dirty widgets
                    if slot.widget.base().render_dirty {
                        slot.widget.sync(&mut self.scene, x, y, w, h);
                        slot.widget.base_mut().render_dirty = false; 
                    }
                }
            }
        }
    }
}
