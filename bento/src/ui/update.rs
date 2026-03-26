use super::Ui;
use crate::fonts::Fonts;
use crate::widget::{Handle, HasBase};

impl Ui {
    pub fn update(&mut self, fonts: &mut Fonts) {
        let Some(root) = self.root else { return };
        let w = self.window_width as f32;
        let h = self.window_height as f32;

        // 1. sync layout styles to taffy
        let style_updates: Vec<(Handle<()>, crate::layout::Layout)> = self
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
        for (handle, layout) in &style_updates {
            self.layout.set_layout(*handle, layout);
        }

        // 2. compute layout — call measure() on widgets that need it
        let measure_handles: Vec<Handle<()>> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                s.as_ref().and_then(|s| {
                    if s.widget.has_measure() {
                        Some(Handle::new(i as u32, s.generation))
                    } else {
                        None
                    }
                })
            })
            .collect();

        self.layout.compute(root, w, h, |handle, max_w, _max_h| {
            if !measure_handles.contains(&handle) {
                return (0.0, 0.0);
            }
            if let Some(Some(slot)) = self.slots.get(handle.id as usize) {
                return slot.widget.measure(fonts, max_w).unwrap_or((0.0, 0.0));
            }
            (0.0, 0.0)
        });

        // 3. sync computed rects to scene graph nodes
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
