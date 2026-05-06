use bento_wgpu::RenderContext;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::WindowId,
};

use crate::{
    fonts::Fonts,
    ui::Ui,
    window::{WindowConfig, WindowInstance},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowHandle(pub(crate) u32);

pub struct App {
    pub(crate) fonts: Fonts,

    pending: Vec<(WindowHandle, WindowConfig, Ui)>,
    next_handle: u32,

    ctx: Option<RenderContext>,
    windows: HashMap<WindowId, WindowInstance>,
    handle_to_id: HashMap<WindowHandle, WindowId>,
    close_queue: Vec<WindowId>,
}

impl App {
    pub fn new() -> Self {
        Self {
            fonts: Fonts::new(),
            pending: Vec::new(),
            next_handle: 0,
            ctx: None,
            windows: HashMap::new(),
            handle_to_id: HashMap::new(),
            close_queue: Vec::new(),
        }
    }

    pub fn open_window(&mut self, config: WindowConfig, ui: Ui) -> WindowHandle {
        let handle = WindowHandle(self.next_handle);
        self.next_handle += 1;
        self.pending.push((handle, config, ui));
        handle
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut self).unwrap();
    }

    fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        handle: WindowHandle,
        config: WindowConfig,
        ui: Ui,
    ) {
        let ctx = self.ctx.as_ref().unwrap();
        let mut win = WindowInstance::new(ctx, event_loop, config, ui);
        let id = win.id();
        self.handle_to_id.insert(handle, id);
        self.windows.insert(id, win);
    }

    fn process_close_queue(&mut self, event_loop: &ActiveEventLoop) {
        for id in self.close_queue.drain(..) {
            if let Some(win) = self.windows.remove(&id) {
                self.handle_to_id.retain(|_, v| *v != id);
                drop(win);
            }
        }
        if self.windows.is_empty() {
            event_loop.exit();
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        /*
        *  tokio later if needed
        *
        *  if self.ctx.is_none() {
        *      let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
        *      self.ctx = Some(rt.block_on(RenderContext::new()));
        *  }
        */
        if self.ctx.is_none() {
            self.ctx = Some(pollster::block_on(RenderContext::new()));
        }

        for (handle, config, ui) in std::mem::take(&mut self.pending) {
            self.create_window(event_loop, handle, config, ui);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        self.process_close_queue(event_loop);

        let Some(win) = self.windows.get_mut(&id) else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {}

            WindowEvent::CursorMoved { position, .. } => {}

            WindowEvent::MouseInput { button, state, .. } => {}

            WindowEvent::MouseWheel { delta, .. } => {}

            WindowEvent::KeyboardInput { event: ke, .. } => {}

            WindowEvent::ModifiersChanged(mods) => {}

            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {}

            WindowEvent::CloseRequested => {
                self.close_queue.push(id);
            }

            _ => {}
        }
    }

    // place for timers and animation ticks
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {}
}
