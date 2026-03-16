use crate::color::Color;
use crate::fonts::Fonts;
use crate::mouse::MouseState;
use crate::render::draw_ctx::DrawContext;
use crate::render::gpu::GpuContext;
use std::sync::Arc;
use winit::window::Window;

pub struct WindowState {
    pub window: Arc<Window>,
    pub gpu: GpuContext,
    pub clear_color: Color,
    pub draw: DrawContext,
    pub fonts: Fonts,
    pub mouse: MouseState,
}

impl WindowState {
    pub fn new(window: Arc<Window>, gpu: GpuContext, clear_color: Color) -> Self {
        let size = window.inner_size();
        let scale = window.scale_factor();
        let draw = DrawContext::new(
            &gpu.device,
            &gpu.queue,
            gpu.format,
            size.width as f32 / scale as f32,  // logical
            size.height as f32 / scale as f32, // logical
            scale as f32,
        );
        Self {
            window,
            gpu,
            clear_color,
            draw,
            fonts: Fonts::new(),
            mouse: MouseState::default(),
        }
    }

    pub fn begin(&mut self) {
        self.draw.clear();
    }

    pub fn resize_and_rescale(&mut self) {
        let size = self.window.inner_size();
        let scale = self.window.scale_factor();

        self.gpu.resize(size.width, size.height);
        self.draw
            .set_scale(scale as f32, size.width as f32 / scale as f32, size.height as f32 / scale as f32);
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw();
    }

    pub fn render(&mut self) {
        let frame = match self.gpu.begin_frame() {
            Ok(f) => f,
            Err(_) => return,
        };

        let (mut encoder, finisher, view) = frame.begin();

        {
            let c = self.clear_color;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: c.r as f64,
                            g: c.g as f64,
                            b: c.b as f64,
                            a: c.a as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.draw
                .render(&self.gpu.device, &self.gpu.queue, &mut pass);
        }

        finisher.present(encoder, &self.gpu.queue);
    }
}
