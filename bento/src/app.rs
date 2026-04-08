use bento_wgpu::{ImageKey, RenderContext};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::WindowId,
};

use crate::fonts::Fonts;
use crate::images::ImageManager;
use crate::input::cursor::map_cursor;
use crate::input::{Key, MouseButton};
use crate::settings::WindowConfig;
use crate::ui::Ui;
use crate::window::BentoWindow;

const BLINK_MS: u64 = 500;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowHandle(pub(crate) u32);

pub struct App {
    pub fonts: Fonts,
    images: ImageManager,

    pending: Vec<(WindowHandle, WindowConfig, Ui)>,
    next_handle: u32,

    ctx: Option<RenderContext>,
    windows: HashMap<WindowId, BentoWindow>,
    handle_to_id: HashMap<WindowHandle, WindowId>,
    close_queue: Vec<WindowId>,
}

impl App {
    pub fn new() -> Self {
        Self {
            fonts: Fonts::new(),
            images: ImageManager::new(),
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

    pub fn load_image(&mut self, path: &str) -> ImageKey {
        self.images.load_image(path)
    }

    pub fn load_image_svg(&mut self, path: &str, width: u32, height: u32) -> ImageKey {
        self.images.load_image_svg(path, width, height)
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
        let mut win = BentoWindow::new(ctx, event_loop, config, ui);
        self.images.flush(&mut win.renderer, ctx);
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
        if self.ctx.is_none() {
            let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
            self.ctx = Some(rt.block_on(RenderContext::new()));
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
            WindowEvent::RedrawRequested => {
                win.ui.update(&mut self.fonts);

                let clear = win.config.clear_color.to_array();
                let ctx = self.ctx.as_mut().unwrap();
                win.renderer.render(
                    ctx,
                    &mut self.fonts.font_system,
                    &mut win.surface,
                    &mut win.ui.scene,
                    clear,
                );

                schedule_blink(win, event_loop);
            }

            WindowEvent::CursorMoved { position, .. } => {
                let scale = win.window.scale_factor() as f32;
                win.input
                    .mouse
                    .on_move(position.x as f32 / scale, position.y as f32 / scale);
                let old_hovered = win.ui.interaction.hovered;
                let old_pressed = win.ui.interaction.pressed;
                crate::dispatch::dispatch(&mut win.ui, &win.input);
                if win.ui.interaction.pressed.is_some() {
                    reset_blink(win);
                }
                win.ui.drain_events();
                win.input.reset();
                let any_dirty = win.ui.slots.iter().any(|s| {
                    s.as_ref()
                        .map(|s| s.widget.base().render_dirty)
                        .unwrap_or(false)
                });
                if win.ui.interaction.hovered != old_hovered || old_pressed.is_some() || any_dirty {
                    win.window.request_redraw();
                }
                win.sync_cursor();
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
                crate::dispatch::dispatch(&mut win.ui, &win.input);
                if win.ui.interaction.pressed.is_some() {
                    reset_blink(win);
                }
                win.ui.drain_events();
                win.input.reset();
                win.sync_cursor();
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
                crate::dispatch::dispatch(&mut win.ui, &win.input);
                win.ui.drain_events();
                win.input.reset();
                win.sync_cursor();
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
                crate::dispatch::dispatch(&mut win.ui, &win.input);
                win.ui.drain_events();
                win.input.reset();
                win.sync_cursor();
                stop_blink(win);
                win.window.request_redraw();
            }

            WindowEvent::ModifiersChanged(mods) => {
                let s = mods.state();
                win.input.keyboard.modifiers = crate::input::Modifiers {
                    shift: s.shift_key(),
                    ctrl: s.control_key(),
                    cmd: s.control_key(),
                    alt: s.alt_key(),
                    super_key: s.super_key(),
                };
            }

            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                let ctx = self.ctx.as_ref().unwrap();
                win.resize(ctx);
            }

            WindowEvent::CloseRequested => {
                self.close_queue.push(id);
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        for (_, win) in &mut self.windows {
            tick_blink(win, event_loop, now);
        }
    }
}

fn schedule_blink(win: &mut BentoWindow, event_loop: &ActiveEventLoop) {
    if win.ui.has_focused_text_widget() {
        if win.blink_deadline.is_none() {
            win.blink_deadline = Some(Instant::now() + Duration::from_millis(BLINK_MS));
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(win.blink_deadline.unwrap()));
    } else {
        win.blink_deadline = None;
        event_loop.set_control_flow(ControlFlow::Wait);
    }
}

fn reset_blink(win: &mut BentoWindow) {
    win.blink_deadline = Some(Instant::now() + Duration::from_millis(BLINK_MS));
}

fn stop_blink(win: &mut BentoWindow) {
    win.blink_deadline = None;
}

fn tick_blink(win: &mut BentoWindow, event_loop: &ActiveEventLoop, now: Instant) {
    if let Some(deadline) = win.blink_deadline {
        if now >= deadline {
            win.ui.toggle_cursor_blink();
            win.window.request_redraw();
            win.blink_deadline = Some(Instant::now() + Duration::from_millis(BLINK_MS));
            event_loop.set_control_flow(ControlFlow::WaitUntil(win.blink_deadline.unwrap()));
        }
    }
}
