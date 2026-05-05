#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use bento_wgpu::math::transform;
use bento_wgpu::{RectInstance, RectNode, RenderContext, Scene, TextNode, ColorSpan};
use cosmic_text::FontSystem;
use std::sync::Arc;
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

struct App {
    ctx: RenderContext,
    window: Option<Arc<Window>>,
    surface: Option<bento_wgpu::Surface<'static>>,
    renderer: Option<bento_wgpu::Renderer>,
    scene: Scene,
    font_system: cosmic_text::FontSystem,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("demo")
                        .with_inner_size(winit::dpi::LogicalSize::new(800u32, 600u32)),
                )
                .unwrap(),
        );

        let size = window.inner_size();
        let scale = window.scale_factor() as f32;
        let w = size.width as f32 / scale;
        let h = size.height as f32 / scale;

        let surface = bento_wgpu::Surface::new(&self.ctx, Arc::clone(&window), w, h, scale);
        let renderer = bento_wgpu::Renderer::new(&self.ctx, &surface);
        self.window = Some(window);
        self.surface = Some(surface);
        self.renderer = Some(renderer);

        /*
        let mut rect = RectNode::new(60.0, 20.0, 200.0, 100.0);
        rect.color = [0.2, 0.5, 1.0, 1.0];
        rect.radii = [0.0; 4];
        rect.border_color = [0.0, 0.0, 0.0, 1.0];
        rect.border_widths = [3.0; 4];
        // rect.radii = [14.0; 4];
        // rect.scale_x = 5.0;
        // rect.scale_y = 5.0;
        rect.z = 1;
        self.scene.add_rect(rect);
        */

        let mut text = TextNode::new("Enter your name 💁👌🎍😍 Password", 60.0, 60.0, 16.0);
        text.scale_x = 1.0;
        text.scale_y = 1.0;
        text.weight = 400;
        text.font_family = "Times New Roman".to_string();
        text.z = 3;
        text.color_spans.push(ColorSpan { start: 0, end: 29, color: [0.0, 0.0, 0.0, 1.0] });
        self.scene.add_text(text);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let (Some(renderer), Some(surface)) = (self.renderer.as_mut(), self.surface.as_mut())
        else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                renderer.render(
                    &mut self.ctx,
                    &mut self.font_system,
                    surface,
                    [1.0, 1.0, 1.0, 1.0],
                    &mut self.scene,
                );
            }

            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                let window = self.window.as_ref().unwrap();
                let size = window.inner_size();
                let scale = window.scale_factor() as f32;
                let w = size.width as f32 / scale;
                let h = size.height as f32 / scale;
                surface.resize(&self.ctx, w, h, scale);
                renderer.resize(&self.ctx, surface);
                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}

fn main() {
    let ctx = pollster::block_on(RenderContext::new());
    let font_system = FontSystem::new();
    let event_loop = EventLoop::new().unwrap();
    event_loop
        .run_app(&mut App {
            ctx: ctx,
            window: None,
            surface: None,
            renderer: None,
            scene: Scene::new(),
            font_system: cosmic_text::FontSystem::new(),
        })
        .unwrap();
}
