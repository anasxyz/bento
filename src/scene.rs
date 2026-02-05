// src/scene.rs

/// A drawing command that can be stored and replayed
#[derive(Clone, Debug)]
pub enum DrawCommand {
    Rect { x: f32, y: f32, w: f32, h: f32, color: [f32; 4] },
    Circle { cx: f32, cy: f32, radius: f32, color: [f32; 4] },
    RoundedRect { x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4] },
    Text { text: String, x: f32, y: f32, font_size: f32 },
}

/// Simple scene graph - just a list of drawing commands
pub struct Scene {
    commands: Vec<DrawCommand>,
    dirty: bool,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            dirty: true,
        }
    }

    /// Add a rectangle to the scene
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        self.commands.push(DrawCommand::Rect { x, y, w, h, color });
        self.dirty = true;
    }

    /// Add a circle to the scene
    pub fn circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4]) {
        self.commands.push(DrawCommand::Circle { cx, cy, radius, color });
        self.dirty = true;
    }

    /// Add a rounded rectangle to the scene
    pub fn rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4]) {
        self.commands.push(DrawCommand::RoundedRect { x, y, w, h, radius, color });
        self.dirty = true;
    }

    /// Add text to the scene
    pub fn text(&mut self, text: impl Into<String>, x: f32, y: f32) {
        self.commands.push(DrawCommand::Text { 
            text: text.into(), 
            x, 
            y,
            font_size: 22.0, // Default font size
        });
        self.dirty = true;
    }

    /// Add text with custom font size to the scene
    pub fn text_sized(&mut self, text: impl Into<String>, x: f32, y: f32, font_size: f32) {
        self.commands.push(DrawCommand::Text { 
            text: text.into(), 
            x, 
            y,
            font_size,
        });
        self.dirty = true;
    }

    /// Clear all commands
    pub fn clear(&mut self) {
        self.commands.clear();
        self.dirty = true;
    }

    /// Get all commands (for rendering)
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    /// Check if scene needs re-rendering
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Mark scene as clean (called after rendering)
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Force a re-render on next frame
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Draw a button with centered text
    pub fn button(&mut self, x: f32, y: f32, w: f32, h: f32, text: &str) {
        // Draw rounded rectangle background
        self.rounded_rect(x, y, w, h, 8.0, [0.2, 0.4, 0.8, 1.0]);
        
        // Calculate font size that fits
        let base_font_size = 22.0;
        let estimated_text_width = text.len() as f32 * (base_font_size * 0.5); // Rough estimate
        let available_width = w - 20.0; // Padding
        let available_height = h - 10.0; // Padding
        
        // Scale down font if text is too wide or tall
        let scale_for_width = if estimated_text_width > available_width {
            available_width / estimated_text_width
        } else {
            1.0
        };
        
        let scale_for_height = if base_font_size > available_height {
            available_height / base_font_size
        } else {
            1.0
        };
        
        let font_size = base_font_size * scale_for_width.min(scale_for_height);
        
        // Recalculate text width with scaled font
        let text_width = text.len() as f32 * (font_size * 0.5);
        
        // Center text
        let text_x = x + (w - text_width) / 2.0;
        let text_y = y + h / 2.0 - font_size * 0.15;
        
        self.text_sized(text, text_x, text_y, font_size);
    }
}
