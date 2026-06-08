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
    Redraw,
    AsyncCallback,
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

    pub fn run(view: impl bento_ui::View + 'static) {
        let event_loop = EventLoop::<BentoEvent>::with_user_event().build().unwrap();

        // redraw callback
        // wakes event loop when a signal changes
        let proxy = event_loop.create_proxy();
        // pass a redraw callback into Ui so bento_ui stays independent of bento_winit
        // when a signal changes, Ui calls this closure which wakes up the event loop
        let ui = bento_ui::Ui::new(view, move || {
            proxy.send_event(BentoEvent::Redraw).ok();
        });

        // async waker
        // wakes event loop when a spawned future completes
        let proxy = event_loop.create_proxy();
        bento_ui::set_waker(Arc::new(move || {
            proxy.send_event(BentoEvent::AsyncCallback).ok();
        }));

        // async spawner
        // runs futures on tokio thread pool on native, or wasm_bindgen_futures on the web
        #[cfg(not(target_arch = "wasm32"))]
        {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            bento_ui::set_spawner(move |fut| {
                runtime.spawn(fut);
            });
        }
        #[cfg(target_arch = "wasm32")]
        {
            bento_ui::set_spawner(move |fut| {
                wasm_bindgen_futures::spawn_local(fut);
            });
        }

        let mut app = App::new();
        app.pending.push((WindowConfig::default(), ui));
        event_loop.run_app(&mut app).unwrap();
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
            win.ui.set_viewport(win.surface.width, win.surface.height);
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
                let t = web_time::Instant::now();
                win.ui.process_input();

                if win.needs_render() {
                    let draw_list = win.ui.draw();
                    win.renderer.render(
                        ctx,
                        &mut win.ui.measurer,
                        &mut win.surface,
                        win.config.clear_color,
                        &draw_list,
                    );
                    win.needs_render = false;
                }

                win.ui.input.keyboard.clear();
                win.ui.input.mouse.clear();
                // println!("[winit] total frame time {:?}", t.elapsed());
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
                        win.ui.input.mouse.scroll_x += x * 20.0;
                        win.ui.input.mouse.scroll_y += y * 20.0;
                    }
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        let scale = win.surface.scale;
                        #[cfg(target_arch = "wasm32")]
                        {
                            win.ui.input.mouse.scroll_x = (pos.x as f32 / scale) / 6.0;
                            win.ui.input.mouse.scroll_y = (pos.y as f32 / scale) / 6.0;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            win.ui.input.mouse.scroll_x = pos.x as f32 / scale;
                            win.ui.input.mouse.scroll_y = pos.y as f32 / scale;
                        }
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

                win.request_redraw();
            }
            WindowEvent::CursorLeft { .. } => {
                win.ui.input.mouse.inside_window = false;
                win.ui.input.mouse.just_left = true;

                win.request_redraw();
            }

            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                win.resize(ctx);
                win.ui.set_viewport(win.surface.width, win.surface.height);
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
            BentoEvent::Redraw => {
                for win in self.windows.values_mut() {
                    win.needs_render = true;
                    win.request_redraw();
                }
            }
            BentoEvent::AsyncCallback => {
                for cb in bento_ui::drain_callbacks() {
                    cb();
                }
                for win in self.windows.values_mut() {
                    win.needs_render = true;
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
