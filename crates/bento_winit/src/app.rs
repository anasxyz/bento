use std::collections::HashMap;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use crate::{config::WindowConfig, window::Window};
use bento_ui::{CursorIcon, Ui};
use bento_wgpu::RenderContext;

use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use {
    bento_wgpu::{Renderer, Surface},
    winit::dpi::LogicalSize,
};

pub struct App {
    ctx: Option<RenderContext>,
    pending: Vec<(WindowConfig, Ui)>,
    windows: HashMap<WindowId, Window>,
    close_queue: Vec<WindowId>,
    #[cfg(not(target_arch = "wasm32"))]
    runtime: tokio::runtime::Runtime,
}

pub enum BentoEvent {
    Callback(u64),
}

impl App {
    pub fn new() -> Self {
        Self {
            ctx: None,
            pending: Vec::new(),
            windows: HashMap::new(),
            close_queue: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }

    pub fn open_window(&mut self, config: WindowConfig, ui: Ui) -> &mut Self {
        self.pending.push((config, ui));
        self
    }

    pub fn launch(mut self) {
        let event_loop = EventLoop::<BentoEvent>::with_user_event().build().unwrap();
        let proxy = event_loop.create_proxy();
        event_loop.run_app(&mut self).unwrap();
    }

    pub fn run(view: impl bento_ui::View + 'static) {
        let ui = bento_ui::Ui::new(view);
        let mut app = App::new();
        app.open_window(WindowConfig::default(), ui);
        app.launch();
    }
}

impl ApplicationHandler<BentoEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(not(target_arch = "wasm32"))]
        if self.ctx.is_none() {
            self.ctx = Some(pollster::block_on(RenderContext::new()));
        }

        for (config, ui) in std::mem::take(&mut self.pending) {
            #[cfg(target_arch = "wasm32")]
            let mut win = {
                use winit::platform::web::WindowExtWebSys;
                let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
                let window = Arc::new(
                    event_loop
                        .create_window(
                            winit::window::Window::default_attributes()
                                .with_title(&config.title)
                                .with_inner_size(LogicalSize::new(config.width, config.height)),
                        )
                        .unwrap(),
                );
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .body()
                    .unwrap()
                    .append_child(&window.canvas().unwrap())
                    .unwrap();
                let size = window.inner_size();
                let scale = window.scale_factor() as f32;
                let w = size.width as f32 / scale;
                let h = size.height as f32 / scale;
                let surface_handle = instance.create_surface(Arc::clone(&window)).unwrap();
                let ctx =
                    pollster::block_on(RenderContext::new_for_surface(instance, &surface_handle));
                if self.ctx.is_none() {
                    self.ctx = Some(ctx);
                }
                let ctx = self.ctx.as_ref().unwrap();
                let surface = Surface::from_existing(ctx, surface_handle, w, h, scale);
                let renderer = Renderer::new(ctx, &surface);
                Window::from_parts(config, renderer, surface, ui, window)
            };

            #[cfg(not(target_arch = "wasm32"))]
            let mut win = Window::new(self.ctx.as_ref().unwrap(), event_loop, config, ui);
            #[cfg(not(target_arch = "wasm32"))]
            win.request_redraw();
            self.windows.insert(win.id(), win);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(win) = self.windows.get_mut(&id) else {
            return;
        };
        let ctx = self.ctx.as_mut().unwrap();

        match event {
            WindowEvent::RedrawRequested => {
                let needs_redraw = win.needs_render || bento_ui::take_needs_redraw();
                if needs_redraw {
                    let draw_list = win.ui.collect_draw_list();
                    win.renderer.render(
                        ctx,
                        &mut win.ui.measurer,
                        &mut win.surface,
                        win.config.clear_color,
                        &draw_list,
                    );
                    win.needs_render = false;
                }
            }

            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(keycode),
                        text,
                        state,
                        ..
                    },
                ..
            } => {}

            WindowEvent::MouseInput { state, button, .. } => {}
            WindowEvent::MouseWheel { delta, .. } => {}
            WindowEvent::CursorMoved { position, .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}

            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::CloseRequested => {
                self.close_queue.push(id);
            }
            _ => {}
        }

        for id in self.close_queue.drain(..) {
            self.windows.remove(&id);
        }
        if self.windows.is_empty() {
            event_loop.exit();
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: BentoEvent) {
        match event {
            BentoEvent::Callback(id) => {
                for win in self.windows.values_mut() {
                    win.request_redraw();
                }
            }
        }
    }
}

pub fn to_winit_cursor(cursor: CursorIcon) -> winit::window::CursorIcon {
    match cursor {
        CursorIcon::Default => winit::window::CursorIcon::Default,
        CursorIcon::Text => winit::window::CursorIcon::Text,
        CursorIcon::Pointer => winit::window::CursorIcon::Pointer,
        CursorIcon::ResizeHorizontal => winit::window::CursorIcon::EwResize,
        CursorIcon::ResizeVertical => winit::window::CursorIcon::NsResize,
        CursorIcon::ResizeNwSe => winit::window::CursorIcon::NwseResize,
        CursorIcon::ResizeNeSw => winit::window::CursorIcon::NeswResize,
        CursorIcon::Crosshair => winit::window::CursorIcon::Crosshair,
        CursorIcon::NotAllowed => winit::window::CursorIcon::NotAllowed,
        CursorIcon::Grab => winit::window::CursorIcon::Grab,
        CursorIcon::Grabbing => winit::window::CursorIcon::Grabbing,
    }
}
