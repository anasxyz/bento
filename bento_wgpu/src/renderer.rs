use crate::{
    TextNode,
    context::RenderContext,
    pipelines::{
        rect::{RectInstance, RectPipeline},
        text::TextPipeline,
    },
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

        // collect rects
        let rects: Vec<RectInstance> = scene
            .nodes
            .iter()
            .filter_map(|n| match n {
                Node::Rect(r) => Some(RectInstance {
                    pos_size: [r.x, r.y, r.w, r.h],
                    color: r.color,
                    radii: r.radii,
                    border_color: r.border_color,
                    border_widths: r.border_widths,
                    transform: crate::math::transform(r.rotate, r.scale_x, r.scale_y),
                }),
                _ => None,
            })
            .collect();

        // collect texts
        let texts: Vec<(&str, f32, f32, f32, [f32; 4])> = scene
            .nodes
            .iter()
            .filter_map(|n| match n {
                Node::Text(t) => Some((t.text.as_str(), t.x, t.y, t.size, t.color)),
                _ => None,
            })
            .collect();

        // prepare textm, uploads glyph data to GPU before render pass
        self.text
            .prepare(&texts, font_system, &ctx.device, &ctx.queue);

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

            // issue draw calls
            self.rect.draw(&rects, &ctx.queue, &mut pass);
            self.text.draw(&mut pass);
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface) {
        self.rect.resize(&ctx.queue, surface.width, surface.height);
        self.text.resize(&ctx.queue, surface.width, surface.height);
    }
}
