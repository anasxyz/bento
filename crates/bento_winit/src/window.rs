use crate::config::WindowConfig;
use bento_ui::Ui;
use bento_wgpu::{RenderContext, Renderer, Surface};
use cosmic_text::FontSystem;
use std::sync::Arc;
use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::WindowId};

pub struct Window {
    pub config: WindowConfig,
    pub renderer: Renderer,
    pub surface: Surface<'static>,
    pub ui: Ui,
    window: Arc<winit::window::Window>,
    pub last_frame: Option<web_time::Instant>,
    pub needs_render: bool,
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

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .append_child(&window.canvas().unwrap())
                .unwrap();
        }

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
            ui,
            window,
            last_frame: None,
            needs_render: true,
        }
    }

    pub fn from_parts(
        config: WindowConfig,
        renderer: Renderer,
        surface: Surface<'static>,
        ui: Ui,
        window: Arc<winit::window::Window>,
    ) -> Self {
        Self {
            config,
            renderer,
            surface,
            ui,
            window,
            last_frame: None,
            needs_render: true
        }
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }

    pub fn needs_render(&self) -> bool {
        self.needs_render 
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw();
    }

    pub fn set_cursor(&self, cursor: winit::window::CursorIcon) {
        self.window.set_cursor(cursor);
    }

    pub fn resize(&mut self, ctx: &RenderContext) {
        let size = self.window.inner_size();
        let scale = self.window.scale_factor() as f32;
        let w = size.width as f32 / scale;
        let h = size.height as f32 / scale;
        self.surface.resize(ctx, w, h, scale);
        self.renderer.resize(ctx, &self.surface);
    }
}
