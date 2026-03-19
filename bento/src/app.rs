use crate::event::{Event, fire_events};
use crate::input::{Key, Modifiers};
use crate::layout::layout_tree;
use crate::render::{Renderer, WindowState};
use crate::settings::WindowConfig;
use crate::ui::Ui;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

pub struct AppWindow {
    settings: WindowConfig,
}

impl AppWindow {
    pub fn new(settings: WindowConfig) -> Self {
        Self { settings }
    }

    pub fn run<F: FnMut(&mut Ui)>(self, ui: Ui, update: F) {
        let event_loop = EventLoop::new().unwrap();
        event_loop
            .run_app(&mut Runner {
                ui,
                update,
                win: None,
                renderer: None,
                settings: self.settings,
                modifiers: Modifiers::default(),
                last_frame: std::time::Instant::now(),
            })
            .unwrap();
    }
}

struct Runner<F: FnMut(&mut Ui)> {
    ui: Ui,
    update: F,
    win: Option<WindowState>,
    renderer: Option<Renderer>,
    settings: WindowConfig,
    modifiers: Modifiers,
    last_frame: std::time::Instant,
}

impl<F: FnMut(&mut Ui)> Runner<F> {
    fn sync_window_size(&mut self) {
        let (Some(win), Some(renderer)) = (self.win.as_mut(), self.renderer.as_mut()) else {
            return;
        };
        win.resize_and_rescale(renderer);
        let scale = win.window.scale_factor() as f32;
        self.ui.window_width = (win.window.inner_size().width as f32 / scale) as u32;
        self.ui.window_height = (win.window.inner_size().height as f32 / scale) as u32;
        self.ui.mark_all_dirty();
        win.request_redraw();
    }
}

impl<F: FnMut(&mut Ui)> ApplicationHandler for Runner<F> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&self.settings.title)
                        .with_inner_size(winit::dpi::LogicalSize::new(
                            self.settings.width,
                            self.settings.height,
                        )),
                )
                .unwrap(),
        );
        let (win, renderer) = WindowState::create(window, self.settings.clear_color);
        self.win = Some(win);
        self.renderer = Some(renderer);
        self.sync_window_size();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        event_loop.set_control_flow(ControlFlow::Wait);
        let (Some(win), Some(renderer)) = (self.win.as_mut(), self.renderer.as_mut()) else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                let now = std::time::Instant::now();
                let dt = now.duration_since(self.last_frame).as_secs_f32().min(0.1);
                self.last_frame = now;

                // process input and user update
                fire_events(&mut self.ui);
                self.ui.drain_events();
                (self.update)(&mut self.ui);
                crate::element::scroll::tick_scroll_containers(&mut self.ui, dt);

                // render if anything changed
                if self.ui.any_dirty() {
                    if self.ui.draw_list_dirty {
                        self.ui.draw_list_dirty = false;
                        renderer.invalidate();
                        self.ui.mark_all_dirty();
                    }
                    layout_tree(&mut self.ui);
                    crate::element::scroll::sync_scroll_containers(&mut self.ui);
                    let region = self.ui.dirty_region();
                    renderer.paint(&self.ui, win.clear_color.to_array());
                    self.ui.clear_dirty();
                    win.present(renderer, region, true);
                } else {
                    win.present(renderer, None, false);
                }

                if crate::element::scroll::is_scroll_animating(&self.ui) {
                    win.request_redraw();
                }

                self.ui.mouse.reset();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                let key = match event.physical_key {
                    PhysicalKey::Code(code) => Key::from(code),
                    PhysicalKey::Unidentified(_) => Key::Unknown,
                };
                let text = event.text.as_ref().and_then(|t| t.chars().next());
                let mods = self.modifiers.clone();
                let focused = self.ui.interaction.focused;
                let global = self.ui.global();

                match event.state {
                    ElementState::Pressed => {
                        let ev = Event::KeyPress {
                            key: key.clone(),
                            mods: mods.clone(),
                            text,
                        };
                        if let Some(f) = focused {
                            let (k, m) = (key.clone(), mods.clone());
                            self.ui.with_element(f, |el, ui| {
                                el.on_key_press(ui, f, k, m, text);
                            });
                            self.ui.emit_bubbling(f, ev);
                        } else {
                            self.ui.emit(global, ev);
                        }
                    }
                    ElementState::Released => {
                        let ev = Event::KeyRelease {
                            key: key.clone(),
                            mods: mods.clone(),
                        };
                        if let Some(f) = focused {
                            let (k, m) = (key.clone(), mods.clone());
                            self.ui.with_element(f, |el, ui| {
                                el.on_key_release(ui, f, k, m);
                            });
                            self.ui.emit_bubbling(f, ev);
                        } else {
                            self.ui.emit(global, ev);
                        }
                    }
                }
                win.request_redraw();
            }

            WindowEvent::ModifiersChanged(mods) => {
                let s = mods.state();
                self.modifiers = Modifiers {
                    shift: s.shift_key(),
                    ctrl: s.control_key(),
                    alt: s.alt_key(),
                    super_key: s.super_key(),
                };
            }

            WindowEvent::CursorMoved { position, .. } => {
                let scale = win.window.scale_factor() as f32;
                self.ui.mouse.x = position.x as f32 / scale;
                self.ui.mouse.y = position.y as f32 / scale;
                win.request_redraw();
            }

            WindowEvent::MouseInput { button, state, .. } => {
                match (button, state) {
                    (winit::event::MouseButton::Left, ElementState::Pressed) => {
                        self.ui.mouse.on_left_press()
                    }
                    (winit::event::MouseButton::Left, ElementState::Released) => {
                        self.ui.mouse.on_left_release()
                    }
                    (winit::event::MouseButton::Right, ElementState::Pressed) => {
                        self.ui.mouse.on_right_press()
                    }
                    (winit::event::MouseButton::Right, ElementState::Released) => {
                        self.ui.mouse.on_right_release()
                    }
                    (winit::event::MouseButton::Middle, ElementState::Pressed) => {
                        self.ui.mouse.on_middle_press()
                    }
                    (winit::event::MouseButton::Middle, ElementState::Released) => {
                        self.ui.mouse.on_middle_release()
                    }
                    _ => {}
                }
                win.request_redraw();
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x, -y),
                    MouseScrollDelta::PixelDelta(pos) => {
                        (pos.x as f32 / 40.0, -pos.y as f32 / 40.0)
                    }
                };
                self.ui.mouse.on_scroll(dx, dy);
                win.request_redraw();
            }

            WindowEvent::Resized(_) => {
                self.sync_window_size();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                self.sync_window_size();
            }
            WindowEvent::CloseRequested => {
                self.win = None;
                self.renderer = None;
                event_loop.exit();
            }
            _ => {}
        }
    }
}
