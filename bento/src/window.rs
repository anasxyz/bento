use bento_wgpu::{RenderContext, Renderer, Surface};
use std::sync::Arc;
use std::time::Instant;
use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::Window};

use crate::input::{InputState, map_cursor};
use crate::settings::WindowConfig;
use crate::ui::Ui;

pub struct BentoWindow {
    pub config: WindowConfig,
    pub ui: Ui,
    pub input: InputState,
    pub renderer: Renderer,
    pub surface: Surface<'static>,
    pub window: Arc<Window>,
    pub blink_deadline: Option<Instant>,
}

impl BentoWindow {
    pub fn new(
        ctx: &RenderContext,
        event_loop: &ActiveEventLoop,
        config: WindowConfig,
        ui: Ui,
    ) -> Self {
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

        let mut ui = ui;
        ui.window_width = w as u32;
        ui.window_height = h as u32;

        Self {
            window,
            surface,
            renderer,
            ui,
            input: InputState::new(),
            config,
            blink_deadline: None,
        }
    }

    pub fn resize(&mut self, ctx: &RenderContext) {
        let size = self.window.inner_size();
        let scale = self.window.scale_factor() as f32;
        let w = size.width as f32 / scale;
        let h = size.height as f32 / scale;
        self.surface.resize(ctx, w, h, scale);
        self.renderer.resize(ctx, &self.surface, &mut self.ui.scene);
        self.ui.window_width = w as u32;
        self.ui.window_height = h as u32;
        for slot in self.ui.slots.iter_mut().flatten() {
            slot.widget.base_mut().layout_dirty = true;
            slot.widget.base_mut().render_dirty = true;
        }
        self.window.request_redraw();
    }

    pub fn sync_cursor(&self) {
        let cursor = self
            .ui
            .interaction
            .hovered
            .and_then(|h| self.ui.slots.get(h.id as usize))
            .and_then(|s| s.as_ref())
            .map(|s| map_cursor(&s.widget.base().cursor))
            .unwrap_or(winit::window::CursorIcon::Default);
        self.window.set_cursor(cursor);
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
