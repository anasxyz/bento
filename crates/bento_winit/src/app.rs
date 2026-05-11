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

use bento_shared::{BentoEvent, CosmicTextMeasurer};
use std::sync::Arc;

pub struct App {
    ctx: Option<RenderContext>,
    pending: Vec<(WindowConfig, Ui)>,
    windows: HashMap<WindowId, Window>,
    close_queue: Vec<WindowId>,
    runtime: tokio::runtime::Runtime,
}

impl App {
    pub fn new() -> Self {
        Self {
            ctx: None,
            pending: Vec::new(),
            windows: HashMap::new(),
            close_queue: Vec::new(),
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }

    pub fn open_window(&mut self, config: WindowConfig, ui: Ui) -> &mut Self {
        self.pending.push((config, ui));
        self
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::<BentoEvent>::with_user_event().build().unwrap();
        let proxy = event_loop.create_proxy();
        for (_, ui) in &mut self.pending {
            let proxy = proxy.clone();
            ui.events.set_sender(Arc::new(move |id| {
                proxy.send_event(BentoEvent::Callback(id)).ok();
            }));
            let handle = self.runtime.handle().clone();
            ui.events.set_spawner(Arc::new(move |fut| {
                handle.spawn(fut);
            }));
        }
        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler<BentoEvent> for App {
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
                win.ui.input.mouse.reset();

                win.ui.update();

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
                        physical_key: winit::keyboard::PhysicalKey::Code(key),
                        state: winit::event::ElementState::Pressed,
                        ..
                    },
                ..
            } => match key {
                winit::keyboard::KeyCode::KeyS => println!("{}", win.ui.scene()),
                winit::keyboard::KeyCode::KeyU => println!("{}", win.ui),
                _ => {}
            },

            WindowEvent::MouseInput { state, button, .. } => {
                let btn = match button {
                    winit::event::MouseButton::Left => &mut win.ui.input.mouse.left,
                    winit::event::MouseButton::Right => &mut win.ui.input.mouse.right,
                    winit::event::MouseButton::Middle => &mut win.ui.input.mouse.middle,
                    _ => return,
                };
                match state {
                    winit::event::ElementState::Pressed => {
                        btn.pressed = true;
                        btn.released = false;
                        btn.just_pressed = true;
                        btn.just_released = false;
                    }
                    winit::event::ElementState::Released => {
                        btn.pressed = false;
                        btn.released = true;
                        btn.just_pressed = false;
                        btn.just_released = true;
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                winit::event::MouseScrollDelta::LineDelta(x, y) => {
                    win.ui.input.mouse.scroll_x = x;
                    win.ui.input.mouse.scroll_y = y;
                }
                winit::event::MouseScrollDelta::PixelDelta(pos) => {
                    win.ui.input.mouse.scroll_x = pos.x as f32;
                    win.ui.input.mouse.scroll_y = pos.y as f32;
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                let x = position.x as f32;
                let y = position.y as f32;
                win.ui.input.mouse.dx = x - win.ui.input.mouse.x;
                win.ui.input.mouse.dy = y - win.ui.input.mouse.y;
                win.ui.input.mouse.x = x;
                win.ui.input.mouse.y = y;
            }
            WindowEvent::CursorEntered { .. } => {
                win.ui.input.mouse.inside_window = true;
            }
            WindowEvent::CursorLeft { .. } => {
                win.ui.input.mouse.inside_window = false;
            }

            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                win.resize(ctx);
                let w = win.surface.width;
                let h = win.surface.height;
                win.request_redraw();
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

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: BentoEvent) {
        match event {
            BentoEvent::Callback(id) => {
                for win in self.windows.values_mut() {
                    win.ui.fire_callback(id);
                    win.request_redraw();
                }
            }
        }
    }
}
