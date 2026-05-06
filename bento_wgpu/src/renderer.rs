use crate::{
    context::RenderContext,
    pipelines::{
        rect::{RectInstance, RectPipeline},
        text::{TextPipeline, TextSpec},
    },
    scene::{GroupNode, Node, Scene},
    surface::Surface,
};
use wgpu;

// accumulated group state

struct Accumulated {
    x: f32,
    y: f32,
    rotate: f32,
    scale_x: f32,
    scale_y: f32,
    opacity: f32,
    clip: Option<[f32; 4]>,
}

impl Accumulated {
    fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: None,
        }
    }

    fn combine_with_group(&self, g: &GroupNode) -> Self {
        let g_clip_offset = g
            .clip
            .map(|c| [c[0] + self.x + g.x, c[1] + self.y + g.y, c[2], c[3]]);

        Self {
            x: self.x + g.x,
            y: self.y + g.y,
            rotate: self.rotate + g.rotate,
            scale_x: self.scale_x * g.scale_x,
            scale_y: self.scale_y * g.scale_y,
            opacity: self.opacity * g.opacity.unwrap_or(1.0),
            clip: merge_clip(self.clip, g_clip_offset),
        }
    }
}

fn merge_clip(a: Option<[f32; 4]>, b: Option<[f32; 4]>) -> Option<[f32; 4]> {
    match (a, b) {
        (Some(a), Some(b)) => Some(intersect_clip(a, b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn intersect_clip(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    let x = a[0].max(b[0]);
    let y = a[1].max(b[1]);
    let x2 = (a[0] + a[2]).min(b[0] + b[2]);
    let y2 = (a[1] + a[3]).min(b[1] + b[3]);
    [x, y, (x2 - x).max(0.0), (y2 - y).max(0.0)]
}

fn scale_clip(clip: Option<[f32; 4]>, scale: f32) -> [f32; 4] {
    clip.map(|c| [c[0] * scale, c[1] * scale, c[2] * scale, c[3] * scale])
        .unwrap_or([0.0, 0.0, f32::MAX, f32::MAX])
}

// renderer

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

        // traverse scene

        let mut specs: Vec<TextSpec> = Vec::new();
        let mut text_slot: usize = 0;

        Self::traverse(
            &mut scene.nodes,
            &Accumulated::identity(),
            &mut self.rect,
            surface.scale,
            &mut specs,
            &mut text_slot,
        );
        self.rect.upload(&ctx.device, &ctx.queue);
        self.text
            .prepare(&specs, font_system, &ctx.device, &ctx.queue);

        // upload decoration rects into transient buffer before pass begins
        let all_bg_rects: Vec<RectInstance> = self.text.bg_rects.clone();
        let all_line_rects: Vec<RectInstance> = self.text.line_rects.clone();
        let mut combined = all_bg_rects.clone();
        combined.extend_from_slice(&all_line_rects);
        self.rect
            .prepare_transient(&combined, &ctx.device, &ctx.queue);
        let line_offset = all_bg_rects.len() as u32;

        // draw

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

            Self::draw_nodes(&scene.nodes, &self.rect, &self.text, &mut pass, line_offset);
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface) {
        self.rect.resize(&ctx.queue, surface.width, surface.height);
        self.text
            .resize(&ctx.queue, surface.width, surface.height, surface.scale);
    }

    // traverses the scene and prepares all nodes recursively
    fn traverse<'a>(
        nodes: &'a mut Vec<Node>,
        acc: &Accumulated,
        rect: &mut RectPipeline,
        scale: f32,
        specs: &mut Vec<TextSpec<'a>>,
        text_slot: &mut usize,
    ) {
        nodes.sort_by_key(|n| match n {
            Node::Rect(r) => r.z,
            Node::Text(t) => t.z,
            Node::Group(g) => g.z,
        });

        for node in nodes.iter_mut() {
            match node {
                Node::Rect(r) => {
                    if r.slot == u32::MAX {
                        r.slot = rect.alloc_slot();
                    }
                    let r_clip_offset = r.clip.map(|c| [c[0] + acc.x, c[1] + acc.y, c[2], c[3]]);
                    let final_clip = merge_clip(acc.clip, r_clip_offset);
                    rect.write_slot(
                        r.slot,
                        RectInstance {
                            pos_size: [r.x + acc.x, r.y + acc.y, r.w, r.h],
                            color: [
                                r.color[0],
                                r.color[1],
                                r.color[2],
                                r.color[3] * r.opacity * acc.opacity,
                            ],
                            radii: r.radii,
                            border_color: [
                                r.border_color[0],
                                r.border_color[1],
                                r.border_color[2],
                                r.border_color[3] * r.opacity * acc.opacity,
                            ],
                            border_widths: r.border_widths,
                            transform: crate::math::transform(
                                r.rotate + acc.rotate,
                                r.scale_x * acc.scale_x,
                                r.scale_y * acc.scale_y,
                            ),
                            clip: scale_clip(final_clip, scale),
                        },
                    );
                }

                Node::Text(t) => {
                    t.slot = *text_slot;
                    *text_slot += 1;
                    let t_clip_offset = t.clip.map(|c| [c[0] + acc.x, c[1] + acc.y, c[2], c[3]]);
                    let final_clip = merge_clip(acc.clip, t_clip_offset);
                    specs.push(TextSpec {
                        text: t.text.as_str(),
                        x: t.x + acc.x,
                        y: t.y + acc.y,
                        size: t.size,
                        color: t.color,
                        rotate: t.rotate + acc.rotate,
                        scale_x: t.scale_x * acc.scale_x,
                        scale_y: t.scale_y * acc.scale_y,
                        weight: t.weight,
                        italic: t.italic,
                        font_family: t.font_family.as_str(),
                        max_width: t.max_width,
                        opacity: t.opacity * acc.opacity,
                        clip: final_clip,

                        color_ranges: &t.color_ranges,
                        background_ranges: &t.background_ranges,
                        underline_ranges: &t.underline_ranges,
                        strikethrough_ranges: &t.strikethrough_ranges,
                        weight_ranges: &t.weight_ranges,
                        italic_ranges: &t.italic_ranges,
                        font_family_ranges: &t.font_family_ranges,
                    });
                }

                Node::Group(g) => {
                    let child_acc = acc.combine_with_group(g);
                    Self::traverse(&mut g.children, &child_acc, rect, scale, specs, text_slot);
                }
            }
        }
    }

    // draws all nodes recursively in traversal order
    fn draw_nodes<'pass>(
        nodes: &'pass Vec<Node>,
        rect: &'pass RectPipeline,
        text: &'pass TextPipeline,
        pass: &mut wgpu::RenderPass<'pass>,
        line_offset: u32,
    ) {
        for node in nodes {
            match node {
                Node::Rect(r) => {
                    rect.draw_slot(pass, r.slot);
                }
                Node::Text(t) => {
                    if let Some(&(start, end)) = text.bg_ranges.get(t.slot) {
                        rect.draw_transient_range(pass, start as u32, (end - start) as u32);
                    }
                    text.draw_range(pass, t.slot);
                    if let Some(&(start, end)) = text.line_ranges.get(t.slot) {
                        rect.draw_transient_range(
                            pass,
                            line_offset + start as u32,
                            (end - start) as u32,
                        );
                    }
                }
                Node::Group(g) => {
                    Self::draw_nodes(&g.children, rect, text, pass, line_offset);
                }
            }
        }
    }
}
