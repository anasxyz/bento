use crate::allocator::SlotAllocator;
use crate::context::RenderContext;
use crate::pipelines::{rect::RectPipeline, shadow::ShadowPipeline, text::TextPipeline};
use crate::scene::SceneGraph;
use crate::surface::Surface;
use wgpu;

pub struct Renderer {
    rect_pipeline: RectPipeline,
    text_pipeline: TextPipeline,
    shadow_pipeline: ShadowPipeline,

    rect_alloc: SlotAllocator,
    shadow_alloc: SlotAllocator,
}

impl Renderer {
    pub fn new(ctx: &RenderContext, surface: &Surface) -> Self {
        let fmt = surface.format;
        let sw = surface.physical_width() as f32;
        let sh = surface.physical_height() as f32;
        Self {
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
        }
    }

    /// call when the surface is resized or rescaled
    /// resets all gpu slots so everything reuploads with new dimensions
    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface, scene: &mut SceneGraph) {
        let sw = surface.physical_width() as f32;
        let sh = surface.physical_height() as f32;
        self.rect_pipeline.resize(sw, sh);
        self.shadow_pipeline.resize(&ctx.queue, sw, sh);
        self.text_pipeline
            .resize(surface.width, surface.height, surface.scale);

        // reset all gpu slots so write_slot reuploads everything with new scale
        for (_, node) in &mut scene.rects {
            node.slot = u32::MAX;
        }
        for (_, node) in &mut scene.shadows {
            node.slot = u32::MAX;
        }
    }

    /// fully invalidate
    /// resets all gpu slots so everything reuploads next frame:w
    pub fn invalidate(&mut self, scene: &mut SceneGraph) {
        self.rect_pipeline.invalidate();
        self.shadow_pipeline.invalidate();
        for (_, node) in &mut scene.rects {
            node.slot = u32::MAX;
        }
        for (_, node) in &mut scene.shadows {
            node.slot = u32::MAX;
        }
    }

    /// render one frame to the given surface
    /// only uploads nodes that changed since the last call
    pub fn render(
        &mut self,
        ctx: &mut RenderContext,
        surface: &mut Surface,
        scene: &mut SceneGraph,
        clear_color: [f32; 4],
    ) {
        // acquire frame 
        let frame = match surface.surface.get_current_texture() {
            Ok(f) => f,
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                surface.surface.configure(&ctx.device, &surface.config);
                return;
            }
            Err(_) => return,
        };
        let frame_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("bento_render frame"),
            });

        let scale = surface.scale;

        // sync nodes
        // iterate all, write_slot does byte comparison 
        self.sync_rects(scene, scale);
        self.sync_shadows(scene, scale);

        // text: submit all visible nodes 
        self.text_pipeline.begin_frame();
        let text_nodes: Vec<_> = scene
            .texts
            .iter()
            .filter(|(_, n)| n.visible && !n.content.is_empty())
            .map(|(_, n)| {
                (
                    n.x,
                    n.y,
                    n.content.clone(),
                    n.family.clone(),
                    n.size,
                    n.weight,
                    n.italic,
                    n.color,
                    n.width,
                    n.clip,
                )
            })
            .collect();
        for (x, y, content, family, size, weight, italic, color, width, clip) in text_nodes {
            self.text_pipeline.submit(
                &mut ctx.font_system,
                x,
                y,
                &content,
                &family,
                size,
                weight,
                italic,
                color,
                width,
                clip,
            );
        }

        // render pass 
        {
            let [r, g, b, a] = clear_color;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bento_render pass"),
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

            // draw order: shadows, then rects, then text
            self.shadow_pipeline
                .render(&ctx.device, &ctx.queue, &mut pass);
            self.rect_pipeline
                .render(&ctx.device, &ctx.queue, &mut pass);
            self.text_pipeline
                .render(&mut ctx.font_system, &ctx.device, &ctx.queue, &mut pass);
        }

        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        self.text_pipeline.trim_atlas();
    }

    // internal 

    fn sync_rects(&mut self, scene: &mut SceneGraph, scale: f32) {
        for (_, node) in &mut scene.rects {
            if node.slot == u32::MAX {
                node.slot = self.rect_alloc.alloc();
            }
            let slot = node.slot as usize;
            self.rect_pipeline.ensure_slot(slot);

            if node.visible {
                self.rect_pipeline.write_slot(
                    slot,
                    node.x,
                    node.y,
                    node.w,
                    node.h,
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
        }
    }

    fn sync_shadows(&mut self, scene: &mut SceneGraph, scale: f32) {
        for (_, node) in &mut scene.shadows {
            if node.slot == u32::MAX {
                node.slot = self.shadow_alloc.alloc();
            }
            let slot = node.slot as usize;
            self.shadow_pipeline.ensure_slot(slot);

            if node.visible {
                self.shadow_pipeline.write_slot(
                    slot,
                    node.x,
                    node.y,
                    node.w,
                    node.h,
                    node.color,
                    node.blur,
                    node.radius,
                    node.offset_x,
                    node.offset_y,
                    scale,
                );
            } else {
                self.shadow_pipeline.clear_slot(slot);
            }
        }
    }

    /// reclaim the gpu slot for a removed rect node so it can be reused
    /// call after SceneGraph::remove_rect()
    pub fn free_rect_slot(&mut self, slot: u32) {
        if slot != u32::MAX {
            self.rect_pipeline.clear_slot(slot as usize);
            self.rect_alloc.free(slot);
        }
    }

    /// reclaim the gpu slot for a removed shadow node
    pub fn free_shadow_slot(&mut self, slot: u32) {
        if slot != u32::MAX {
            self.shadow_pipeline.clear_slot(slot as usize);
            self.shadow_alloc.free(slot);
        }
    }

    /// for debugging
    /// how many rect slots were actually uploaded to the gpu last frame
    /// should be 0 when nothing changed, 1 when one rect changed
    pub fn rect_uploads(&self) -> u32 {
        self.rect_pipeline.upload_count
    }
}
