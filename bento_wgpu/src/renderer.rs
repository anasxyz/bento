use crate::allocator::SlotAllocator;
use crate::context::RenderContext;
use crate::nodes::*;
use crate::pipelines::{rect::RectPipeline, shadow::ShadowPipeline, text::TextPipeline};
use crate::scene::{SceneGraph, TraversalState};
use crate::surface::Surface;
use wgpu;

pub struct RendererStats {
    pub rect_uploads: u32,
    pub rects_culled: u32,
    pub texts_culled: u32,
}

pub struct Renderer {
    rect_pipeline: RectPipeline,
    text_pipeline: TextPipeline,
    shadow_pipeline: ShadowPipeline,
    rect_alloc: SlotAllocator,
    shadow_alloc: SlotAllocator,
    pub stats: RendererStats,
}

struct RectCall {
    node_idx: usize,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: [f32; 4],
    radius: f32,
    border_color: [f32; 4],
    border_widths: [f32; 4],
    clip: Option<[f32; 4]>,
}

struct ShadowCall {
    node_idx: usize,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: [f32; 4],
    blur: f32,
    radius: f32,
    offset_x: f32,
    offset_y: f32,
    clip: Option<[f32; 4]>,
}

struct TextCall {
    x: f32,
    y: f32,
    content: String,
    family: String,
    size: f32,
    weight: u16,
    italic: bool,
    color: [f32; 4],
    width: f32,
    clip: Option<[f32; 4]>,
}

impl Renderer {
    pub fn new(ctx: &RenderContext, surface: &Surface) -> Self {
        let fmt = surface.format;
        let sw = surface.physical_width() as f32;
        let sh = surface.physical_height() as f32;
        let mut r = Self {
            rect_pipeline: RectPipeline::new(&ctx.device, fmt, sw, sh),
            shadow_pipeline: ShadowPipeline::new(&ctx.device, &ctx.queue, fmt, sw, sh),
            text_pipeline: TextPipeline::new(
                &ctx.device,
                &ctx.queue,
                fmt,
                surface.width,
                surface.height,
                surface.scale,
            ),
            rect_alloc: SlotAllocator::new(),
            shadow_alloc: SlotAllocator::new(),
            stats: RendererStats {
                rect_uploads: 0,
                rects_culled: 0,
                texts_culled: 0,
            },
        };
        r.rect_pipeline.resize(&ctx.queue, sw, sh);
        r
    }

    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface, scene: &mut SceneGraph) {
        let sw = surface.physical_width() as f32;
        let sh = surface.physical_height() as f32;
        self.rect_pipeline.resize(&ctx.queue, sw, sh);
        self.shadow_pipeline.resize(&ctx.queue, sw, sh);
        self.text_pipeline
            .resize(surface.width, surface.height, surface.scale);
    }

    pub fn invalidate(&mut self, scene: &mut SceneGraph) {
        self.rect_pipeline.invalidate();
        self.shadow_pipeline.invalidate();
        for (_, node) in &mut scene.nodes {
            match node {
                SceneNode::Rect(n) => n.slot = u32::MAX,
                SceneNode::Shadow(n) => n.slot = u32::MAX,
                _ => {}
            }
        }
    }

    pub fn render(
        &mut self,
        ctx: &mut RenderContext,
        font_system: &mut glyphon::FontSystem,
        surface: &mut Surface,
        scene: &mut SceneGraph,
        clear_color: [f32; 4],
    ) {
        let frame = match surface.surface.get_current_texture() {
            Ok(f) => f,
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                surface.surface.configure(&ctx.device, &surface.config);
                return;
            }
            Err(_) => return,
        };

        let scale = surface.scale;
        let screen_w = surface.width;
        let screen_h = surface.height;

        self.stats.rect_uploads = 0;
        self.stats.rects_culled = 0;
        self.stats.texts_culled = 0;

        // traverse scene tree
        // collect calls with their slab indices
        // order here is guaranteed to match the order we upload below
        let mut rect_calls: Vec<RectCall> = Vec::new();
        let mut shadow_calls: Vec<ShadowCall> = Vec::new();
        let mut text_calls: Vec<TextCall> = Vec::new();
        let mut culled_rect_nodes: Vec<usize> = Vec::new();
        let mut culled_rects = 0u32;
        let mut culled_texts = 0u32;

        let root = scene.root;
        scene.traverse(
            root,
            TraversalState::new(),
            &mut |node, node_idx, state| match node {
                SceneNode::Rect(n) if n.visible => {
                    let x = n.x + state.offset_x;
                    let y = n.y + state.offset_y;
                    let in_window = x < screen_w && y < screen_h && x + n.w > 0.0 && y + n.h > 0.0;
                    let in_clip = state.clip.map_or(true, |[cx, cy, cx2, cy2]| {
                        x < cx2 && y < cy2 && x + n.w > cx && y + n.h > cy
                    });
                    if in_window && in_clip {
                        rect_calls.push(RectCall {
                            node_idx,
                            x,
                            y,
                            w: n.w,
                            h: n.h,
                            color: apply_opacity(n.color, state.opacity),
                            radius: n.radius,
                            border_color: apply_opacity(n.border_color, state.opacity),
                            border_widths: n.border_widths,
                            clip: state.clip,
                        });
                    } else {
                        culled_rects += 1;
                        culled_rect_nodes.push(node_idx);
                    }
                }
                SceneNode::Shadow(n) if n.visible => {
                    let x = n.x + state.offset_x;
                    let y = n.y + state.offset_y;
                    let in_window = x < screen_w && y < screen_h && x + n.w > 0.0 && y + n.h > 0.0;
                    let in_clip = state.clip.map_or(true, |[cx, cy, cx2, cy2]| {
                        x < cx2 && y < cy2 && x + n.w > cx && y + n.h > cy
                    });
                    if in_window && in_clip {
                        shadow_calls.push(ShadowCall {
                            node_idx,
                            x,
                            y,
                            w: n.w,
                            h: n.h,
                            color: apply_opacity(n.color, state.opacity),
                            blur: n.blur,
                            radius: n.radius,
                            offset_x: n.offset_x,
                            offset_y: n.offset_y,
                            clip: state.clip,
                        });
                    }
                }
                SceneNode::Text(n) if n.visible && !n.content.is_empty() => {
                    let x = n.x + state.offset_x;
                    let y = n.y + state.offset_y;
                    let in_window = x < screen_w
                        && y < screen_h
                        && x + n.width > 0.0
                        && y + n.size * 20.0 > 0.0;
                    let in_clip = state.clip.map_or(true, |[cx, cy, cx2, cy2]| {
                        x < cx2 && y < cy2 && x + n.width > cx && y + n.size * 20.0 > cy
                    });
                    if in_window && in_clip {
                        text_calls.push(TextCall {
                            x,
                            y,
                            content: n.content.clone(),
                            family: n.family.clone(),
                            size: n.size,
                            weight: n.weight,
                            italic: n.italic,
                            color: apply_opacity(n.color, state.opacity),
                            width: n.width,
                            clip: state.clip,
                        });
                    } else {
                        culled_texts += 1;
                    }
                }
                _ => {}
            },
        );

        self.stats.rects_culled = culled_rects;
        self.stats.texts_culled = culled_texts;

        // assign slots and upload using the node_idx stored on each call
        // this guarantees the call data matches the correct node
        for c in &rect_calls {
            let n = match &mut scene.nodes[c.node_idx] {
                SceneNode::Rect(n) => n,
                _ => continue,
            };
            if n.slot == u32::MAX {
                n.slot = self.rect_alloc.alloc();
            }
            let slot = n.slot as usize;
            self.rect_pipeline.ensure_slot(slot);
            self.rect_pipeline.write_slot(
                slot,
                c.x,
                c.y,
                c.w,
                c.h,
                c.color,
                c.radius,
                c.border_color,
                c.border_widths,
                c.clip,
                scale,
            );
        }

        // clear slots for invisible or culled rects
        for (_, node) in &mut scene.nodes {
            if let SceneNode::Rect(n) = node {
                if !n.visible && n.slot != u32::MAX {
                    self.rect_pipeline.clear_slot(n.slot as usize);
                }
            }
        }
        for node_idx in culled_rect_nodes {
            if let SceneNode::Rect(n) = &mut scene.nodes[node_idx] {
                if n.slot != u32::MAX {
                    self.rect_pipeline.clear_slot(n.slot as usize);
                }
            }
        }

        for c in &shadow_calls {
            let n = match &mut scene.nodes[c.node_idx] {
                SceneNode::Shadow(n) => n,
                _ => continue,
            };
            if n.slot == u32::MAX {
                n.slot = self.shadow_alloc.alloc();
            }
            let slot = n.slot as usize;
            self.shadow_pipeline.ensure_slot(slot);
            self.shadow_pipeline.write_slot(
                slot, c.x, c.y, c.w, c.h, c.color, c.blur, c.radius, c.offset_x, c.offset_y, scale,
            );
        }

        // clear slots for invisible shadows
        for (_, node) in &mut scene.nodes {
            if let SceneNode::Shadow(n) = node {
                if !n.visible && n.slot != u32::MAX {
                    self.shadow_pipeline.clear_slot(n.slot as usize);
                }
            }
        }

        // submit text
        self.text_pipeline.begin_frame();
        for call in &text_calls {
            self.text_pipeline.submit(
                font_system,
                call.x,
                call.y,
                &call.content,
                &call.family,
                call.size,
                call.weight,
                call.italic,
                call.color,
                call.width,
                call.clip,
            );
        }

        // render pass
        let frame_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("bento frame"),
            });
        let [r, g, b, a] = clear_color;
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bento pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view,
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

            self.shadow_pipeline
                .render(&ctx.device, &ctx.queue, &mut pass);
            self.rect_pipeline
                .render(&ctx.device, &ctx.queue, &mut pass);
            self.text_pipeline
                .render(font_system, &ctx.device, &ctx.queue, &mut pass);
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        self.text_pipeline.trim_atlas();
        self.stats.rect_uploads = self.rect_pipeline.upload_count;
    }

    pub fn free_rect_slot(&mut self, slot: u32) {
        if slot != u32::MAX {
            self.rect_pipeline.clear_slot(slot as usize);
            self.rect_alloc.free(slot);
        }
    }

    pub fn free_shadow_slot(&mut self, slot: u32) {
        if slot != u32::MAX {
            self.shadow_pipeline.clear_slot(slot as usize);
            self.shadow_alloc.free(slot);
        }
    }
}

fn apply_opacity(color: [f32; 4], opacity: f32) -> [f32; 4] {
    [color[0], color[1], color[2], color[3] * opacity]
}
