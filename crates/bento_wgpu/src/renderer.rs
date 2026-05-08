use crate::{
    context::RenderContext,
    pipelines::{
        image::{ImageInstance, ImagePipeline},
        rect::{RectInstance, RectPipeline},
        text::{TextPipeline, TextSpec},
    },
    surface::Surface,
};
use bento_shared::{GroupNode, Node, Scene, SceneNodeId};
use wgpu;

// accumulated group state

struct Accumulated {
    rotate: f32,
    scale_x: f32,
    scale_y: f32,
    opacity: f32,
    clip: Option<[f32; 4]>,
    offset_x: f32,
    offset_y: f32,
}

impl Accumulated {
    fn identity() -> Self {
        Self {
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: None,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    fn combine_with_group(&self, g: &GroupNode) -> Self {
        Self {
            rotate: self.rotate + g.rotate,
            scale_x: self.scale_x * g.scale_x,
            scale_y: self.scale_y * g.scale_y,
            opacity: self.opacity * g.opacity.unwrap_or(1.0),
            clip: merge_clip(self.clip, g.clip),
            offset_x: self.offset_x + g.offset_x,
            offset_y: self.offset_y + g.offset_y,
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
    image: ImagePipeline,
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
            image: ImagePipeline::new(
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

        // sort root nodes by z
        let mut root_ids = scene.root.clone();
        root_ids.sort_by_key(|id| match scene.nodes.get(id.0) {
            Some(Node::Rect(r)) => r.z,
            Some(Node::Text(t)) => t.z,
            Some(Node::Image(i)) => i.z,
            Some(Node::Group(g)) => g.z,
            None => 0,
        });

        let mut specs: Vec<TextSpec> = Vec::new();
        let mut text_slot: usize = 0;

        Self::traverse(
            &root_ids,
            scene,
            &Accumulated::identity(),
            &mut self.rect,
            &mut self.image,
            surface.scale,
            &mut specs,
            &mut text_slot,
        );
        self.rect.upload(&ctx.device, &ctx.queue);
        self.text
            .prepare(&specs, font_system, &ctx.device, &ctx.queue);
        self.image.upload(&ctx.device, &ctx.queue);

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

            Self::draw_nodes(
                &root_ids,
                scene,
                &self.rect,
                &self.text,
                &self.image,
                &mut pass,
                line_offset,
            );
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface) {
        self.rect.resize(&ctx.queue, surface.width, surface.height);
        self.text
            .resize(&ctx.queue, surface.width, surface.height, surface.scale);
        self.image
            .resize(&ctx.queue, surface.width, surface.height, surface.scale);
    }

    fn traverse(
        ids: &[SceneNodeId],
        scene: &mut Scene,
        acc: &Accumulated,
        rect: &mut RectPipeline,
        image: &mut ImagePipeline,
        scale: f32,
        specs: &mut Vec<TextSpec>,
        text_slot: &mut usize,
    ) {
        for &id in ids {
            let node = match scene.nodes.get_mut(id.0) {
                Some(n) => n,
                None => continue,
            };

            match node {
                Node::Rect(r) => {
                    if r.slot == u32::MAX {
                        r.slot = rect.alloc_slot();
                    }
                    let final_clip = merge_clip(acc.clip, r.clip);
                    rect.write_slot(
                        r.slot,
                        RectInstance {
                            pos_size: [r.x + acc.offset_x, r.y + acc.offset_y, r.w, r.h],
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
                            transform: bento_shared::math::transform(
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
                    let final_clip = merge_clip(acc.clip, t.clip);
                    specs.push(TextSpec {
                        text: t.text.clone(),
                        x: t.x + acc.offset_x,
                        y: t.y + acc.offset_y,
                        size: t.size,
                        color: t.color,
                        rotate: t.rotate + acc.rotate,
                        scale_x: t.scale_x * acc.scale_x,
                        scale_y: t.scale_y * acc.scale_y,
                        weight: t.weight,
                        italic: t.italic,
                        font_family: t.font_family.clone(),
                        max_width: t.max_width,
                        line_height: t.line_height,
                        letter_spacing: t.letter_spacing,
                        align: t.align.clone(),
                        opacity: t.opacity * acc.opacity,
                        clip: final_clip,
                        color_ranges: t.color_ranges.clone(),
                        background_ranges: t.background_ranges.clone(),
                        underline_ranges: t.underline_ranges.clone(),
                        strikethrough_ranges: t.strikethrough_ranges.clone(),
                        weight_ranges: t.weight_ranges.clone(),
                        italic_ranges: t.italic_ranges.clone(),
                        font_family_ranges: t.font_family_ranges.clone(),
                    });
                }

                Node::Image(img) => {
                    if img.slot == usize::MAX {
                        img.slot = image.alloc_slot();
                    }
                    let final_clip = merge_clip(acc.clip, img.clip);
                    image.write_slot(
                        img.slot,
                        ImageInstance {
                            pos_size: [img.x + acc.offset_x, img.y + acc.offset_y, img.w, img.h],
                            radii: img.radii,
                            border_color: [
                                img.border_color[0],
                                img.border_color[1],
                                img.border_color[2],
                                img.border_color[3] * img.opacity * acc.opacity,
                            ],
                            border_widths: img.border_widths,
                            transform: bento_shared::math::transform(
                                img.rotate + acc.rotate,
                                img.scale_x * acc.scale_x,
                                img.scale_y * acc.scale_y,
                            ),
                            clip: scale_clip(final_clip, scale),
                            opacity: img.opacity * acc.opacity,
                            _pad: [0.0; 3],
                        },
                        img.image_id,
                    );
                }

                Node::Group(g) => {
                    let child_acc = acc.combine_with_group(g);
                    let mut child_ids = g.children.clone();
                    child_ids.sort_by_key(|cid| match scene.nodes.get(cid.0) {
                        Some(Node::Rect(r)) => r.z,
                        Some(Node::Text(t)) => t.z,
                        Some(Node::Image(i)) => i.z,
                        Some(Node::Group(g)) => g.z,
                        None => 0,
                    });
                    Self::traverse(
                        &child_ids, scene, &child_acc, rect, image, scale, specs, text_slot,
                    );
                }
            }
        }
    }

    fn draw_nodes<'pass>(
        ids: &[SceneNodeId],
        scene: &'pass Scene,
        rect: &'pass RectPipeline,
        text: &'pass TextPipeline,
        image: &'pass ImagePipeline,
        pass: &mut wgpu::RenderPass<'pass>,
        line_offset: u32,
    ) {
        for &id in ids {
            let node = match scene.nodes.get(id.0) {
                Some(n) => n,
                None => continue,
            };

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
                Node::Image(img) => {
                    image.draw_slot(pass, img.slot);
                }
                Node::Group(g) => {
                    let child_ids = g.children.clone();
                    Self::draw_nodes(&child_ids, scene, rect, text, image, pass, line_offset);
                }
            }
        }
    }

    pub fn upload_image(
        &mut self,
        id: u64,
        bytes: &[u8],
        width: u32,
        height: u32,
        ctx: &RenderContext,
    ) {
        self.image
            .upload_image(id, bytes, width, height, &ctx.device, &ctx.queue);
    }
}
