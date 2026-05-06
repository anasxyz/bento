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

        let mut text = TextNode::new(
            "في صباحٍ هادئ، كانت الشمس تتسلل بلطفٍ بين أغصان الأشجار، تنشر دفئها على الأرض وتوقظ الحياة من سكونها. خرج الناس إلى أعمالهم بابتساماتٍ خفيفة، يحملون آمالهم الصغيرة ليومٍ جديد، بينما كانت الطيور تغرد بألحانٍ عذبة تضيف إلى اللحظة جمالًا لا يُوصف.",
            0.0,
            0.0,
            32.0,
        );
        text.color([0.1, 0.1, 0.1, 1.0])
            .max_width(800.0)
            .align(TextAlign::Right);

        // alternating colors throughout
        text.add_color(0, 2, [0.8, 0.1, 0.1, 1.0]); // red
        text.add_color(3, 8, [0.1, 0.5, 0.8, 1.0]); // blue
        text.add_color(9, 14, [0.1, 0.7, 0.3, 1.0]); // green
        text.add_color(15, 20, [0.8, 0.5, 0.0, 1.0]); // orange
        text.add_color(21, 26, [0.6, 0.0, 0.8, 1.0]); // purple
        text.add_color(27, 32, [0.8, 0.1, 0.4, 1.0]); // pink
        text.add_color(33, 38, [0.0, 0.6, 0.6, 1.0]); // teal
        text.add_color(39, 44, [0.9, 0.7, 0.0, 1.0]); // yellow
        text.add_color(45, 57, [0.2, 0.3, 0.9, 1.0]); // indigo

        // yellow background on "كانت الشمس"
        text.add_background(16, 26, [1.0, 0.85, 0.0, 0.35]);

        // blue background + strikethrough on "تتسلل بلطفٍ"
        text.add_background(27, 38, [0.2, 0.4, 1.0, 0.3]);
        text.add_strikethrough(27, 38, [0.2, 0.4, 1.0, 1.0]);

        // bold on "أغصان الأشجار"
        text.add_weight(44, 57, 700);

        // green color + underline on "تنشر دفئها"
        text.add_color(59, 68, [0.1, 0.6, 0.2, 1.0]);
        text.add_underline(59, 68, [0.1, 0.6, 0.2, 1.0]);

        // more color changes in second half
        text.add_color(70, 78, [0.8, 0.2, 0.2, 1.0]); // red
        text.add_color(79, 86, [0.0, 0.5, 0.7, 1.0]); // cyan

        // red background + white text on "الحياة"
        text.add_background(80, 86, [0.8, 0.1, 0.1, 1.0]);
        text.add_color(80, 86, [1.0, 1.0, 1.0, 1.0]);

        text.add_color(87, 95, [0.4, 0.8, 0.0, 1.0]); // lime
        text.add_color(96, 105, [0.9, 0.3, 0.0, 1.0]); // burnt orange
        text.add_color(106, 115, [0.3, 0.0, 0.7, 1.0]); // deep purple

        // overlapping backgrounds
        text.add_background(100, 120, [0.2, 0.5, 1.0, 0.2]);
        text.add_background(110, 130, [1.0, 0.5, 0.0, 0.25]);

        // bold + italic + color on "آمالهم الصغيرة"
        text.add_weight(135, 149, 900);
        text.add_italic(135, 149);
        text.add_color(135, 149, [0.6, 0.0, 0.8, 1.0]);

        text.add_color(150, 158, [0.0, 0.7, 0.5, 1.0]); // emerald
        text.add_color(159, 167, [0.8, 0.0, 0.3, 1.0]); // crimson

        // underline + strikethrough on same range
        text.add_underline(160, 175, [0.0, 0.0, 0.0, 1.0]);
        text.add_strikethrough(160, 175, [0.8, 0.0, 0.0, 0.8]);

        text.add_color(176, 185, [0.1, 0.4, 0.9, 1.0]); // royal blue
        text.add_color(186, 195, [0.7, 0.5, 0.0, 1.0]); // gold
        text.add_color(196, 205, [0.0, 0.6, 0.4, 1.0]); // sea green

        // bold + underline on last section
        text.add_weight(200, 220, 700);
        text.add_underline(200, 220, [0.5, 0.0, 0.8, 1.0]);
        text.add_color(200, 220, [0.5, 0.0, 0.2, 1.0]);

        text.add_color(221, 235, [0.9, 0.1, 0.1, 1.0]); // bright red
        text.add_color(236, 250, [0.0, 0.5, 0.9, 1.0]); // sky blue

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
