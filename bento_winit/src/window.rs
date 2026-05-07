use crate::config::WindowConfig;
use bento_wgpu::{RenderContext, Renderer, Scene, Surface};
use cosmic_text::FontSystem;
use std::sync::Arc;
use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::WindowId};

pub struct Window {
    pub config: WindowConfig,
    pub renderer: Renderer,
    pub surface: Surface<'static>,
    pub font_system: FontSystem,
    pub scene: Scene,
    window: Arc<winit::window::Window>,
}

impl Window {
    pub fn new(ctx: &RenderContext, event_loop: &ActiveEventLoop, config: WindowConfig) -> Self {
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
        Self {
            config,
            renderer,
            surface,
            font_system: FontSystem::new(),
            scene: Scene::new(),
            window,
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
