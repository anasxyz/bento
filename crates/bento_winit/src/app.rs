use bento_shared::CosmicTextMeasurer;
use std::collections::HashMap;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use crate::{config::WindowConfig, window::Window};
use bento_ui::Ui;
use bento_wgpu::RenderContext;

pub struct App {
    ctx: Option<RenderContext>,
    pending: Vec<(WindowConfig, Ui)>,
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

    pub fn open_window(&mut self, config: WindowConfig, ui: Ui) -> &mut Self {
        self.pending.push((config, ui));
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
        for (config, ui) in std::mem::take(&mut self.pending) {
            let ctx = self.ctx.as_ref().unwrap();
            let win = Window::new(ctx, event_loop, config, ui);
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
                let now = std::time::Instant::now();
                let delta = win
                    .last_frame
                    .map(|t| t.elapsed().as_secs_f32())
                    .unwrap_or(0.0);
                win.last_frame = Some(now);

                let mut measurer =
                    CosmicTextMeasurer::new(&mut win.font_system, &mut win.measure_cache);
                win.ui.update(&mut measurer, delta);

                if win.ui.any_dirty() {
                    win.request_redraw();
                }

                let clear = win.config.clear_color;
                win.renderer.render(
                    ctx,
                    &mut win.font_system,
                    &mut win.surface,
                    clear,
                    win.ui.scene_mut(),
                );
            }
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key:
                            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD),
                        state: winit::event::ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                println!("{:#?}", win.ui.scene);
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
