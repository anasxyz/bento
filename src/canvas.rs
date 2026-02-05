// src/canvas.rs

use crate::Scene;

/// Drawing context with direct shape methods
pub struct Canvas<'a> {
    pub scene: &'a mut Scene,
    pub width: f32,
    pub height: f32,
    pub scale_factor: f64,
    pub mouse: MouseState,
}

impl<'a> Canvas<'a> {
    // ========================================================================
    // Direct Shape Drawing (no .scene needed!)
    // ========================================================================

    /// Draw a rectangle
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.scene.rect(x, y, w, h, color, outline_color, outline_thickness);
    }

    /// Draw a circle
    pub fn circle(&mut self, cx: f32, cy: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.scene.circle(cx, cy, radius, color, outline_color, outline_thickness);
    }

    /// Draw a rounded rectangle
    pub fn rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [f32; 4], outline_color: [f32; 4], outline_thickness: f32) {
        self.scene.rounded_rect(x, y, w, h, radius, color, outline_color, outline_thickness);
    }

    /// Draw text
    pub fn text(&mut self, text: impl Into<String>, x: f32, y: f32) {
        self.scene.text(text, x, y);
    }
}

/// Mouse state available to user code
#[derive(Clone, Copy, Debug)]
pub struct MouseState {
    pub x: f32,
    pub y: f32,
    pub left_pressed: bool,
    pub left_just_pressed: bool,
    pub left_just_released: bool,
    pub right_pressed: bool,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            left_pressed: false,
            left_just_pressed: false,
            left_just_released: false,
            right_pressed: false,
        }
    }
}
