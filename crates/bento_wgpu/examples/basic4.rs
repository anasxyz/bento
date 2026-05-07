#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use bento_wgpu::{GroupNode, ImageNode, RectNode, RenderContext, Scene, TextAlign, TextNode};
use cosmic_text::FontSystem;
use std::sync::Arc;
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
    font_system: FontSystem,
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

        let mut group = GroupNode::new();
        group.clip(0.0, 0.0, 200.0, 500.0);
        let mut text = TextNode::new("hello", 0.0, 0.0, 80.0);
        text.color([0.1, 0.1, 0.1, 1.0]);
        text.font_family("Times New Roman");
        text.add_background(0, 5, [0.8, 0.1, 0.1, 0.4]);
        text.line_height(200.0);
        group.add_text(text);

        self.scene.add_group(group);
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
            ctx,
            window: None,
            surface: None,
            renderer: None,
            scene: Scene::new(),
            font_system,
        })
        .unwrap();
}

