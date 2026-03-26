// per frame update pass
//   push current layout styles to taffy
//   run layout compute
//   call sync() on every widget with its computed position

use super::Ui;
use crate::widget::{Handle, HasBase};

impl Ui {
    pub fn update(&mut self) {
        let Some(root) = self.root else { return };
        let w = self.window_width as f32;
        let h = self.window_height as f32;

        // sync all layout styles to taffy
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

        // compute layout
        self.layout
            .compute(root, w, h, |_handle, _max_w, _max_h| (0.0, 0.0));

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
