use crate::{
    DrawCommand, DrawList,
    context::RenderContext,
    pipelines::{
        image::{ImageInstance, ImagePipeline},
        rect::{RectInstance, RectPipeline},
        text::{TextPipeline, TextSpec},
    },
    surface::Surface,
};
use wgpu;

pub fn transform(rotate: f32, scale_x: f32, scale_y: f32) -> [f32; 4] {
    let cos = rotate.cos();
    let sin = rotate.sin();
    [cos * scale_x, sin * scale_x, -sin * scale_y, cos * scale_y]
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
        draw_list: &DrawList,
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

        // process all commands — upload data to GPU
        let mut text_specs: Vec<(u64, TextSpec)> = Vec::new();
        let mut culled_text: std::collections::HashSet<u64> = std::collections::HashSet::new();

        for cmd in &draw_list.commands {
            match cmd {
                DrawCommand::Rect(id, r) => {
                    // cull
                    if r.x + r.w < 0.0
                        || r.x > surface.width
                        || r.y + r.h < 0.0
                        || r.y > surface.height
                    {
                        continue;
                    }

                    let slot = self.rect.get_or_alloc_slot(*id);
                    self.rect.write_slot(
                        slot,
                        RectInstance {
                            pos_size: [r.x, r.y, r.w, r.h],
                            color: [r.color[0], r.color[1], r.color[2], r.color[3] * r.opacity],
                            radii: r.radii,
                            border_color: r.border_color,
                            border_widths: r.border_widths,
                            transform: transform(r.rotate, r.scale_x, r.scale_y),
                            clip: scale_clip(r.clip, surface.scale),
                        },
                    );
                }
                DrawCommand::Text(id, t) => {
                    if t.x + t.w < 0.0
                        || t.x > surface.width
                        || t.y + t.h < 0.0
                        || t.y > surface.height
                    {
                        culled_text.insert(*id);
                        continue;
                    }
                    text_specs.push((
                        *id,
                        TextSpec {
                            text: t.text.clone(),
                            x: t.x,
                            y: t.y,
                            size: t.size,
                            color: t.color,
                            rotate: t.rotate,
                            scale_x: t.scale_x,
                            scale_y: t.scale_y,
                            weight: t.weight,
                            italic: t.italic,
                            font_family: t.font_family.clone(),
                            max_width: t.max_width,
                            line_height: t.line_height,
                            letter_spacing: t.letter_spacing,
                            align: t.align.clone(),
                            opacity: t.opacity,
                            clip: t.clip,
                            color_ranges: t.color_ranges.clone(),
                            background_ranges: t.background_ranges.clone(),
                            underline_ranges: t.underline_ranges.clone(),
                            strikethrough_ranges: t.strikethrough_ranges.clone(),
                            weight_ranges: t.weight_ranges.clone(),
                            italic_ranges: t.italic_ranges.clone(),
                            font_family_ranges: t.font_family_ranges.clone(),
                        },
                    ));
                }
                DrawCommand::Image(id, img) => {
                    // cull
                    if img.x + img.w < 0.0
                        || img.x > surface.width
                        || img.y + img.h < 0.0
                        || img.y > surface.height
                    {
                        continue;
                    }

                    let slot = self.image.get_or_alloc_slot(*id);
                    self.image.write_slot(
                        slot,
                        ImageInstance {
                            pos_size: [img.x, img.y, img.w, img.h],
                            radii: img.radii,
                            border_color: img.border_color,
                            border_widths: img.border_widths,
                            transform: transform(img.rotate, img.scale_x, img.scale_y),
                            clip: scale_clip(img.clip, surface.scale),
                            opacity: img.opacity,
                            _pad: [0.0; 3],
                        },
                        img.image_id,
                    );
                }
            }
        }

        /*
                let visible = text_specs.len();
                let culled = culled_text.len();
                println!("text visible: {}, culled: {}", visible, culled);
        */

        self.rect.upload(&ctx.device, &ctx.queue);
        // let t = std::time::Instant::now();
        self.text
            .prepare(&text_specs, font_system, &ctx.device, &ctx.queue);
        // println!("text prepare time: {:?}", t.elapsed());
        self.image.upload(&ctx.device, &ctx.queue);

        let all_bg_rects = self.text.bg_rects.clone();
        let all_line_rects = self.text.line_rects.clone();
        let mut combined = all_bg_rects.clone();
        combined.extend_from_slice(&all_line_rects);
        self.rect
            .prepare_transient(&combined, &ctx.device, &ctx.queue);
        let line_offset = all_bg_rects.len() as u32;

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

            // draw in z order
            // text_index only counts non-culled texts since that matches text_specs order
            let mut text_index = 0;
            for cmd in &draw_list.commands {
                match cmd {
                    DrawCommand::Rect(id, _) => {
                        if let Some(slot) = self.rect.slot_for_id(*id) {
                            self.rect.draw_slot(&mut pass, slot);
                        }
                    }
                    DrawCommand::Text(id, _) => {
                        if culled_text.contains(id) {
                            continue;
                        }
                        if let Some(&slot) = self.text.id_to_slot.get(id) {
                            if let Some(&(start, end)) = self.text.bg_ranges.get(text_index) {
                                self.rect.draw_transient_range(
                                    &mut pass,
                                    start as u32,
                                    (end - start) as u32,
                                );
                            }
                            self.text.draw_range(&mut pass, slot);
                            if let Some(&(start, end)) = self.text.line_ranges.get(text_index) {
                                self.rect.draw_transient_range(
                                    &mut pass,
                                    line_offset + start as u32,
                                    (end - start) as u32,
                                );
                            }
                        }
                        text_index += 1;
                    }
                    DrawCommand::Image(id, _) => {
                        if let Some(slot) = self.image.slot_for_id(*id) {
                            self.image.draw_slot(&mut pass, slot);
                        }
                    }
                }
            }
        }

        // let t = std::time::Instant::now();
        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        // println!("present time: {:?}", t.elapsed());
    }

    pub fn resize(&mut self, ctx: &RenderContext, surface: &Surface) {
        self.rect.resize(&ctx.queue, surface.width, surface.height);
        self.text
            .resize(&ctx.queue, surface.width, surface.height, surface.scale);
        self.image
            .resize(&ctx.queue, surface.width, surface.height, surface.scale);
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
