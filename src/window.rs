use crate::color::Color;
use crate::draw::DrawList;
use crate::render::draw_ctx::DrawContext;
use crate::render::gpu::GpuContext;
use std::sync::Arc;
use winit::window::Window;

pub struct WindowState {
    pub window: Arc<Window>,
    pub gpu: GpuContext,
    pub clear_color: Color,
    pub draw: DrawContext,
    pub draw_list: DrawList,
    first_frame: bool,
}

impl WindowState {
    pub fn new(window: Arc<Window>, gpu: GpuContext, clear_color: Color) -> Self {
        let size = window.inner_size();
        let scale = window.scale_factor();
        let draw = DrawContext::new(
            &gpu.device,
            &gpu.queue,
            gpu.format,
            size.width as f32 / scale as f32,
            size.height as f32 / scale as f32,
            scale as f32,
        );
        Self {
            window,
            gpu,
            clear_color,
            draw,
            draw_list: DrawList::new(),
            first_frame: true,
        }
    }

    pub fn begin(&mut self) {
        self.draw.clear();
    }

    pub fn resize_and_rescale(&mut self) {
        let size = self.window.inner_size();
        let scale = self.window.scale_factor();
        self.gpu.resize(size.width, size.height);
        self.draw.set_scale(
            scale as f32,
            size.width as f32 / scale as f32,
            size.height as f32 / scale as f32,
        );
        self.draw_list.invalidate();
        self.first_frame = true;
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw();
    }

    pub fn render(&mut self, dirty_region: Option<[f32; 4]>, has_dirty: bool) {
        let frame = match self.gpu.begin_frame() {
            Ok(f) => f,
            Err(_) => return,
        };

        let scale = self.window.scale_factor() as f32;
        let phys_w = self.gpu.config.width;
        let phys_h = self.gpu.config.height;
        let c = self.clear_color;

        let (mut encoder, finisher, _surface_view) = frame.begin();

        if has_dirty || self.first_frame {
            // compute scissor rect in physical pixels
            let scissor = if self.first_frame || dirty_region.is_none() {
                None // full redraw
            } else {
                dirty_region.map(|[x, y, x2, y2]| {
                    let px = ((x * scale).floor() as u32).min(phys_w);
                    let py = ((y * scale).floor() as u32).min(phys_h);
                    let px2 = ((x2 * scale).ceil() as u32).min(phys_w);
                    let py2 = ((y2 * scale).ceil() as u32).min(phys_h);
                    (px, py, (px2 - px).max(1), (py2 - py).max(1))
                })
            };

            // render into backing store
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Backing Store Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.gpu.backing_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: if scissor.is_none() {
                                // full clear
                                wgpu::LoadOp::Clear(wgpu::Color {
                                    r: c.r as f64,
                                    g: c.g as f64,
                                    b: c.b as f64,
                                    a: c.a as f64,
                                })
                            } else {
                                // partial
                                // preserve existing backing store
                                wgpu::LoadOp::Load
                            },
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                if let Some((x, y, w, h)) = scissor {
                    pass.set_scissor_rect(x, y, w, h);
                }

                self.draw
                    .render(&self.gpu.device, &self.gpu.queue, &mut pass);
            }

            self.first_frame = false;
        }

        // blit backing store to surface texture
        encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.gpu.backing_store,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyTextureInfo {
                texture: &finisher.frame.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: phys_w,
                height: phys_h,
                depth_or_array_layers: 1,
            },
        );

        finisher.present(encoder, &self.gpu.queue);
    }
}
