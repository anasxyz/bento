use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::event::Event;
use crate::event::fire_events;
use crate::input::{Key, Modifiers};
use crate::layout::layout_tree;
use crate::render::{Renderer, WindowState};
use crate::settings::WindowConfig;
use crate::ui::Ui;

pub struct AppWindow {
    settings: WindowConfig,
}

impl AppWindow {
    pub fn new(settings: WindowConfig) -> Self {
        Self { settings }
    }

    pub fn run<F>(self, ui: Ui, update: F)
    where
        F: FnMut(&mut Ui),
    {
        let event_loop = EventLoop::new().unwrap();
        event_loop
            .run_app(&mut Runner {
                ui,
                update,
                win: None,
                renderer: None,
                settings: self.settings,
                modifiers: Modifiers::default(),
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
                fire_events(&mut self.ui);
                self.ui.drain_events();
                (self.update)(&mut self.ui);

                if self.ui.any_dirty() {
                    if self.ui.draw_list_dirty {
                        self.ui.draw_list_dirty = false;
                        renderer.invalidate();
                        self.ui.mark_all_dirty();
                    }

                    layout_tree(&mut self.ui);
                    let region = self.ui.dirty_region();
                    renderer.paint(&self.ui, win.clear_color.to_array());
                    self.ui.clear_dirty();
                    win.present(renderer, region, true);
                } else {
                    win.present(renderer, None, false);
                }

                self.ui.mouse.reset();
            }
            WindowEvent::ModifiersChanged(mods) => {
                let state = mods.state();
                self.modifiers = Modifiers {
                    shift: state.shift_key(),
                    ctrl: state.control_key(),
                    alt: state.alt_key(),
                    super_key: state.super_key(),
                };
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
                            self.ui
                                .get_any_mut(f)
                                .map(|e| e.on_key_press(key.clone(), mods.clone(), text));
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
                            self.ui
                                .get_any_mut(f)
                                .map(|e| e.on_key_release(key.clone(), mods.clone()));
                            self.ui.emit_bubbling(f, ev);
                        } else {
                            self.ui.emit(global, ev);
                        }
                    }
                }
                win.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                let scale = win.window.scale_factor() as f32;
                self.ui.mouse.x = position.x as f32 / scale;
                self.ui.mouse.y = position.y as f32 / scale;
                win.request_redraw();
            }
            WindowEvent::MouseInput { button, state, .. } => {
                match button {
                    winit::event::MouseButton::Left => match state {
                        ElementState::Pressed => self.ui.mouse.on_left_press(),
                        ElementState::Released => self.ui.mouse.on_left_release(),
                    },
                    winit::event::MouseButton::Right => match state {
                        ElementState::Pressed => self.ui.mouse.on_right_press(),
                        ElementState::Released => self.ui.mouse.on_right_release(),
                    },
                    winit::event::MouseButton::Middle => match state {
                        ElementState::Pressed => self.ui.mouse.on_middle_press(),
                        ElementState::Released => self.ui.mouse.on_middle_release(),
                    },
                    _ => {}
                }
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
