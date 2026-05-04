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
        scene: &mut Scene,
    ) {
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

        // prepare phase

        // assign slots and write rect data
        for node in &mut scene.nodes {
            if let Node::Rect(r) = node {
                if r.slot == u32::MAX {
                    r.slot = self.rect.alloc_slot();
                }
                self.rect.write_slot(
                    r.slot,
                    RectInstance {
                        pos_size: [r.x, r.y, r.w, r.h],
                        color: r.color,
                        radii: r.radii,
                        border_color: r.border_color,
                        border_widths: r.border_widths,
                        transform: crate::math::transform(r.rotate, r.scale_x, r.scale_y),
                    },
                );
            }
        }
        self.rect.upload(&ctx.device, &ctx.queue);

        // prepare text
        let mut texts: Vec<(&str, f32, f32, f32, [f32; 4])> = Vec::new();
        for node in &mut scene.nodes {
            if let Node::Text(t) = node {
                t.slot = texts.len();
                texts.push((t.text.as_str(), t.x, t.y, t.size, t.color));
            }
        }
        self.text
            .prepare(&texts, font_system, &ctx.device, &ctx.queue);

        // draw phase

        let mut sorted: Vec<&Node> = scene.nodes.iter().collect();
        sorted.sort_by_key(|n| match n {
            Node::Rect(r) => r.z,
            Node::Text(t) => t.z,
        });

        let [r, g, b, a] = clear_color;
        {
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

            for node in &sorted {
                match node {
                    Node::Rect(r) => self.rect.draw_slot(&mut pass, r.slot),
                    Node::Text(t) => self.text.draw_range(&mut pass, t.slot),
                }
            }
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface) {
        self.rect.resize(&ctx.queue, surface.width, surface.height);
        self.text.resize(&ctx.queue, surface.width, surface.height);
    }
}
