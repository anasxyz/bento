use crate::context::RenderContext;
use crate::surface::Surface;
use wgpu;

use crate::pipelines::rect::{RectPipeline, RectInstance};

pub struct Renderer {
    rect: RectPipeline,
}

impl Renderer {
    pub fn new(ctx: &RenderContext, surface: &Surface) -> Self {
        Self {
            rect: RectPipeline::new(
                &ctx.device,
                &ctx.queue,
                surface.format,
                surface.width,
                surface.height,
            ),
        }
    }

    pub fn render(
        &mut self,
        ctx: &mut RenderContext,
        surface: &mut Surface,
        clear_color: [f32; 4],
    ) {
        // get the next frame from the swapchain
        let frame = match surface.surface.get_current_texture() {
            Ok(f) => f,
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                surface.surface.configure(&ctx.device, &surface.config);
                return;
            }
            Err(_) => return,
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame"),
            });

        let [r, g, b, a] = clear_color;
        {
            // a render pass clears the screen and is where draw calls go
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: r as f64,
                            g: g as f64,
                            b: b as f64,
                            a: a as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // draw calls
            self.rect.draw(
                &[
                    RectInstance {
                        pos_size: [50.0, 50.0, 200.0, 100.0],
                        color: [0.2, 0.5, 1.0, 1.0],
                        radii: [6.0; 4]
                    },
                ],
                &ctx.queue,
                &mut pass,
            );
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface) {
        // pipelines will resize here later
    }
}
