// src/scene.rs

/// A drawing command that can be stored and replayed
#[derive(Clone, Debug)]
pub enum DrawCommand {
    Rect { x: f32, y: f32, w: f32, h: f32, color: [f32; 4] },
    Circle { cx: f32, cy: f32, radius: f32, color: [f32; 4] },
    RoundedRect { x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4] },
    Text { text: String, x: f32, y: f32 },
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
            y 
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
}
