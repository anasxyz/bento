// renderer.rs
//
// The Renderer is the only place that touches GPU state.
// It reads the SceneGraph each frame, assigns stable GPU slots to new nodes,
// uploads only dirty nodes, and issues draw calls.
//
// One Renderer per RenderContext (shared across windows).
// Each call to render() targets a specific Surface.
//
// Frame sequence:
//   1. renderer.render(&ctx, &mut surface, &mut scene, clear_color)
//      a. acquire surface texture
//      b. begin_frame on text pipeline
//      c. walk scene graph — assign slots, upload dirty rects/shadows
//      d. submit visible text nodes
//      e. begin render pass: draw shadows → rects → text
//      f. present

use wgpu;
use crate::allocator::SlotAllocator;
use crate::context::RenderContext;
use crate::pipelines::{rect::RectPipeline, shadow::ShadowPipeline, text::TextPipeline};
use crate::scene::SceneGraph;
use crate::surface::Surface;

pub struct Renderer {
    rect_pipeline:   RectPipeline,
    text_pipeline:   TextPipeline,
    shadow_pipeline: ShadowPipeline,

    rect_alloc:   SlotAllocator,
    shadow_alloc: SlotAllocator,
}

impl Renderer {
    pub fn new(ctx: &RenderContext, surface: &Surface) -> Self {
        let fmt = surface.format;
        let sw  = surface.physical_width()  as f32;
        let sh  = surface.physical_height() as f32;

        Self {
            rect_pipeline:   RectPipeline::new(&ctx.device, fmt, sw, sh),
            shadow_pipeline: ShadowPipeline::new(&ctx.device, &ctx.queue, fmt, sw, sh),
            text_pipeline:   TextPipeline::new(&ctx.device, &ctx.queue, fmt,
                                               surface.width, surface.height, surface.scale),
            rect_alloc:   SlotAllocator::new(),
            shadow_alloc: SlotAllocator::new(),
        }
    }

    /// Call when the surface is resized or rescaled.
    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface) {
        let sw = surface.physical_width()  as f32;
        let sh = surface.physical_height() as f32;
        self.rect_pipeline.resize(sw, sh);
        self.shadow_pipeline.resize(&ctx.queue, sw, sh);
        self.text_pipeline.resize(surface.width, surface.height, surface.scale);
    }

    /// Fully invalidate all GPU state — call when the SceneGraph is rebuilt
    /// from scratch (e.g. after removing all elements).
    pub fn invalidate(&mut self) {
        self.rect_pipeline.invalidate();
        self.shadow_pipeline.invalidate();
    }

    /// Render one frame to the given surface.
    pub fn render(
        &mut self,
        ctx:         &RenderContext,
        surface:     &mut Surface,
        scene:       &mut SceneGraph,
        clear_color: [f32; 4],
    ) {
        // ── acquire frame ─────────────────────────────────────────────────────
        let frame = match surface.surface.get_current_texture() {
            Ok(f) => f,
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                surface.surface.configure(&ctx.device, &surface.config);
                return;
            }
            Err(_) => return,
        };
        let frame_view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = ctx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some("bento_wgpu frame") }
        );

        // ── sync dirty nodes to GPU ───────────────────────────────────────────
        let scale = surface.scale;
        self.sync_rects(scene, scale);
        self.sync_shadows(scene, scale);

        // ── text: begin frame + submit all visible nodes ──────────────────────
        self.text_pipeline.begin_frame();
        for (_, node) in &scene.texts {
            if node.visible && !node.content.is_empty() {
                self.text_pipeline.submit(
                    node.x, node.y,
                    &node.content, &node.family,
                    node.size, node.weight, node.italic,
                    node.color, node.width, node.clip,
                );
            }
        }

        // ── render pass ───────────────────────────────────────────────────────
        {
            let [r, g, b, a] = clear_color;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bento_wgpu pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: r as f64, g: g as f64,
                            b: b as f64, a: a as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes:  None,
                occlusion_query_set: None,
            });

            // draw order: shadows behind rects, text on top
            self.shadow_pipeline.render(&ctx.device, &ctx.queue, &mut pass);
            self.rect_pipeline.render(&ctx.device, &ctx.queue, &mut pass);
            self.text_pipeline.render(&ctx.device, &ctx.queue, &mut pass);
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        self.text_pipeline.trim_atlas();
    }

    // ── internal sync helpers ─────────────────────────────────────────────────

    fn sync_rects(&mut self, scene: &mut SceneGraph, scale: f32) {
        for (_, node) in &mut scene.rects {
            // assign a stable slot on first encounter
            if node.slot == u32::MAX {
                node.slot = self.rect_alloc.alloc();
                node.dirty = true;
            }
            if !node.dirty { continue; }

            let slot = node.slot as usize;
            self.rect_pipeline.ensure_slot(slot);

            if node.visible {
                self.rect_pipeline.write_slot(
                    slot,
                    node.x, node.y, node.w, node.h,
                    node.color,
                    node.radius,
                    node.border_color,
                    node.border_widths,
                    node.clip,
                    scale,
                );
            } else {
                self.rect_pipeline.clear_slot(slot);
            }
            node.dirty = false;
        }
    }

    fn sync_shadows(&mut self, scene: &mut SceneGraph, scale: f32) {
        for (_, node) in &mut scene.shadows {
            if node.slot == u32::MAX {
                node.slot = self.shadow_alloc.alloc();
                node.dirty = true;
            }
            if !node.dirty { continue; }

            let slot = node.slot as usize;
            self.shadow_pipeline.ensure_slot(slot);

            if node.visible {
                self.shadow_pipeline.write_slot(
                    slot,
                    node.x, node.y, node.w, node.h,
                    node.color,
                    node.blur, node.radius,
                    node.offset_x, node.offset_y,
                    scale,
                );
            } else {
                self.shadow_pipeline.clear_slot(slot);
            }
            node.dirty = false;
        }
    }

    /// Reclaim the GPU slot for a rect node that has been removed from the scene.
    /// Call this after SceneGraph::remove_rect() if you want to immediately reuse
    /// the GPU slot. Otherwise the slot stays zeroed until the allocator wraps.
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

    /// Expose the font system for text measurement outside the render loop.
    pub fn font_system(&mut self) -> &mut glyphon::FontSystem {
        &mut self.text_pipeline.font_system
    }
}
