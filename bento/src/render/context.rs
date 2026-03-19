use super::shapes::{RectParams, ShapeRenderer};
use super::text::{TextParams, TextRenderer};
use glyphon::FontSystem;
use wgpu;

pub struct DrawContext {
    shapes: ShapeRenderer,
    text: TextRenderer,
    pub font_system: FontSystem,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
}

impl DrawContext {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        width: f32,
        height: f32,
        scale: f32,
    ) -> Self {
        Self {
            shapes: ShapeRenderer::new(device, format, width, height, scale),
            text: TextRenderer::new(device, queue, format),
            font_system: FontSystem::new(),
            width,
            height,
            scale,
        }
    }

    pub fn set_scale(&mut self, scale: f32, width: f32, height: f32) {
        self.scale = scale;
        self.width = width;
        self.height = height;
        self.shapes.set_scale(scale, width, height);
        self.text.resize(width, height, scale as f64);
    }

    // clear rect lives at slot 0, always written each frame
    pub fn draw_clear(&mut self, color: [f32; 4]) {
        self.shapes.write_slot(
            0,
            0.0,
            0.0,
            self.width,
            self.height,
            RectParams {
                color,
                radius: 0.0,
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                clip: None,
            },
        );
    }

    // element rect slots start at index 1 (0 is clear rect)
    // renderer calls these with idx = 1-based z-sorted rect position
    pub fn ensure_rect_slot(&mut self, idx: usize) {
        // idx is 0-based within element rects, so actual slot is idx + 1
        self.shapes.ensure_slot(idx + 1);
    }

    pub fn write_rect_slot(&mut self, idx: usize, x: f32, y: f32, w: f32, h: f32, p: RectParams) {
        self.shapes.write_slot(idx + 1, x, y, w, h, p);
    }

    // called after all rects submitted
    // frees slots beyond whats needed
    pub fn truncate_rect_slots(&mut self, count: usize) {
        // count element rects + 1 for clear rect
        self.shapes.truncate(count + 1);
    }

    pub fn invalidate_rects(&mut self) {
        self.shapes.invalidate();
    }

    pub fn draw_text(&mut self, x: f32, y: f32, content: &str, p: TextParams) {
        self.text.draw(&mut self.font_system, content, x, y, p);
    }

    pub fn clear_text(&mut self) {
        self.text.clear();
        self.text.trim_atlas();
    }

    pub fn render<'pass>(
        &'pass mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        self.shapes.render(device, queue, pass);
        self.text.render(
            &mut self.font_system,
            self.width,
            self.height,
            self.scale as f64,
            device,
            queue,
            pass,
        );
    }
}
