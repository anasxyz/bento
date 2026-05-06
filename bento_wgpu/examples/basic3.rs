#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use bento_wgpu::{GroupNode, ImageNode, RectNode, RenderContext, Scene, TextNode};
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

        let mut text = TextNode::new(
            "The quick brown fox jumps over the lazy dog and then some more words follow here",
            60.0,
            60.0,
            20.0,
        );
        text.color([0.3, 0.3, 0.3, 1.0])
            .font_family("Times New Roman")
            .max_width(350.0)
            .z(1);

        // red + bold on "quick"
        text.add_color(4, 9, [0.8, 0.1, 0.1, 1.0]);
        text.add_weight(4, 9, 700);

        // blue background + italic on "brown fox"
        text.add_background(10, 19, [0.2, 0.5, 1.0, 0.3]);
        text.add_italic(10, 19);

        // yellow background + underline on "jumps over"
        text.add_background(20, 30, [1.0, 0.85, 0.0, 0.4]);
        text.add_underline(20, 30, [0.8, 0.6, 0.0, 1.0]);

        // green + bold + italic on "the lazy"
        text.add_color(31, 39, [0.1, 0.6, 0.2, 1.0]);
        text.add_weight(31, 39, 700);
        text.add_italic(31, 39);

        // red background + white text + strikethrough on "dog"
        text.add_background(40, 43, [0.8, 0.1, 0.1, 1.0]);
        text.add_color(40, 43, [1.0, 1.0, 1.0, 1.0]);
        text.add_strikethrough(40, 43, [1.0, 1.0, 1.0, 1.0]);

        // different font + underline on "and then"
        text.add_font_family(44, 52, "Georgia");
        text.add_underline(44, 52, [0.4, 0.0, 0.8, 1.0]);

        // heavy weight + color on "some more"
        text.add_weight(53, 62, 900);
        text.add_color(53, 62, [0.9, 0.4, 0.0, 1.0]);

        // overlapping background ranges — blue under orange
        text.add_background(58, 70, [0.2, 0.5, 1.0, 0.2]);
        text.add_background(65, 75, [1.0, 0.5, 0.0, 0.3]);

        // underline + strikethrough on same range "words"
        text.add_underline(69, 74, [0.0, 0.0, 0.0, 1.0]);
        text.add_strikethrough(69, 74, [0.8, 0.0, 0.0, 0.8]);

        // fade out the end with opacity handled at node level
        // last section bold + italic + colored
        text.add_weight(75, 82, 700);
        text.add_italic(75, 82);
        text.add_color(75, 82, [0.5, 0.0, 0.8, 1.0]);
        text.add_underline(75, 82, [0.5, 0.0, 0.8, 1.0]);

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
            ctx,
            window: None,
            surface: None,
            renderer: None,
            scene: Scene::new(),
            font_system,
        })
        .unwrap();
}

