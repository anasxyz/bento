use crate::{config::WindowConfig, window::Window};
use bento_shared::{
    scene::{FontFamilyRange, ItalicRange, WeightRange},
    measure::{TextMeasureRequest, TextMeasurer},
    measurer::CosmicTextMeasurer,
};
use bento_wgpu::{RenderContext};
use std::collections::HashMap;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

pub struct App {
    ctx: Option<RenderContext>,
    pending: Vec<WindowConfig>,
    windows: HashMap<WindowId, Window>,
    close_queue: Vec<WindowId>,
}

impl App {
    pub fn new() -> Self {
        Self {
            ctx: None,
            pending: Vec::new(),
            windows: HashMap::new(),
            close_queue: Vec::new(),
        }
    }

    pub fn open_window(&mut self, config: WindowConfig) -> &mut Self {
        self.pending.push(config);
        self
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.ctx.is_none() {
            self.ctx = Some(pollster::block_on(RenderContext::new()));
        }
        for config in std::mem::take(&mut self.pending) {
            let ctx = self.ctx.as_ref().unwrap();
            let win = Window::new(ctx, event_loop, config);
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
                let clear = win.config.clear_color;
                win.renderer.render(
                    ctx,
                    &mut win.font_system,
                    &mut win.surface,
                    clear,
                    &mut win.scene,
                );
            }
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                win.resize(ctx);
            }
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
}
