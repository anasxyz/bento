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
    sel_slots_last_frame: Vec<u32>,
    sel_slots_this_frame: Vec<u32>,
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
    layer: u32,
    selection_start: Option<usize>,
    selection_end: Option<usize>,
    selection_color: [f32; 4],
    underlines: Vec<crate::nodes::TextDecoration>,
    strikethroughs: Vec<crate::nodes::TextDecoration>,
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
            sel_slots_last_frame: Vec::new(),
            sel_slots_this_frame: Vec::new(),
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
        self.text_pipeline.resize(
            &ctx.device,
            &ctx.queue,
            surface.width,
            surface.height,
            surface.scale,
        );
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
        font_system: &mut cosmic_text::FontSystem,
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

        self.sel_slots_this_frame.clear();

        let mut layers: std::collections::BTreeMap<
            u32,
            (Vec<ShadowCall>, Vec<RectCall>, Vec<TextCall>),
        > = std::collections::BTreeMap::new();
        let mut culled_rect_nodes: Vec<usize> = Vec::new();
        let mut culled_rects = 0u32;
        let mut culled_texts = 0u32;

        let root = scene.root;
        scene.traverse(
            root,
            TraversalState::new(),
            &mut |node, node_idx, state| match node {
                SceneNode::Rect(n) if n.visible => {
                    let layer = n.z.max(0) as u32;
                    let bucket = layers
                        .entry(layer)
                        .or_insert_with(|| (Vec::new(), Vec::new(), Vec::new()));
                    let x = n.x + state.offset_x;
                    let y = n.y + state.offset_y;
                    let in_window = x < screen_w && y < screen_h && x + n.w > 0.0 && y + n.h > 0.0;
                    let in_clip = state.clip.map_or(true, |[cx, cy, cx2, cy2]| {
                        x < cx2 && y < cy2 && x + n.w > cx && y + n.h > cy
                    });
                    if in_window && in_clip {
                        bucket.1.push(RectCall {
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
                    let layer = n.z.max(0) as u32;
                    let bucket = layers
                        .entry(layer)
                        .or_insert_with(|| (Vec::new(), Vec::new(), Vec::new()));
                    let x = n.x + state.offset_x;
                    let y = n.y + state.offset_y;
                    let in_window = x < screen_w && y < screen_h && x + n.w > 0.0 && y + n.h > 0.0;
                    let in_clip = state.clip.map_or(true, |[cx, cy, cx2, cy2]| {
                        x < cx2 && y < cy2 && x + n.w > cx && y + n.h > cy
                    });
                    if in_window && in_clip {
                        bucket.0.push(ShadowCall {
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
                    let layer = n.z.max(0) as u32;
                    let bucket = layers
                        .entry(layer)
                        .or_insert_with(|| (Vec::new(), Vec::new(), Vec::new()));
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
                        bucket.2.push(TextCall {
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
                            layer,
                            selection_start: n.selection_start,
                            selection_end: n.selection_end,
                            selection_color: apply_opacity(n.selection_color, state.opacity),
                            underlines: n
                                .underlines
                                .iter()
                                .map(|d| crate::nodes::TextDecoration {
                                    start: d.start,
                                    end: d.end,
                                    color: apply_opacity(d.color, state.opacity),
                                    thickness: d.thickness,
                                })
                                .collect(),
                            strikethroughs: n
                                .strikethroughs
                                .iter()
                                .map(|d| crate::nodes::TextDecoration {
                                    start: d.start,
                                    end: d.end,
                                    color: apply_opacity(d.color, state.opacity),
                                    thickness: d.thickness,
                                })
                                .collect(),
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

        // upload all rects and shadows
        for (_, (shadow_calls, rect_calls, _)) in &layers {
            for c in rect_calls {
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
            for c in shadow_calls {
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
                    slot, c.x, c.y, c.w, c.h, c.color, c.blur, c.radius, c.offset_x, c.offset_y,
                    scale,
                );
            }
        }

        for slot in self.sel_slots_last_frame.drain(..) {
            self.rect_pipeline.clear_slot(slot as usize);
            self.rect_alloc.free(slot);
        }

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

        for (_, node) in &mut scene.nodes {
            if let SceneNode::Shadow(n) = node {
                if !n.visible && n.slot != u32::MAX {
                    self.shadow_pipeline.clear_slot(n.slot as usize);
                }
            }
        }

        // submit all text and compute selection/decoration rects
        self.text_pipeline.begin_frame();
        let mut text_call_index = 0usize;
        let mut layer_text_ranges: std::collections::BTreeMap<u32, (usize, usize)> =
            std::collections::BTreeMap::new();

        for (&layer, (_, _, text_calls)) in &layers {
            let range_start = text_call_index;
            for call in text_calls {
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
                text_call_index += 1;
            }
            layer_text_ranges.insert(layer, (range_start, text_call_index));
        }

        // compute selection/decoration rects for all text calls, bucketed by layer
        let mut layer_sel_slots: std::collections::BTreeMap<u32, Vec<u32>> =
            std::collections::BTreeMap::new();
        let all_text_calls: Vec<&TextCall> =
            layers.values().flat_map(|(_, _, tc)| tc.iter()).collect();
        for (idx, call) in all_text_calls.iter().enumerate() {
            let sel_slots = layer_sel_slots.entry(call.layer).or_default();
            if let (Some(sel_start), Some(sel_end)) = (call.selection_start, call.selection_end) {
                let sel_rects = self
                    .text_pipeline
                    .compute_selection_rects(idx, sel_start, sel_end, 0.0, 0.0, scale);
                for (rx, ry, rw, rh) in sel_rects {
                    let in_clip = call.clip.map_or(true, |[cx, cy, cx2, cy2]| {
                        rx < cx2 && ry < cy2 && rx + rw > cx && ry + rh > cy
                    });
                    if !in_clip {
                        continue;
                    }
                    let slot = self.rect_alloc.alloc();
                    self.rect_pipeline.ensure_slot(slot as usize);
                    self.rect_pipeline.write_slot(
                        slot as usize,
                        rx,
                        ry,
                        rw,
                        rh,
                        call.selection_color,
                        0.0,
                        [0.0; 4],
                        [0.0; 4],
                        call.clip,
                        scale,
                    );
                    sel_slots.push(slot);
                    self.sel_slots_this_frame.push(slot);
                }
            }
            for dec in &call.underlines {
                let rects = self.text_pipeline.compute_decoration_rects(
                    idx,
                    dec.start,
                    dec.end,
                    dec.thickness,
                    crate::pipelines::text::DecorationKind::Underline,
                    0.0,
                    0.0,
                    scale,
                );
                for (rx, ry, rw, rh) in rects {
                    let in_clip = call.clip.map_or(true, |[cx, cy, cx2, cy2]| {
                        rx < cx2 && ry < cy2 && rx + rw > cx && ry + rh > cy
                    });
                    if !in_clip {
                        continue;
                    }
                    let slot = self.rect_alloc.alloc();
                    self.rect_pipeline.ensure_slot(slot as usize);
                    self.rect_pipeline.write_slot(
                        slot as usize,
                        rx,
                        ry,
                        rw,
                        rh,
                        dec.color,
                        0.0,
                        [0.0; 4],
                        [0.0; 4],
                        call.clip,
                        scale,
                    );
                    sel_slots.push(slot);
                    self.sel_slots_this_frame.push(slot);
                }
            }
            for dec in &call.strikethroughs {
                let rects = self.text_pipeline.compute_decoration_rects(
                    idx,
                    dec.start,
                    dec.end,
                    dec.thickness,
                    crate::pipelines::text::DecorationKind::Strikethrough,
                    0.0,
                    0.0,
                    scale,
                );
                for (rx, ry, rw, rh) in rects {
                    let in_clip = call.clip.map_or(true, |[cx, cy, cx2, cy2]| {
                        rx < cx2 && ry < cy2 && rx + rw > cx && ry + rh > cy
                    });
                    if !in_clip {
                        continue;
                    }
                    let slot = self.rect_alloc.alloc();
                    self.rect_pipeline.ensure_slot(slot as usize);
                    self.rect_pipeline.write_slot(
                        slot as usize,
                        rx,
                        ry,
                        rw,
                        rh,
                        dec.color,
                        0.0,
                        [0.0; 4],
                        [0.0; 4],
                        call.clip,
                        scale,
                    );
                    sel_slots.push(slot);
                    self.sel_slots_this_frame.push(slot);
                }
            }
        }

        // upload all rect and shadow data once
        self.rect_pipeline.upload(&ctx.device, &ctx.queue);
        self.shadow_pipeline.upload(&ctx.device, &ctx.queue);
        self.text_pipeline
            .prepare(font_system, &ctx.device, &ctx.queue);

        // render pass — draw shadow→rect→text per layer in ascending order
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

            for (&layer, (shadow_calls, rect_calls, _)) in &layers {
                let shadow_slots: Vec<usize> = shadow_calls
                    .iter()
                    .filter_map(|c| match &scene.nodes[c.node_idx] {
                        SceneNode::Shadow(n) if n.slot != u32::MAX => Some(n.slot as usize),
                        _ => None,
                    })
                    .collect();
                let mut rect_slots: Vec<usize> = rect_calls
                    .iter()
                    .filter_map(|c| match &scene.nodes[c.node_idx] {
                        SceneNode::Rect(n) if n.slot != u32::MAX => Some(n.slot as usize),
                        _ => None,
                    })
                    .collect();

                // add selection/decoration slots for this layer
                if let Some(extra) = layer_sel_slots.get(&layer) {
                    rect_slots.extend(extra.iter().map(|&s| s as usize));
                }

                self.shadow_pipeline.draw_slots(&mut pass, &shadow_slots);
                self.rect_pipeline.draw_slots(&mut pass, &rect_slots);

                if let Some(&(range_start, range_end)) = layer_text_ranges.get(&layer) {
                    for idx in range_start..range_end {
                        let (start, count) = self.text_pipeline.instance_range(idx);
                        self.text_pipeline.draw_range(&mut pass, start, count);
                    }
                }
            }
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        self.text_pipeline.trim_atlas();
        self.text_pipeline.end_frame();
        self.stats.rect_uploads = self.rect_pipeline.upload_count;
        // swap selection slot lists for next frame cleanup
        std::mem::swap(
            &mut self.sel_slots_last_frame,
            &mut self.sel_slots_this_frame,
        );
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
