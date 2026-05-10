use crate::config::WindowConfig;
use bento_shared::MeasureCache;
use bento_ui::Ui;
use bento_wgpu::{RenderContext, Renderer, Surface};
use cosmic_text::FontSystem;
use std::sync::Arc;
use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::WindowId};

pub struct Window {
    pub config: WindowConfig,
    pub renderer: Renderer,
    pub surface: Surface<'static>,
    pub font_system: FontSystem,
    pub measure_cache: MeasureCache,
    pub ui: Ui,
    window: Arc<winit::window::Window>,
    pub last_frame: Option<std::time::Instant>,
}

impl Window {
    pub fn new(
        ctx: &RenderContext,
        event_loop: &ActiveEventLoop,
        config: WindowConfig,
        ui: Ui,
    ) -> Self {
        let window = Arc::new(
            event_loop
                .create_window(
                    winit::window::Window::default_attributes()
                        .with_title(&config.title)
                        .with_inner_size(LogicalSize::new(config.width, config.height)),
                )
                .unwrap(),
        );
        let size = window.inner_size();
        let scale = window.scale_factor() as f32;
        let w = size.width as f32 / scale;
        let h = size.height as f32 / scale;
        let surface = Surface::new(ctx, Arc::clone(&window), w, h, scale);
        let renderer = Renderer::new(ctx, &surface);

        let mut font_system = FontSystem::new();
        let measure_cache = MeasureCache::new(&mut font_system);

        Self {
            config,
            renderer,
            surface,
            font_system,
            measure_cache,
            ui,
            window,
            last_frame: None,
        }
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&mut self, ctx: &RenderContext) {
        let size = self.window.inner_size();
        let scale = self.window.scale_factor() as f32;
        let w = size.width as f32 / scale;
        let h = size.height as f32 / scale;
        self.surface.resize(ctx, w, h, scale);
        self.renderer.resize(ctx, &self.surface);
        self.window.request_redraw();
    }
}
