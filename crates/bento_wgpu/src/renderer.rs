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

        // phase 1: build CPU vecs from draw list
        let mut rect_instances: Vec<RectInstance> = Vec::new();
        let mut image_instances: Vec<ImageInstance> = Vec::new();
        let mut image_ids: Vec<u64> = Vec::new();
        let mut text_specs: Vec<TextSpec> = Vec::new();

        // track which text commands are culled so we can skip them in the render pass
        let mut text_culled: Vec<bool> = Vec::new();

        for cmd in &draw_list.commands {
            match cmd {
                DrawCommand::Rect(r) => {
                    let culled = r.x + r.w < 0.0
                        || r.x > surface.width
                        || r.y + r.h < 0.0
                        || r.y > surface.height;
                    // push a zero-size instance for culled rects to keep indices aligned
                    rect_instances.push(if culled {
                        RectInstance {
                            pos_size: [0.0; 4],
                            color: [0.0; 4],
                            radii: [0.0; 4],
                            border_color: [0.0; 4],
                            border_widths: [0.0; 4],
                            transform: [1.0, 0.0, 0.0, 1.0],
                            clip: [0.0, 0.0, 0.0, 0.0],
                        }
                    } else {
                        RectInstance {
                            pos_size: [r.x, r.y, r.w, r.h],
                            color: [r.color[0], r.color[1], r.color[2], r.color[3] * r.opacity],
                            radii: r.radii,
                            border_color: r.border_color,
                            border_widths: r.border_widths,
                            transform: transform(r.rotate, r.scale_x, r.scale_y),
                            clip: scale_clip(r.clip, surface.scale),
                        }
                    });
                }
                DrawCommand::Text(t) => {
                    let culled = t.x + t.w < 0.0
                        || t.x > surface.width
                        || t.y + t.h < 0.0
                        || t.y > surface.height;
                    text_culled.push(culled);
                    if !culled {
                        text_specs.push(TextSpec {
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
                        });
                    }
                }
                DrawCommand::Image(img) => {
                    let culled = img.x + img.w < 0.0
                        || img.x > surface.width
                        || img.y + img.h < 0.0
                        || img.y > surface.height;
                    image_instances.push(if culled {
                        ImageInstance {
                            pos_size: [0.0; 4],
                            radii: [0.0; 4],
                            border_color: [0.0; 4],
                            border_widths: [0.0; 4],
                            transform: [1.0, 0.0, 0.0, 1.0],
                            clip: [0.0, 0.0, 0.0, 0.0],
                            opacity: 0.0,
                            _pad: [0.0; 3],
                        }
                    } else {
                        ImageInstance {
                            pos_size: [img.x, img.y, img.w, img.h],
                            radii: img.radii,
                            border_color: img.border_color,
                            border_widths: img.border_widths,
                            transform: transform(img.rotate, img.scale_x, img.scale_y),
                            clip: scale_clip(img.clip, surface.scale),
                            opacity: img.opacity,
                            _pad: [0.0; 3],
                        }
                    });
                    image_ids.push(img.image_id);
                }
            }
        }

        // phase 2: prepare text, then append decorations to rect instances
        let t = std::time::Instant::now();
        self.text
            .prepare(&text_specs, font_system, &ctx.device, &ctx.queue);
        println!("text prepare time: {:?}", t.elapsed());
        let decoration_offset = rect_instances.len();
        rect_instances.extend_from_slice(&self.text.bg_rects);
        let line_offset = rect_instances.len();
        rect_instances.extend_from_slice(&self.text.line_rects);

        // upload
        self.rect
            .prepare_transient(&rect_instances, &ctx.device, &ctx.queue);
        self.image
            .prepare_transient(&image_instances, &ctx.device, &ctx.queue);

        // phase 3: render pass
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

            let mut rect_index: u32 = 0;
            let mut text_index: usize = 0; // indexes into text_specs (non culled only)
            let mut text_cmd_index: usize = 0; // indexes into text_culled
            let mut image_index: usize = 0;

            for cmd in &draw_list.commands {
                match cmd {
                    DrawCommand::Rect(_) => {
                        self.rect.draw_transient_range(&mut pass, rect_index, 1);
                        rect_index += 1;
                    }
                    DrawCommand::Text(_) => {
                        if !text_culled[text_cmd_index] {
                            if let Some((bg_start, bg_end)) = self.text.bg_range(text_index) {
                                self.rect.draw_transient_range(
                                    &mut pass,
                                    decoration_offset as u32 + bg_start as u32,
                                    (bg_end - bg_start) as u32,
                                );
                            }
                            self.text.draw_range(&mut pass, text_index);
                            if let Some((line_start, line_end)) = self.text.line_range(text_index) {
                                self.rect.draw_transient_range(
                                    &mut pass,
                                    line_offset as u32 + line_start as u32,
                                    (line_end - line_start) as u32,
                                );
                            }
                            text_index += 1;
                        }
                        text_cmd_index += 1;
                    }
                    DrawCommand::Image(_) => {
                        self.image
                            .draw_slot(&mut pass, image_index, image_ids[image_index]);
                        image_index += 1;
                    }
                }
            }
        }

        let t = std::time::Instant::now();
        ctx.queue.submit(Some(encoder.finish()));
        frame.present();
        println!("present time: {:?}", t.elapsed());
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
