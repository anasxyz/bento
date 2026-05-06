use crate::{
    context::RenderContext,
    pipelines::{
        rect::{RectInstance, RectPipeline},
        text::{TextPipeline, TextSpec},
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
                surface.scale,
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

        // prepare rects

        for node in &mut scene.nodes {
            if let Node::Rect(r) = node {
                if r.slot == u32::MAX {
                    r.slot = self.rect.alloc_slot();
                }
                self.rect.write_slot(
                    r.slot,
                    RectInstance {
                        pos_size: [r.x, r.y, r.w, r.h],
                        color: [r.color[0], r.color[1], r.color[2], r.color[3] * r.opacity],
                        radii: r.radii,
                        border_color: [
                            r.border_color[0],
                            r.border_color[1],
                            r.border_color[2],
                            r.border_color[3] * r.opacity,
                        ],
                        border_widths: r.border_widths,
                        transform: crate::math::transform(r.rotate, r.scale_x, r.scale_y),
                    },
                );
            }
        }
        self.rect.upload(&ctx.device, &ctx.queue);

        // prepare text

        let mut text_slot = 0usize;
        let specs: Vec<TextSpec> = scene
            .nodes
            .iter_mut()
            .filter_map(|n| match n {
                Node::Text(t) => {
                    t.slot = text_slot;
                    text_slot += 1;
                    Some(TextSpec {
                        text: t.text.as_str(),
                        x: t.x,
                        y: t.y,
                        size: t.size,
                        color: t.color,
                        rotate: t.rotate,
                        scale_x: t.scale_x,
                        scale_y: t.scale_y,
                        weight: t.weight,
                        italic: t.italic,
                        font_family: t.font_family.as_str(),
                        max_width: t.max_width,
                        opacity: t.opacity,

                        color_ranges: &t.color_ranges,
                        background_ranges: &t.background_ranges,
                        underline_ranges: &t.underline_ranges,
                        strikethrough_ranges: &t.strikethrough_ranges,
                        weight_ranges: &t.weight_ranges,
                        italic_ranges: &t.italic_ranges,
                        font_family_ranges: &t.font_family_ranges,
                    })
                }
                _ => None,
            })
            .collect();

        self.text
            .prepare(&specs, font_system, &ctx.device, &ctx.queue);

        // upload all decoration rects into transient buffer before pass begins
        let all_bg_rects: Vec<RectInstance> = self.text.bg_rects.clone();
        let all_line_rects: Vec<RectInstance> = self.text.line_rects.clone();
        let mut combined = all_bg_rects.clone();
        combined.extend_from_slice(&all_line_rects);
        self.rect
            .prepare_transient(&combined, &ctx.device, &ctx.queue);
        let line_offset = all_bg_rects.len() as u32;

        // draw

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
                    Node::Rect(r) => {
                        self.rect.draw_slot(&mut pass, r.slot);
                    }
                    Node::Text(t) => {
                        // this is in order of which to be drawn first
                        // so backgrounds first, then glyphs, then decorations

                        // background rects
                        if let Some(&(start, end)) = self.text.bg_ranges.get(t.slot) {
                            self.rect.draw_transient_range(
                                &mut pass,
                                start as u32,
                                (end - start) as u32,
                            );
                        }
                        // glyphs
                        self.text.draw_range(&mut pass, t.slot);
                        // line decorations
                        if let Some(&(start, end)) = self.text.line_ranges.get(t.slot) {
                            self.rect.draw_transient_range(
                                &mut pass,
                                line_offset + start as u32,
                                (end - start) as u32,
                            );
                        }
                    }
                }
            }
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface) {
        self.rect.resize(&ctx.queue, surface.width, surface.height);
        self.text
            .resize(&ctx.queue, surface.width, surface.height, surface.scale);
    }
}
