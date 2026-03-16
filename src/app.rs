use pollster;
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::draw::draw_tree;
use crate::event::Event;
use crate::events::fire_events;
use crate::keyboard::{Key, Modifiers};
use crate::layout::layout_tree;
use crate::render::gpu::GpuContext;
use crate::settings::WindowConfig;
use crate::ui::Ui;
use crate::window::WindowState;

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
    settings: WindowConfig,
    modifiers: Modifiers,
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
        let gpu = pollster::block_on(GpuContext::new(window.clone()));
        self.win = Some(WindowState::new(window, gpu, self.settings.clear_color));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        event_loop.set_control_flow(ControlFlow::Wait);
        let Some(win) = self.win.as_mut() else { return };

        match event {
            WindowEvent::RedrawRequested => {
                fire_events(&mut self.ui);
                self.ui.drain_events();
                (self.update)(&mut self.ui);
                win.begin();
                layout_tree(&mut self.ui);
                draw_tree(&self.ui, &mut win.draw);
                win.render();
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
                let Some(win) = self.win.as_mut() else { return };
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
                        let ev = Event::KeyPress { key: key.clone(), mods: mods.clone(), text };
                        if let Some(f) = focused {
                            self.ui.get_any_mut(f).and_then(|e| e.on_key_press(key.clone(), mods.clone(), text));
                            self.ui.emit_bubbling(f, ev.clone());
                        }
                        self.ui.emit(global, ev);
                    }
                    ElementState::Released => {
                        let ev = Event::KeyRelease { key: key.clone(), mods: mods.clone() };
                        if let Some(f) = focused {
                            self.ui.get_any_mut(f).and_then(|e| e.on_key_release(key.clone(), mods.clone()));
                            self.ui.emit_bubbling(f, ev.clone());
                        }
                        self.ui.emit(global, ev);
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
                let Some(win) = self.win.as_mut() else { return };
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
                win.resize_and_rescale();
                self.ui.window_width =
                    win.window.inner_size().width / win.window.scale_factor() as u32;
                self.ui.window_height =
                    win.window.inner_size().height / win.window.scale_factor() as u32;
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                win.resize_and_rescale();
            }
            WindowEvent::CloseRequested => {
                self.win = None;
                event_loop.exit();
            }
            _ => {}
        }
    }
}
