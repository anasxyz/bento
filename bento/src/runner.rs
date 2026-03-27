use bento_wgpu::RenderContext;
use std::collections::HashMap;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    keyboard::PhysicalKey,
    window::WindowId,
};

use crate::app::WindowHandle;
use crate::fonts::Fonts;
use crate::input::{Key, MouseButton};
use crate::settings::WindowConfig;
use crate::ui::Ui;
use crate::window::BentoWindow;

pub struct Runner {
    pub ctx: RenderContext,
    pub fonts: Fonts,
    pub windows: HashMap<WindowId, BentoWindow>,
    pending: Vec<(WindowHandle, WindowConfig, Ui)>,
    close_queue: Vec<WindowHandle>,
    handle_to_id: HashMap<WindowHandle, WindowId>,
}

impl Runner {
    pub fn new(
        ctx: RenderContext,
        fonts: Fonts,
        pending: Vec<(WindowHandle, WindowConfig, Ui)>,
    ) -> Self {
        Self {
            ctx,
            fonts,
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
        ui: Ui,
    ) {
        let win = BentoWindow::new(&self.ctx, event_loop, config, ui);
        let id = win.id();
        self.handle_to_id.insert(handle, id);
        self.windows.insert(id, win);
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
        for (handle, config, ui) in pending {
            self.open_window(event_loop, handle, config, ui);
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
                crate::dispatch::dispatch(&mut win.ui, &win.input);

                win.ui.drain_events();

                win.ui.update(&mut self.fonts);

                let clear = win.config.clear_color.to_array();

                // DEBUG
                // println!("nodes: {}", win.ui.scene.nodes.len());

                win.renderer.render(
                    &mut self.ctx,
                    &mut self.fonts.font_system,
                    &mut win.surface,
                    &mut win.ui.scene,
                    clear,
                );

                // DEBUG
                /*
                println!(
                    "uploads: {} culled_texts: {} culled_rects: {}",
                    win.renderer.stats.rect_uploads,
                    win.renderer.stats.texts_culled,
                    win.renderer.stats.rects_culled
                );
                */

                win.input.reset();
            }

            WindowEvent::CursorMoved { position, .. } => {
                let scale = win.window.scale_factor() as f32;
                win.input
                    .mouse
                    .on_move(position.x as f32 / scale, position.y as f32 / scale);
                win.window.request_redraw();
            }

            WindowEvent::MouseInput { button, state, .. } => {
                let btn = match button {
                    winit::event::MouseButton::Left => MouseButton::Left,
                    winit::event::MouseButton::Right => MouseButton::Right,
                    winit::event::MouseButton::Middle => MouseButton::Middle,
                    _ => return,
                };
                match state {
                    ElementState::Pressed => win.input.mouse.on_press(&btn),
                    ElementState::Released => win.input.mouse.on_release(&btn),
                }
                win.window.request_redraw();
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x, -y),
                    MouseScrollDelta::PixelDelta(pos) => {
                        (pos.x as f32 / 40.0, -pos.y as f32 / 40.0)
                    }
                };
                win.input.mouse.on_scroll(dx, dy);
                win.window.request_redraw();
            }

            WindowEvent::KeyboardInput { event: ke, .. } => {
                let key = match ke.physical_key {
                    PhysicalKey::Code(code) => Key::from(code),
                    PhysicalKey::Unidentified(_) => Key::Unknown,
                };
                let text = ke.text.as_ref().and_then(|t| t.chars().next());
                match ke.state {
                    ElementState::Pressed => win.input.keyboard.on_press(key, text),
                    ElementState::Released => win.input.keyboard.on_release(key),
                }
                win.window.request_redraw();
            }

            WindowEvent::ModifiersChanged(mods) => {
                let s = mods.state();
                win.input.keyboard.modifiers = crate::input::Modifiers {
                    shift: s.shift_key(),
                    ctrl: s.control_key(),
                    cmd: s.alt_key(),
                    super_key: s.super_key(),
                };
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
                self.handle_to_id.retain(|_, v| *v != id);
                if self.windows.is_empty() {
                    event_loop.exit();
                }
            }

            _ => {}
        }
    }
}
