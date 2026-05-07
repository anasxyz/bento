// wraps a wgpu Surface for a single window
// multiple surfaces can share the same RenderContext

use wgpu;
use crate::context::RenderContext;

pub struct Surface<'window> {
    pub(crate) surface: wgpu::Surface<'window>,
    pub(crate) config:  wgpu::SurfaceConfiguration,
    pub format:  wgpu::TextureFormat,
    pub width:   f32,   // logical pixels
    pub height:  f32,
    pub scale:   f32,
}

impl<'window> Surface<'window> {
    pub fn new(
        ctx:    &RenderContext,
        window: impl wgpu::WindowHandle + 'window,
        width:  f32,
        height: f32,
        scale:  f32,
    ) -> Self {
        let surface = ctx.instance.create_surface(window).unwrap();
        let caps    = surface.get_capabilities(&ctx.adapter);
        let format = caps.formats.iter()
            .find(|f| **f == wgpu::TextureFormat::Rgba8UnormSrgb)
            .copied()
            .unwrap_or(caps.formats[0]);

        // skould prefer PreMultiplied if supported for correct compositing
        let alpha_mode = if caps.alpha_modes.contains(&wgpu::CompositeAlphaMode::PreMultiplied) {
            wgpu::CompositeAlphaMode::PreMultiplied
        } else {
            caps.alpha_modes[0]
        };

        let phys_w = (width  * scale) as u32;
        let phys_h = (height * scale) as u32;

        let config = wgpu::SurfaceConfiguration {
            usage:    wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
            format,
            width:    phys_w.max(1),
            height:   phys_h.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };
        surface.configure(&ctx.device, &config);

        Self { surface, config, format, width, height, scale }
    }

    /// call when the window is resized or rescaled
    pub fn resize(&mut self, ctx: &RenderContext, width: f32, height: f32, scale: f32) {
        self.width  = width;
        self.height = height;
        self.scale  = scale;
        self.config.width  = ((width  * scale) as u32).max(1);
        self.config.height = ((height * scale) as u32).max(1);
        self.surface.configure(&ctx.device, &self.config);
    }

    pub fn physical_width(&self)  -> u32 { self.config.width }
    pub fn physical_height(&self) -> u32 { self.config.height }
}
