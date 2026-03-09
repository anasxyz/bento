use crate::render::shape_renderer::{ShapeDrawParams, ShapeRenderer};
use crate::render::text_renderer::{TextDrawParams, TextRenderer};
use glyphon::FontSystem;
use wgpu;

pub struct DrawContext {
    shapes: ShapeRenderer,
    text: TextRenderer,
    font_system: FontSystem,
    width: f32,
    height: f32,
    scale: f32,
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

    // logical pixels
    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.shapes.resize(width, height);
        self.text.resize(width, height, self.scale as f64);
    }

    pub fn set_scale(&mut self, scale: f32, width: f32, height: f32) {
        self.scale = scale;
        self.width = width;
        self.height = height;
        self.shapes.set_scale(scale, width, height);
        self.text.resize(width, height, scale as f64);
    }

    // all inputs in logical pixels
    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, p: ShapeDrawParams) {
        self.shapes.draw_rect(x, y, w, h, p);
    }

    pub fn draw_text(&mut self, x: f32, y: f32, content: &str, p: TextDrawParams) {
        self.text.draw(&mut self.font_system, content, x, y, p);
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

    pub fn clear(&mut self) {
        self.shapes.clear();
        self.text.clear();
    }
}
