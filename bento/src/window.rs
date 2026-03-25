use bento_wgpu::{RenderContext, Renderer, SceneGraph, Surface};
use std::sync::Arc;
use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::Window};

use crate::settings::WindowConfig;

pub struct BentoWindow {
    pub config: WindowConfig,
    pub scene: SceneGraph,
    pub renderer: Renderer,
    pub surface: Surface<'static>,
    pub window: Arc<Window>,
}

impl BentoWindow {
    pub fn new(ctx: &RenderContext, event_loop: &ActiveEventLoop, config: WindowConfig) -> Self {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
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
            window,
            surface,
            renderer,
            config,
            scene: SceneGraph::new(),
        }
    }

    pub fn resize(&mut self, ctx: &RenderContext) {
        let size = self.window.inner_size();
        let scale = self.window.scale_factor() as f32;
        let w = size.width as f32 / scale;
        let h = size.height as f32 / scale;
        self.surface.resize(ctx, w, h, scale);
        self.window.request_redraw();
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
