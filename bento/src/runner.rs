/*
* routes events to the correct BentoWindow
* owns the shared RenderContext and the map of open windows
*/

use bento_wgpu::RenderContext;
use std::collections::HashMap;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::WindowId,
};

use crate::app::WindowHandle;
use crate::settings::WindowConfig;
use crate::window::BentoWindow;

pub struct Runner {
    pub ctx: RenderContext,
    pub windows: HashMap<WindowId, BentoWindow>,
    // configs queued before the event loop started, consumed on first resumed()
    pending: Vec<(WindowHandle, WindowConfig)>,
    // pending close requests
    // processed at the start of each event
    close_queue: Vec<WindowHandle>,
    // mapping from user facing WindowHandle to winit WindowId
    handle_to_id: HashMap<WindowHandle, WindowId>,
}

impl Runner {
    pub fn new(ctx: RenderContext, pending: Vec<(WindowHandle, WindowConfig)>) -> Self {
        Self {
            ctx,
            windows: HashMap::new(),
            pending,
            close_queue: Vec::new(),
            handle_to_id: HashMap::new(),
        }
    }

    pub fn open_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        handle: WindowHandle,
        config: WindowConfig,
    ) {
        let win = BentoWindow::new(&self.ctx, event_loop, config);
        let id = win.id();
        self.handle_to_id.insert(handle, id);
        self.windows.insert(id, win);
    }

    pub fn close_window(&mut self, handle: WindowHandle) {
        self.close_queue.push(handle);
    }

    fn process_close_queue(&mut self, event_loop: &ActiveEventLoop) {
        for handle in self.close_queue.drain(..) {
            if let Some(&id) = self.handle_to_id.get(&handle) {
                if let Some(win) = self.windows.remove(&id) {
                    let BentoWindow {
                        renderer,
                        surface,
                        window,
                        ..
                    } = win;
                    drop(renderer);
                    drop(surface);
                    drop(window);
                }
                self.handle_to_id.remove(&handle);
            }
        }
        if self.windows.is_empty() {
            event_loop.exit();
        }
    }
}

impl ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let pending = std::mem::take(&mut self.pending);
        for (handle, config) in pending {
            self.open_window(event_loop, handle, config);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        event_loop.set_control_flow(ControlFlow::Wait);

        self.process_close_queue(event_loop);

        let Some(win) = self.windows.get_mut(&id) else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                let clear = win.config.clear_color.to_array();
                win.renderer
                    .render(&self.ctx, &mut win.surface, &mut win.scene, clear);
            }

            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                win.resize(&self.ctx);
            }

            WindowEvent::CloseRequested => {
                if let Some(win) = self.windows.remove(&id) {
                    let BentoWindow {
                        renderer,
                        surface,
                        window,
                        ..
                    } = win;
                    drop(renderer);
                    drop(surface);
                    drop(window);
                }
                // remove from handle map
                self.handle_to_id.retain(|_, v| *v != id);
                if self.windows.is_empty() {
                    event_loop.exit();
                }
            }

            _ => {}
        }
    }
}
