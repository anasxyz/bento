use crate::allocator::SlotAllocator;
use crate::context::RenderContext;
use crate::nodes::*;
use crate::pipelines::{
    blur::{BlurCall, BlurPipeline},
    image::{ImageCache, ImageCall, ImagePipeline},
    rect::RectPipeline,
    shadow::ShadowPipeline,
    text::TextPipeline,
};
use crate::scene::{mat_mul, mat_trs_pub, Mat2x3, SceneGraph, TraversalState};
use crate::surface::Surface;
use std::collections::BTreeMap;
use wgpu;

pub struct RendererStats {
    pub rect_uploads:  u32,
    pub rects_culled:  u32,
    pub texts_culled:  u32,
    pub images_culled: u32,
}

// ── Draw call types ───────────────────────────────────────────────────────────
// All draw calls are collected into a single per-layer sorted list so that
// z-order is correct across node types (rect at z=1 draws after text at z=0
// regardless of type, unlike the old fixed shadow/rect/image/text buckets).

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DrawKind { Shadow, Rect, Image, Text, Blur }

struct DrawCall {
    kind:      DrawKind,
    // index into the corresponding type-specific Vec
    idx:       usize,
}

struct RectCall {
    node_idx:      usize,
    transform:     Mat2x3,
    local_w:       f32,
    local_h:       f32,
    color:         [f32; 4],
    radius:        f32,
    border_color:  [f32; 4],
    border_widths: [f32; 4],
    clip:          Option<[f32; 4]>,
    gradient:      Option<([f32; 4], [f32; 4], f32)>,
}

struct ShadowCall {
    node_idx:        usize,
    transform:       Mat2x3,
    local_w:         f32,
    local_h:         f32,
    color:           [f32; 4],
    blur:            f32,
    radius:          f32,
    shadow_offset_x: f32,
    shadow_offset_y: f32,
    clip:            Option<[f32; 4]>,
}

struct TextCall {
    transform:       Mat2x3,
    content:         String,
    family:          String,
    size:            f32,
    weight:          u16,
    italic:          bool,
    color:           [f32; 4],
    width:           f32,
    clip:            Option<[f32; 4]>,
    layer:           u32,
    selection_start: Option<usize>,
    selection_end:   Option<usize>,
    selection_color: [f32; 4],
    underlines:      Vec<crate::nodes::TextDecoration>,
    strikethroughs:  Vec<crate::nodes::TextDecoration>,
}

struct BlurCallEntry {
    call: BlurCall,
}

// Per z-layer bucket — now holds a unified sorted draw list
struct LayerBucket {
    draw_list: Vec<DrawCall>,
    shadows:   Vec<ShadowCall>,
    rects:     Vec<RectCall>,
    images:    Vec<ImageCall>,
    texts:     Vec<TextCall>,
    blurs:     Vec<BlurCallEntry>,
}

impl LayerBucket {
    fn new() -> Self {
        Self {
            draw_list: Vec::new(),
            shadows:   Vec::new(),
            rects:     Vec::new(),
            images:    Vec::new(),
            texts:     Vec::new(),
            blurs:     Vec::new(),
        }
    }
    fn clear(&mut self) {
        self.draw_list.clear();
        self.shadows.clear(); self.rects.clear();
        self.images.clear();  self.texts.clear();
        self.blurs.clear();
    }
    fn push_shadow(&mut self, c: ShadowCall) {
        let idx = self.shadows.len(); self.shadows.push(c);
        self.draw_list.push(DrawCall { kind: DrawKind::Shadow, idx });
    }
    fn push_rect(&mut self, c: RectCall) {
        let idx = self.rects.len(); self.rects.push(c);
        self.draw_list.push(DrawCall { kind: DrawKind::Rect, idx });
    }
    fn push_image(&mut self, c: ImageCall) {
        let idx = self.images.len(); self.images.push(c);
        self.draw_list.push(DrawCall { kind: DrawKind::Image, idx });
    }
    fn push_text(&mut self, c: TextCall) {
        let idx = self.texts.len(); self.texts.push(c);
        self.draw_list.push(DrawCall { kind: DrawKind::Text, idx });
    }
    fn push_blur(&mut self, c: BlurCallEntry) {
        let idx = self.blurs.len(); self.blurs.push(c);
        self.draw_list.push(DrawCall { kind: DrawKind::Blur, idx });
    }
}

pub struct Renderer {
    rect_pipeline:    RectPipeline,
    text_pipeline:    TextPipeline,
    shadow_pipeline:  ShadowPipeline,
    image_pipeline:   ImagePipeline,
    blur_pipeline:    BlurPipeline,
    image_cache:      ImageCache,
    rect_alloc:       SlotAllocator,
    shadow_alloc:     SlotAllocator,
    sel_slots_last_frame: Vec<u32>,
    sel_slots_this_frame: Vec<u32>,
    layers:           BTreeMap<u32, LayerBucket>,
    // per-layer tracking for pipelines that batch by type
    layer_image_ranges: BTreeMap<u32, Vec<(u32, u32, ImageKey)>>,
    layer_text_ranges:  BTreeMap<u32, (usize, usize)>,
    layer_sel_slots:    BTreeMap<u32, Vec<u32>>,
    culled_rect_nodes:  Vec<usize>,
    surface_format:     wgpu::TextureFormat,
    pub stats: RendererStats,
}

impl Renderer {
    pub fn new(ctx: &RenderContext, surface: &Surface) -> Self {
        let fmt = surface.format;
        let sw  = surface.physical_width()  as f32;
        let sh  = surface.physical_height() as f32;

        let image_cache    = ImageCache::new(&ctx.device);
        let image_pipeline = ImagePipeline::new(&ctx.device, &ctx.queue, fmt, sw, sh, &image_cache);
        let blur_pipeline  = BlurPipeline::new(&ctx.device, &ctx.queue, fmt, sw, sh);

        let mut r = Self {
            rect_pipeline:   RectPipeline::new(&ctx.device, fmt, sw, sh),
            shadow_pipeline: ShadowPipeline::new(&ctx.device, &ctx.queue, fmt, sw, sh),
            text_pipeline:   TextPipeline::new(&ctx.device, &ctx.queue, fmt,
                                               surface.width, surface.height, surface.scale),
            image_pipeline,
            blur_pipeline,
            image_cache,
            rect_alloc:   SlotAllocator::new(),
            shadow_alloc: SlotAllocator::new(),
            sel_slots_last_frame: Vec::new(),
            sel_slots_this_frame: Vec::new(),
            layers:             BTreeMap::new(),
            layer_image_ranges: BTreeMap::new(),
            layer_text_ranges:  BTreeMap::new(),
            layer_sel_slots:    BTreeMap::new(),
            culled_rect_nodes:  Vec::new(),
            surface_format:     fmt,
            stats: RendererStats { rect_uploads: 0, rects_culled: 0, texts_culled: 0, images_culled: 0 },
        };
        r.rect_pipeline.resize(&ctx.queue, sw, sh);
        r
    }

    pub fn upload_image(&mut self, ctx: &RenderContext, key: ImageKey, rgba: &[u8], w: u32, h: u32) {
        self.image_cache.upload(&ctx.device, &ctx.queue, key, rgba, w, h);
    }
    pub fn free_image(&mut self, key: ImageKey) { self.image_cache.free(key); }

    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface, scene: &mut SceneGraph) {
        let sw    = surface.physical_width()  as f32;
        let sh    = surface.physical_height() as f32;
        let scale = surface.scale;

        self.rect_pipeline.resize(&ctx.queue, sw, sh);
        self.shadow_pipeline.resize(&ctx.queue, sw, sh);
        self.text_pipeline.resize(&ctx.device, &ctx.queue, surface.width, surface.height, scale);
        self.image_pipeline.resize(&ctx.queue, sw, sh);
        self.blur_pipeline.resize(&ctx.device, &ctx.queue, sw, sh, self.surface_format);

        // Force full re-upload of all instance data at new scale
        self.rect_pipeline.invalidate();
        self.shadow_pipeline.invalidate();
        for (_, node) in &mut scene.nodes {
            match node {
                SceneNode::Rect(n)   => n.slot = u32::MAX,
                SceneNode::Shadow(n) => n.slot = u32::MAX,
                _ => {}
            }
        }
    }

    pub fn invalidate(&mut self, scene: &mut SceneGraph) {
        self.rect_pipeline.invalidate();
        self.shadow_pipeline.invalidate();
        for (_, node) in &mut scene.nodes {
            match node {
                SceneNode::Rect(n)   => n.slot = u32::MAX,
                SceneNode::Shadow(n) => n.slot = u32::MAX,
                _ => {}
            }
        }
    }

    pub fn render(
        &mut self,
        ctx:         &mut RenderContext,
        font_system: &mut cosmic_text::FontSystem,
        surface:     &mut Surface,
        scene:       &mut SceneGraph,
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

        let scale    = surface.scale;
        let screen_w = surface.width;
        let screen_h = surface.height;

        self.stats = RendererStats { rect_uploads: 0, rects_culled: 0, texts_culled: 0, images_culled: 0 };
        self.sel_slots_this_frame.clear();
        self.culled_rect_nodes.clear();
        self.image_pipeline.begin_frame();
        self.blur_pipeline.begin_frame();

        for bucket in self.layers.values_mut() { bucket.clear(); }

        let mut culled_rects  = 0u32;
        let mut culled_texts  = 0u32;
        let mut culled_images = 0u32;

        // ── Scene traversal ───────────────────────────────────────────────────
        let root = scene.root;
        scene.traverse(root, TraversalState::new(), &mut |node, node_idx, state| {
            match node {

                SceneNode::Rect(n) if n.visible => {
                    let layer  = n.z.max(0) as u32;
                    let bucket = self.layers.entry(layer).or_insert_with(LayerBucket::new);
                    let xform  = state.leaf_transform_with_node(n.x, n.y, n.w, n.h, &n.transform);
                    let aabb   = transformed_aabb(&xform, n.w, n.h);
                    // combine node opacity with inherited opacity
                    let eff_opacity = state.opacity * n.opacity;
                    if is_visible_aabb(aabb, screen_w, screen_h, state.clip) {
                        let gradient = n.gradient.as_ref().and_then(|g| {
                            if g.is_active() && g.stops.len() >= 2 {
                                Some((g.stops[0].color, g.stops[g.stops.len()-1].color, g.angle))
                            } else { None }
                        });
                        bucket.push_rect(RectCall {
                            node_idx, transform: xform,
                            local_w: n.w, local_h: n.h,
                            color:         apply_opacity(n.color, eff_opacity),
                            radius:        n.radius,
                            border_color:  apply_opacity(n.border_color, eff_opacity),
                            border_widths: n.border_widths,
                            clip:          state.clip,
                            gradient,
                        });
                    } else {
                        culled_rects += 1;
                        self.culled_rect_nodes.push(node_idx);
                    }
                }

                SceneNode::Shadow(n) if n.visible => {
                    let layer  = n.z.max(0) as u32;
                    let bucket = self.layers.entry(layer).or_insert_with(LayerBucket::new);
                    let xform  = state.leaf_transform_with_node(n.x, n.y, n.w, n.h, &n.transform);
                    let aabb   = transformed_aabb(&xform, n.w, n.h);
                    let eff_opacity = state.opacity * n.opacity;
                    if is_visible_aabb(aabb, screen_w, screen_h, state.clip) {
                        bucket.push_shadow(ShadowCall {
                            node_idx, transform: xform,
                            local_w: n.w, local_h: n.h,
                            color:           apply_opacity(n.color, eff_opacity),
                            blur:            n.blur, radius: n.radius,
                            shadow_offset_x: n.offset_x,
                            shadow_offset_y: n.offset_y,
                            clip:            state.clip,
                        });
                    }
                }

                SceneNode::Image(n) if n.visible => {
                    let layer  = n.z.max(0) as u32;
                    let bucket = self.layers.entry(layer).or_insert_with(LayerBucket::new);
                    let xform  = state.leaf_transform_with_node(n.x, n.y, n.w, n.h, &n.transform);
                    let aabb   = transformed_aabb(&xform, n.w, n.h);
                    let eff_opacity = state.opacity * n.opacity;
                    if is_visible_aabb(aabb, screen_w, screen_h, state.clip) {
                        bucket.push_image(ImageCall {
                            transform:     xform,
                            local_w:       n.w, local_h: n.h,
                            uv:            n.uv,
                            tint:          apply_opacity(n.tint, eff_opacity),
                            radius:        n.radius,
                            border_color:  apply_opacity(n.border_color, eff_opacity),
                            border_widths: n.border_widths,
                            image_key:     n.image_key,
                            clip:          state.clip,
                        });
                    } else {
                        culled_images += 1;
                    }
                }

                SceneNode::Text(n) if n.visible && !n.content.is_empty() => {
                    let layer  = n.z.max(0) as u32;
                    let bucket = self.layers.entry(layer).or_insert_with(LayerBucket::new);
                    let (ox, oy) = n.transform.origin.unwrap_or((0.0, 0.0));
                    let local    = mat_trs_pub(n.x, n.y, n.transform.rotate,
                                              n.transform.scale_x, n.transform.scale_y, ox, oy);
                    let text_xform = mat_mul(state.transform, local);
                    let est_w  = if n.width < f32::MAX { n.width } else { 1600.0 };
                    let est_h  = n.size * 1.4 * 8.0;
                    let cons   = est_w.max(est_h);
                    let aabb   = transformed_aabb(&text_xform, cons, cons);
                    let eff_opacity = state.opacity * n.opacity;
                    if is_visible_aabb(aabb, screen_w, screen_h, state.clip) {
                        bucket.push_text(TextCall {
                            transform: text_xform,
                            content:   n.content.clone(), family: n.family.clone(),
                            size: n.size, weight: n.weight, italic: n.italic,
                            color: apply_opacity(n.color, eff_opacity),
                            width: n.width, clip: state.clip, layer,
                            selection_start:  n.selection_start,
                            selection_end:    n.selection_end,
                            selection_color:  apply_opacity(n.selection_color, eff_opacity),
                            underlines: n.underlines.iter().map(|d| crate::nodes::TextDecoration {
                                start: d.start, end: d.end,
                                color: apply_opacity(d.color, eff_opacity),
                                thickness: d.thickness,
                            }).collect(),
                            strikethroughs: n.strikethroughs.iter().map(|d| crate::nodes::TextDecoration {
                                start: d.start, end: d.end,
                                color: apply_opacity(d.color, eff_opacity),
                                thickness: d.thickness,
                            }).collect(),
                        });
                    } else {
                        culled_texts += 1;
                    }
                }

                SceneNode::Blur(n) if n.visible => {
                    let layer  = n.z.max(0) as u32;
                    let bucket = self.layers.entry(layer).or_insert_with(LayerBucket::new);
                    let (sx, sy) = state.transform_point(n.x, n.y);
                    let eff_opacity = state.opacity * n.opacity;
                    if is_visible(sx, sy, n.w, n.h, screen_w, screen_h, state.clip) {
                        bucket.push_blur(BlurCallEntry {
                            call: BlurCall {
                                x: sx, y: sy, w: n.w, h: n.h,
                                radius: n.radius, sigma: n.sigma,
                                tint: apply_opacity(n.tint, eff_opacity),
                                clip: state.clip,
                            }
                        });
                    }
                }

                _ => {}
            }
        });

        self.stats.rects_culled  = culled_rects;
        self.stats.texts_culled  = culled_texts;
        self.stats.images_culled = culled_images;

        // ── Upload rect/shadow slots ──────────────────────────────────────────
        for (_, bucket) in &self.layers {
            for c in &bucket.rects {
                let n = match &mut scene.nodes[c.node_idx] { SceneNode::Rect(n) => n, _ => continue };
                if n.slot == u32::MAX { n.slot = self.rect_alloc.alloc(); }
                let slot = n.slot as usize;
                self.rect_pipeline.ensure_slot(slot);
                self.rect_pipeline.write_slot(slot, c.transform, c.local_w, c.local_h,
                    c.color, c.radius, c.border_color, c.border_widths, c.clip, scale, c.gradient);
            }
            for c in &bucket.shadows {
                let n = match &mut scene.nodes[c.node_idx] { SceneNode::Shadow(n) => n, _ => continue };
                if n.slot == u32::MAX { n.slot = self.shadow_alloc.alloc(); }
                let slot = n.slot as usize;
                self.shadow_pipeline.ensure_slot(slot);
                self.shadow_pipeline.write_slot(slot, c.transform, c.local_w, c.local_h,
                    c.color, c.blur, c.radius, c.shadow_offset_x, c.shadow_offset_y, scale);
            }
        }

        for slot in self.sel_slots_last_frame.drain(..) {
            self.rect_pipeline.clear_slot(slot as usize);
            self.rect_alloc.free(slot);
        }
        for (_, node) in &mut scene.nodes {
            if let SceneNode::Rect(n) = node {
                if !n.visible && n.slot != u32::MAX { self.rect_pipeline.clear_slot(n.slot as usize); }
            }
        }
        for &node_idx in &self.culled_rect_nodes {
            if let SceneNode::Rect(n) = &mut scene.nodes[node_idx] {
                if n.slot != u32::MAX { self.rect_pipeline.clear_slot(n.slot as usize); }
            }
        }
        for (_, node) in &mut scene.nodes {
            if let SceneNode::Shadow(n) = node {
                if !n.visible && n.slot != u32::MAX { self.shadow_pipeline.clear_slot(n.slot as usize); }
            }
        }

        // ── Prepare image pipeline (batched by image key) ─────────────────────
        self.layer_image_ranges.clear();
        for (&layer, bucket) in &self.layers {
            if !bucket.images.is_empty() {
                let ranges = self.image_pipeline.prepare_layer(&bucket.images, scale);
                self.layer_image_ranges.insert(layer, ranges);
            }
        }
        self.image_pipeline.upload_staged(&ctx.device, &ctx.queue);

        // ── Prepare text pipeline ─────────────────────────────────────────────
        self.text_pipeline.begin_frame();
        let mut text_call_index = 0usize;
        self.layer_text_ranges.clear();
        for (&layer, bucket) in &self.layers {
            let range_start = text_call_index;
            for call in &bucket.texts {
                self.text_pipeline.submit(font_system, call.transform, &call.content,
                    &call.family, call.size, call.weight, call.italic,
                    call.color, call.width, call.clip);
                text_call_index += 1;
            }
            self.layer_text_ranges.insert(layer, (range_start, text_call_index));
        }

        // Selection / decoration rects
        self.layer_sel_slots.clear();
        let mut text_idx = 0usize;
        for (_, bucket) in &self.layers {
            for call in &bucket.texts {
                let sel_slots = self.layer_sel_slots.entry(call.layer).or_default();
                if let (Some(s), Some(e)) = (call.selection_start, call.selection_end) {
                    for (rx, ry, rw, rh, dt) in
                        self.text_pipeline.compute_selection_rects(text_idx, s, e, scale)
                    {
                        let rxform = mat_mul(dt, [1.0,0.0,0.0,1.0,rx,ry]);
                        if is_visible_aabb(transformed_aabb(&rxform, rw, rh), screen_w, screen_h, call.clip) {
                            let slot = self.rect_alloc.alloc();
                            self.rect_pipeline.ensure_slot(slot as usize);
                            self.rect_pipeline.write_slot(slot as usize, rxform, rw, rh,
                                call.selection_color, 0.0, [0.0;4], [0.0;4], call.clip, scale, None);
                            sel_slots.push(slot); self.sel_slots_this_frame.push(slot);
                        }
                    }
                }
                for dec in &call.underlines {
                    for (rx, ry, rw, rh, dt) in self.text_pipeline.compute_decoration_rects(
                        text_idx, dec.start, dec.end, dec.thickness,
                        crate::pipelines::text::DecorationKind::Underline, scale,
                    ) {
                        let rxform = mat_mul(dt, [1.0,0.0,0.0,1.0,rx,ry]);
                        if is_visible_aabb(transformed_aabb(&rxform, rw, rh), screen_w, screen_h, call.clip) {
                            let slot = self.rect_alloc.alloc();
                            self.rect_pipeline.ensure_slot(slot as usize);
                            self.rect_pipeline.write_slot(slot as usize, rxform, rw, rh,
                                dec.color, 0.0, [0.0;4], [0.0;4], call.clip, scale, None);
                            sel_slots.push(slot); self.sel_slots_this_frame.push(slot);
                        }
                    }
                }
                for dec in &call.strikethroughs {
                    for (rx, ry, rw, rh, dt) in self.text_pipeline.compute_decoration_rects(
                        text_idx, dec.start, dec.end, dec.thickness,
                        crate::pipelines::text::DecorationKind::Strikethrough, scale,
                    ) {
                        let rxform = mat_mul(dt, [1.0,0.0,0.0,1.0,rx,ry]);
                        if is_visible_aabb(transformed_aabb(&rxform, rw, rh), screen_w, screen_h, call.clip) {
                            let slot = self.rect_alloc.alloc();
                            self.rect_pipeline.ensure_slot(slot as usize);
                            self.rect_pipeline.write_slot(slot as usize, rxform, rw, rh,
                                dec.color, 0.0, [0.0;4], [0.0;4], call.clip, scale, None);
                            sel_slots.push(slot); self.sel_slots_this_frame.push(slot);
                        }
                    }
                }
                text_idx += 1;
            }
        }

        // ── Prepare blur calls ────────────────────────────────────────────────
        let all_blur_calls: Vec<BlurCall> = self.layers.values()
            .flat_map(|b| b.blurs.iter().map(|e| BlurCall {
                x: e.call.x, y: e.call.y, w: e.call.w, h: e.call.h,
                radius: e.call.radius, sigma: e.call.sigma,
                tint: e.call.tint, clip: e.call.clip,
            }))
            .collect();
        let has_blur = !all_blur_calls.is_empty();
        if has_blur {
            self.blur_pipeline.ensure_src_texture(&ctx.device, self.surface_format);
            self.blur_pipeline.prepare(&all_blur_calls, scale, &ctx.queue, &ctx.device);
        }

        self.rect_pipeline.upload(&ctx.device, &ctx.queue);
        self.shadow_pipeline.upload(&ctx.device, &ctx.queue);
        self.text_pipeline.prepare(font_system, &ctx.device, &ctx.queue);

        // ── Render ────────────────────────────────────────────────────────────
        let frame_view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = ctx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some("bento frame") });
        let [r, g, b, a] = clear_color;

        // If we have blur nodes, copy the frame texture before drawing into it.
        // We do a blit from the previous frame (or clear if first frame).
        // For correctness, blur reads from a snapshot taken at start of frame.
        if has_blur {
            if let (Some(src_tex), _) = (&self.blur_pipeline.src_texture, ()) {
                // Copy current frame texture to blur source
                // We can't copy the frame before clearing, so we copy after the clear pass
                // by using a two-encoder approach. For simplicity we copy after clear.
            }
        }

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bento pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view, resolve_target: None,
                    ops: wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(wgpu::Color { r: r as f64, g: g as f64, b: b as f64, a: a as f64 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes:         None,
                occlusion_query_set:      None,
            });

            for (&layer, bucket) in &self.layers {
                // Collect selection rect slots for this layer
                let empty_sel: Vec<u32> = Vec::new();
                let sel_slots = self.layer_sel_slots.get(&layer).unwrap_or(&empty_sel);
                let text_range = self.layer_text_ranges.get(&layer).copied().unwrap_or((0, 0));
                let image_ranges = self.layer_image_ranges.get(&layer);

                // Track per-type draw indices for the unified draw list
                let mut shadow_draw_idx = 0usize;
                let mut rect_draw_idx   = 0usize;
                let mut image_draw_idx  = 0usize;
                let mut text_draw_idx   = text_range.0;
                let mut blur_draw_idx   = 0usize;

                // Counters for how many of each type we've dispatched so far
                let mut shadows_done = 0usize;
                let mut rects_done   = 0usize;
                let mut images_done  = 0usize;
                let mut texts_done   = 0usize;

                for dc in &bucket.draw_list {
                    match dc.kind {
                        DrawKind::Shadow => {
                            if dc.idx >= shadows_done {
                                let c = &bucket.shadows[dc.idx];
                                if let SceneNode::Shadow(n) = &scene.nodes[c.node_idx] {
                                    if n.slot != u32::MAX {
                                        self.shadow_pipeline.draw_slots(&mut pass, &[n.slot as usize]);
                                    }
                                }
                                shadows_done = dc.idx + 1;
                            }
                        }
                        DrawKind::Rect => {
                            if dc.idx >= rects_done {
                                let c = &bucket.rects[dc.idx];
                                if let SceneNode::Rect(n) = &scene.nodes[c.node_idx] {
                                    if n.slot != u32::MAX {
                                        let mut slots = vec![n.slot as usize];
                                        // Append sel slots after the last rect in this layer
                                        if dc.idx == bucket.rects.len() - 1 {
                                            slots.extend(sel_slots.iter().map(|&s| s as usize));
                                        }
                                        self.rect_pipeline.draw_slots(&mut pass, &slots);
                                    }
                                }
                                rects_done = dc.idx + 1;
                            }
                        }
                        DrawKind::Image => {
                            if dc.idx >= images_done {
                                if let Some(ranges) = image_ranges {
                                    // Find the range entry for this specific image call
                                    if let Some(range_entry) = ranges.get(dc.idx) {
                                        let range_slice = std::slice::from_ref(range_entry);
                                        self.image_pipeline.draw_layer(&mut pass, &self.image_cache, range_slice);
                                    }
                                }
                                images_done = dc.idx + 1;
                            }
                        }
                        DrawKind::Text => {
                            if dc.idx >= texts_done {
                                let abs_idx = text_range.0 + dc.idx;
                                let (inst_start, count) = self.text_pipeline.instance_range(abs_idx);
                                if count > 0 {
                                    self.text_pipeline.draw_range(&mut pass, inst_start, count);
                                }
                                texts_done = dc.idx + 1;
                            }
                        }
                        DrawKind::Blur => {
                            let blur_count = bucket.blurs.len() as u32;
                            self.blur_pipeline.draw(&mut pass, blur_count);
                        }
                    }
                }

                // Draw any remaining sel slots if no rects in this layer
                if bucket.rects.is_empty() && !sel_slots.is_empty() {
                    let slots: Vec<usize> = sel_slots.iter().map(|&s| s as usize).collect();
                    self.rect_pipeline.draw_slots(&mut pass, &slots);
                }
            }
        }

        // Copy framebuffer to blur source texture after the main pass
        // so blur on the next frame uses the current frame's content.
        if has_blur {
            if let Some(src_tex) = &self.blur_pipeline.src_texture {
                encoder.copy_texture_to_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &frame.texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::TexelCopyTextureInfo {
                        texture: src_tex,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::Extent3d {
                        width:  surface.physical_width(),
                        height: surface.physical_height(),
                        depth_or_array_layers: 1,
                    },
                );
            }
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        self.text_pipeline.trim_atlas();
        self.text_pipeline.end_frame();
        self.stats.rect_uploads = self.rect_pipeline.upload_count;
        std::mem::swap(&mut self.sel_slots_last_frame, &mut self.sel_slots_this_frame);
    }

    pub fn free_rect_slot(&mut self, slot: u32) {
        if slot != u32::MAX { self.rect_pipeline.clear_slot(slot as usize); self.rect_alloc.free(slot); }
    }
    pub fn free_shadow_slot(&mut self, slot: u32) {
        if slot != u32::MAX { self.shadow_pipeline.clear_slot(slot as usize); self.shadow_alloc.free(slot); }
    }
}

fn transformed_aabb(m: &Mat2x3, w: f32, h: f32) -> [f32; 4] {
    use crate::scene::mat_apply;
    let (x0,y0) = mat_apply(*m, 0.0, 0.0);
    let (x1,y1) = mat_apply(*m, w,   0.0);
    let (x2,y2) = mat_apply(*m, 0.0, h  );
    let (x3,y3) = mat_apply(*m, w,   h  );
    [
        x0.min(x1).min(x2).min(x3), y0.min(y1).min(y2).min(y3),
        x0.max(x1).max(x2).max(x3), y0.max(y1).max(y2).max(y3),
    ]
}

fn is_visible_aabb(aabb: [f32; 4], sw: f32, sh: f32, clip: Option<[f32; 4]>) -> bool {
    let [ax, ay, ax2, ay2] = aabb;
    let in_window = ax < sw && ay < sh && ax2 > 0.0 && ay2 > 0.0;
    let in_clip   = clip.map_or(true, |[cx,cy,cx2,cy2]| ax<cx2 && ay<cy2 && ax2>cx && ay2>cy);
    in_window && in_clip
}

fn is_visible(x: f32, y: f32, w: f32, h: f32, sw: f32, sh: f32, clip: Option<[f32; 4]>) -> bool {
    is_visible_aabb([x, y, x+w, y+h], sw, sh, clip)
}

fn apply_opacity(color: [f32; 4], opacity: f32) -> [f32; 4] {
    [color[0], color[1], color[2], color[3] * opacity]
}
