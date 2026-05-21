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
            ui.asyncs.set_sender(Arc::new(move |id| {
                proxy.send_event(BentoEvent::Callback(id)).ok();
            }));
            let handle = self.runtime.handle().clone();
            ui.asyncs.set_spawner(Arc::new(move |fut| {
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
                win.ui.process_input();
                win.ui.update();

                if win.needs_render || win.ui.needs_redraw {
                    let font_system = &mut win.ui.measurer.font_system;
                    let scene = &mut win.ui.scene;
                    win.renderer.render(
                        ctx,
                        font_system,
                        &mut win.surface,
                        win.config.clear_color,
                        scene,
                    );
                    win.needs_render = false;
                    win.ui.needs_redraw = false;
                }

                win.ui.input.mouse.clear();
                win.ui.input.keyboard.clear();
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
            } => {
                let key = crate::input::keycode_to_key(keycode);
                let ch = text.and_then(|s| s.chars().next());
                match state {
                    winit::event::ElementState::Pressed => {
                        match key {
                            bento_ui::Key::LShift | bento_ui::Key::RShift => {
                                win.ui.input.keyboard.modifiers.shift = true
                            }
                            bento_ui::Key::LCtrl | bento_ui::Key::RCtrl => {
                                win.ui.input.keyboard.modifiers.ctrl = true
                            }
                            bento_ui::Key::LAlt | bento_ui::Key::RAlt => {
                                win.ui.input.keyboard.modifiers.alt = true
                            }
                            bento_ui::Key::LSuper | bento_ui::Key::RSuper => {
                                win.ui.input.keyboard.modifiers.super_key = true
                            }
                            _ => {}
                        }
                        win.ui.input.keyboard.on_press(key, ch);
                    }
                    winit::event::ElementState::Released => {
                        match key {
                            bento_ui::Key::LShift | bento_ui::Key::RShift => {
                                win.ui.input.keyboard.modifiers.shift = false
                            }
                            bento_ui::Key::LCtrl | bento_ui::Key::RCtrl => {
                                win.ui.input.keyboard.modifiers.ctrl = false
                            }
                            bento_ui::Key::LAlt | bento_ui::Key::RAlt => {
                                win.ui.input.keyboard.modifiers.alt = false
                            }
                            bento_ui::Key::LSuper | bento_ui::Key::RSuper => {
                                win.ui.input.keyboard.modifiers.super_key = false
                            }
                            _ => {}
                        }
                        win.ui.input.keyboard.on_release(key);
                    }
                }

                win.request_redraw();
            }

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

                win.request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => {
                        win.ui.input.mouse.scroll_x = x;
                        win.ui.input.mouse.scroll_y = y;
                    }
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        win.ui.input.mouse.scroll_x = pos.x as f32;
                        win.ui.input.mouse.scroll_y = pos.y as f32;
                    }
                }

                win.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                let scale = win.surface.scale;
                let x = position.x as f32 / scale;
                let y = position.y as f32 / scale;
                win.ui.input.mouse.dx = x - win.ui.input.mouse.x;
                win.ui.input.mouse.dy = y - win.ui.input.mouse.y;
                win.ui.input.mouse.x = x;
                win.ui.input.mouse.y = y;
                win.request_redraw();
            }
            WindowEvent::CursorEntered { .. } => {
                win.ui.input.mouse.inside_window = true;
                win.ui.input.mouse.just_entered = true;
            }
            WindowEvent::CursorLeft { .. } => {
                win.ui.input.mouse.inside_window = false;
                win.ui.input.mouse.just_left = true;
            }

            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                win.resize(ctx);
                let w = win.surface.width;
                let h = win.surface.height;
                win.needs_render = true;
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
