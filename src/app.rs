use pollster;
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

// use crate::layout::layout_tree;
use crate::render::gpu::GpuContext;
use crate::settings::WindowConfig;
use crate::window::WindowState;

pub struct AppWindow {
    settings: WindowConfig,
}

impl AppWindow {
    pub fn new(settings: WindowConfig) -> Self {
        Self { settings }
    }

    pub fn run<F>(self, update: F)
    where
        F: FnMut(),
    {
        let event_loop = EventLoop::new().unwrap();
        event_loop
            .run_app(&mut Runner {
                update,
                win: None,
                settings: self.settings,
            })
            .unwrap();
    }
}

struct Runner<F: FnMut()> {
    update: F,
    win: Option<WindowState>,
    settings: WindowConfig,
}

impl<F: FnMut()> ApplicationHandler for Runner<F> {
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
                // call user update

                win.begin();

                let size = win.window.inner_size();
                let scale = win.window.scale_factor() as f32;
                let logical_w = size.width as f32 / scale;
                let logical_h = size.height as f32 / scale;


                win.render();
                win.mouse.reset();
            }
            WindowEvent::CursorMoved { position, .. } => {
                let Some(win) = self.win.as_mut() else { return };
                win.mouse.x = position.x as f32;
                win.mouse.y = position.y as f32;
                win.mouse.update_drag();
                win.window.request_redraw();
            }
            WindowEvent::MouseInput { button, state, .. } => {
                let Some(win) = self.win.as_mut() else { return };
                match button {
                    winit::event::MouseButton::Left => match state {
                        ElementState::Pressed => {
                            win.mouse.left_pressed = true;
                            win.mouse.left_just_pressed = true;
                            win.mouse.left_click_x = win.mouse.x;
                            win.mouse.left_click_y = win.mouse.y;
                        }
                        ElementState::Released => {
                            win.mouse.left_pressed = false;
                            win.mouse.left_just_released = true;
                        }
                    },
                    winit::event::MouseButton::Right => match state {
                        ElementState::Pressed => {
                            win.mouse.right_pressed = true;
                            win.mouse.right_just_pressed = true;
                            win.mouse.right_click_x = win.mouse.x;
                            win.mouse.right_click_y = win.mouse.y;
                        }
                        ElementState::Released => {
                            win.mouse.right_pressed = false;
                        }
                    },
                    winit::event::MouseButton::Middle => match state {
                        ElementState::Pressed => {
                            win.mouse.middle_pressed = true;
                            win.mouse.middle_just_pressed = true;
                            win.mouse.middle_click_x = win.mouse.x;
                            win.mouse.middle_click_y = win.mouse.y;
                        }
                        ElementState::Released => {
                            win.mouse.middle_pressed = false;
                        }
                    },
                    _ => {}
                }
                win.window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                let Some(win) = self.win.as_mut() else { return };
                let scale = win.window.scale_factor() as f32;
                win.gpu.resize(size.width, size.height);
                win.draw.resize(size.width as f32 / scale, size.height as f32 / scale);
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let Some(win) = self.win.as_mut() else { return };
                let size = win.window.inner_size();
                let scale = scale_factor as f32;
                win.gpu.resize(size.width, size.height);
                win.draw.set_scale(scale, size.width as f32 / scale, size.height as f32 / scale);
            }
            WindowEvent::CloseRequested => {
                self.win = None;
                event_loop.exit();
            }
            _ => {}
        }
    }
}
