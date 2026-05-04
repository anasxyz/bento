use crate::{
    context::RenderContext,
    pipelines::rect::{RectInstance, RectPipeline},
    pipelines::text::TextPipeline,
    scene::{Node, Scene},
    surface::Surface,
};
use wgpu;

pub struct Renderer {
    rect: RectPipeline,
    text: TextPipeline,
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
            text: TextPipeline::new(
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
        font_system: &mut cosmic_text::FontSystem,
        surface: &mut Surface,
        clear_color: [f32; 4],
        scene: &Scene,
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

            let rects: Vec<RectInstance> = scene
                .nodes()
                .iter()
                .filter_map(|node| match node {
                    Node::Rect(r) => Some(*r),
                })
                .collect();

            // draw calls
            self.rect.draw(&rects, &ctx.queue, &mut pass);
            self.text.draw(
                "Hello world",
                50.0,
                50.0,
                12.0,
                [1.0, 1.0, 1.0, 1.0],
                font_system,
                &ctx.device,
                &ctx.queue,
                &mut pass,
            );
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface) {
        self.rect.resize(&ctx.queue, surface.width, surface.height);
        self.text.resize(&ctx.queue, surface.width, surface.height);
    }
}
