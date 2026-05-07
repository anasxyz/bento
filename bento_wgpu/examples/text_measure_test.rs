#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use bento_shared::{
    measurer::CosmicTextMeasurer,
    measure::{TextMeasurer, TextMeasureRequest},
    scene::{FontFamilyRange, ItalicRange, WeightRange},
};
use bento_wgpu::{RectNode, RenderContext, Scene, TextAlign, TextNode};
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

        let mut measurer = CosmicTextMeasurer::new(&mut self.font_system);

        // --- test 1: plain single line ---
        // green border rect should tightly wrap "Hello world"
        {
            let text = "Hello world";
            let x = 40.0;
            let y = 40.0;
            let size = 24.0;

            let result = measurer.measure(TextMeasureRequest {
                text,
                font_family: "",
                size,
                weight: 400,
                italic: false,
                letter_spacing: 0.0,
                line_height: None,
                max_width: None,
                weight_ranges: &[],
                italic_ranges: &[],
                font_family_ranges: &[],
            });

            println!("test 1 — plain: w={:.1} h={:.1} lines={}", result.width, result.height, result.line_count);

            let mut border = RectNode::new(x, y, result.width, result.height);
            border.border_color([0.0, 0.8, 0.0, 1.0]);
            border.border_width(1.0);
            border.color([0.0; 4]);
            border.z(2);
            self.scene.add_rect(border);

            let mut t = TextNode::new(text, x, y, size);
            t.color([0.1, 0.1, 0.1, 1.0]);
            t.z(3);
            self.scene.add_text(t);
        }

        // --- test 2: wrapped text ---
        // green border should wrap all lines, not overflow
        {
            let text = "The quick brown fox jumps over the lazy dog";
            let x = 40.0;
            let y = 120.0;
            let size = 18.0;
            let max_width = 200.0;

            let result = measurer.measure(TextMeasureRequest {
                text,
                font_family: "",
                size,
                weight: 400,
                italic: false,
                letter_spacing: 0.0,
                line_height: None,
                max_width: Some(max_width),
                weight_ranges: &[],
                italic_ranges: &[],
                font_family_ranges: &[],
            });

            println!("test 2 — wrapped: w={:.1} h={:.1} lines={}", result.width, result.height, result.line_count);

            let mut border = RectNode::new(x, y, result.width, result.height);
            border.border_color([0.0, 0.8, 0.0, 1.0]);
            border.border_width(1.0);
            border.color([0.0; 4]);
            border.z(2);
            self.scene.add_rect(border);

            let mut t = TextNode::new(text, x, y, size);
            t.color([0.1, 0.1, 0.1, 1.0]);
            t.max_width(max_width);
            t.z(3);
            self.scene.add_text(t);
        }

        // --- test 3: bold range ---
        // border should account for bold being wider
        {
            let text = "Normal BOLD normal";
            let x = 40.0;
            let y = 300.0;
            let size = 20.0;
            let bold_ranges = [WeightRange { start: 7, end: 11, weight: 700 }];

            let result = measurer.measure(TextMeasureRequest {
                text,
                font_family: "",
                size,
                weight: 400,
                italic: false,
                letter_spacing: 0.0,
                line_height: None,
                max_width: None,
                weight_ranges: &bold_ranges,
                italic_ranges: &[],
                font_family_ranges: &[],
            });

            println!("test 3 — bold range: w={:.1} h={:.1} lines={}", result.width, result.height, result.line_count);

            let mut border = RectNode::new(x, y, result.width, result.height);
            border.border_color([0.0, 0.8, 0.0, 1.0]);
            border.border_width(1.0);
            border.color([0.0; 4]);
            border.z(2);
            self.scene.add_rect(border);

            let mut t = TextNode::new(text, x, y, size);
            t.color([0.1, 0.1, 0.1, 1.0]);
            t.add_weight(7, 11, 700);
            t.z(3);
            self.scene.add_text(t);
        }

        // --- test 4: large font ---
        {
            let text = "Big text";
            let x = 40.0;
            let y = 400.0;
            let size = 48.0;

            let result = measurer.measure(TextMeasureRequest {
                text,
                font_family: "",
                size,
                weight: 400,
                italic: false,
                letter_spacing: 0.0,
                line_height: None,
                max_width: None,
                weight_ranges: &[],
                italic_ranges: &[],
                font_family_ranges: &[],
            });

            println!("test 4 — large font: w={:.1} h={:.1} lines={}", result.width, result.height, result.line_count);

            let mut border = RectNode::new(x, y, result.width, result.height);
            border.border_color([0.0, 0.8, 0.0, 1.0]);
            border.border_width(1.0);
            border.color([0.0; 4]);
            border.z(2);
            self.scene.add_rect(border);

            let mut t = TextNode::new(text, x, y, size);
            t.color([0.1, 0.1, 0.1, 1.0]);
            t.z(3);
            self.scene.add_text(t);
        }
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
