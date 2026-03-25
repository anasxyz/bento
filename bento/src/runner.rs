use bento_wgpu::RenderContext;
use std::collections::HashMap;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::WindowId,
};

use crate::settings::WindowConfig;
use crate::window::BentoWindow;

pub struct Runner {
    pub ctx: RenderContext,
    pub windows: HashMap<WindowId, BentoWindow>,
    pending: Vec<WindowConfig>,
}

impl Runner {
    pub fn new(ctx: RenderContext, config: WindowConfig) -> Self {
        Self {
            ctx,
            windows: HashMap::new(),
            pending: vec![config],
        }
    }

    pub fn open_window(&mut self, event_loop: &ActiveEventLoop, config: WindowConfig) {
        let win = BentoWindow::new(&self.ctx, event_loop, config);
        self.windows.insert(win.id(), win);
    }
}

impl ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("resumed");
        let pending = std::mem::take(&mut self.pending);
        for config in pending {
            self.open_window(event_loop, config);
            println!("window opened");
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        event_loop.set_control_flow(ControlFlow::Wait);
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
                    // explicitly drop renderer before surface before window
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
                if self.windows.is_empty() {
                    event_loop.exit();
                }
            }

            _ => {}
        }
    }
}
