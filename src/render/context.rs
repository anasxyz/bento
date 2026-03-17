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

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, p: RectParams) {
        self.shapes.draw(x, y, w, h, p);
    }

    pub fn draw_text(&mut self, x: f32, y: f32, content: &str, p: TextParams) {
        self.text.draw(&mut self.font_system, content, x, y, p);
    }

    pub fn draw_clear(&mut self, color: [f32; 4]) {
        self.shapes.draw(
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
        self.text.trim_atlas();
    }
}
