// src/scene.rs

/// Represents a single drawing operation that can be stored and replayed.
/// 
/// Each variant contains all the data needed to execute that drawing command.
#[derive(Clone, Debug)]
pub enum DrawCommand {
    /// Draw a filled rectangle
    Rect { 
        x: f32, 
        y: f32, 
        w: f32, 
        h: f32, 
        color: [f32; 4] 
    },
    
    /// Draw a filled circle
    Circle { 
        cx: f32, 
        cy: f32, 
        radius: f32, 
        color: [f32; 4] 
    },
    
    /// Draw a rounded rectangle (useful for buttons, cards, etc.)
    RoundedRect { 
        x: f32, 
        y: f32, 
        w: f32, 
        h: f32, 
        radius: f32, 
        color: [f32; 4] 
    },
    
    /// Draw text at a position
    Text { 
        text: String, 
        x: f32, 
        y: f32 
    },
}

/// A scene graph that stores drawing commands for retained-mode rendering.
/// 
/// ## Retained Mode vs Immediate Mode
/// 
/// In **immediate mode**, you redraw everything every frame. The Scene system
/// uses **retained mode**, where:
/// - Commands are stored in a list
/// - The scene is only rebuilt when marked as "dirty"
/// - Rendering simply replays the stored commands
/// 
/// ## Dirty Flag Pattern
/// 
/// The `dirty` flag tracks whether the scene needs to be rebuilt:
/// - `true`: Scene has changed, needs rebuilding
/// - `false`: Scene is unchanged, can skip the update callback
/// 
/// This optimizes performance by avoiding unnecessary work.
pub struct Scene {
    /// List of all drawing commands in order
    commands: Vec<DrawCommand>,
    
    /// Whether the scene needs to be rebuilt
    dirty: bool,
}

impl Scene {
    /// Create a new empty scene.
    /// 
    /// The scene starts as "dirty" to ensure it gets drawn on the first frame.
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            dirty: true,
        }
    }

    // ========================================================================
    // Drawing API - Add commands to the scene
    // ========================================================================

    /// Add a filled rectangle to the scene.
    /// 
    /// # Arguments
    /// * `x`, `y` - Top-left corner position (logical coordinates)
    /// * `w`, `h` - Width and height
    /// * `color` - RGBA color values from 0.0 to 1.0
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        self.commands.push(DrawCommand::Rect { x, y, w, h, color });
        self.dirty = true;
    }

    /// Add a filled circle to the scene.
    /// 
    /// # Arguments
    /// * `cx`, `cy` - Center position (logical coordinates)
    /// * `radius` - Circle radius
    /// * `color` - RGBA color values from 0.0 to 1.0
    pub fn circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4]) {
        self.commands.push(DrawCommand::Circle { cx, cy, radius, color });
        self.dirty = true;
    }

    /// Add a rounded rectangle to the scene (useful for buttons, cards).
    /// 
    /// # Arguments
    /// * `x`, `y` - Top-left corner position (logical coordinates)
    /// * `w`, `h` - Width and height
    /// * `radius` - Corner radius (automatically clamped to half the smaller dimension)
    /// * `color` - RGBA color values from 0.0 to 1.0
    pub fn rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4]) {
        self.commands.push(DrawCommand::RoundedRect { x, y, w, h, radius, color });
        self.dirty = true;
    }

    /// Add text to the scene.
    /// 
    /// # Arguments
    /// * `text` - The text to display (can be `&str` or `String`)
    /// * `x`, `y` - Top-left position for the text (logical coordinates)
    pub fn text(&mut self, text: impl Into<String>, x: f32, y: f32) {
        self.commands.push(DrawCommand::Text { 
            text: text.into(), 
            x, 
            y 
        });
        self.dirty = true;
    }

    // ========================================================================
    // Scene Management
    // ========================================================================

    /// Remove all commands from the scene.
    /// 
    /// This marks the scene as dirty, triggering a rebuild on the next frame.
    pub fn clear(&mut self) {
        self.commands.clear();
        self.dirty = true;
    }

    /// Get read-only access to all drawing commands.
    /// 
    /// Used internally during rendering to replay the scene.
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    // ========================================================================
    // Dirty Flag Management
    // ========================================================================

    /// Check if the scene needs to be rebuilt.
    /// 
    /// Returns `true` if any changes have been made since the last render.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Mark the scene as clean (up-to-date).
    /// 
    /// Called automatically after rendering. Only call this manually if you
    /// know the scene doesn't need rebuilding.
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Force the scene to be rebuilt on the next frame.
    /// 
    /// Useful when external state changes (like window resize) that don't
    /// directly modify the scene but require it to be redrawn.
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
