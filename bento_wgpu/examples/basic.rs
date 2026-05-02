use bento_wgpu::{RenderContext, Renderer, SceneGraph, Surface, TextId};
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
    font_system: FontSystem,
    scene: SceneGraph,
    window: Option<Arc<Window>>,
    surface: Option<Surface<'static>>,
    renderer: Option<Renderer>,

    rotating_label: Option<TextId>,
    angle_label: Option<TextId>,

    start: Instant,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("rotation quality test")
                        .with_inner_size(winit::dpi::LogicalSize::new(800u32, 600u32)),
                )
                .unwrap(),
        );

        let size = window.inner_size();
        let scale = window.scale_factor() as f32;
        let w = size.width as f32 / scale;
        let h = size.height as f32 / scale;

        let surface = Surface::new(&self.ctx, Arc::clone(&window), w, h, scale);
        let renderer = Renderer::new(&self.ctx, &surface);

        // Static reference — always 0 degrees so we can compare quality
        let ref_label = self.scene.add_text();
        self.scene.text_mut(ref_label).set_pos(40.0, 40.0);
        self.scene
            .text_mut(ref_label)
            .set_content("reference (0 deg) — The quick brown fox");
        self.scene.text_mut(ref_label).set_family("sans-serif");
        self.scene.text_mut(ref_label).set_size(20.0);
        self.scene
            .text_mut(ref_label)
            .set_color([1.0, 1.0, 1.0, 1.0]);
        self.scene.text_mut(ref_label).set_visible(true);
        self.scene.add_child(self.scene.root, ref_label.to_scene());

        // Rotating label — same text, same size, spins continuously
        let rotating = self.scene.add_text();
        self.scene.text_mut(rotating).set_pos(400.0, 300.0);
        self.scene
            .text_mut(rotating)
            .set_content("The quick brown fox");
        self.scene.text_mut(rotating).set_family("sans-serif");
        self.scene.text_mut(rotating).set_size(20.0);
        self.scene
            .text_mut(rotating)
            .set_color([1.0, 0.85, 0.3, 1.0]);
        self.scene.text_mut(rotating).set_visible(true);
        self.scene.text_mut(rotating).set_rotate(45.0_f32.to_radians());
        self.scene.add_child(self.scene.root, rotating.to_scene());

        // Live angle readout at bottom
        let angle_label = self.scene.add_text();
        self.scene.text_mut(angle_label).set_pos(40.0, 555.0);
        self.scene
            .text_mut(angle_label)
            .set_content("angle: 0.0 deg");
        self.scene.text_mut(angle_label).set_family("sans-serif");
        self.scene.text_mut(angle_label).set_size(15.0);
        self.scene
            .text_mut(angle_label)
            .set_color([0.5, 0.5, 0.5, 1.0]);
        self.scene.text_mut(angle_label).set_visible(true);
        self.scene
            .add_child(self.scene.root, angle_label.to_scene());

        self.rotating_label = Some(rotating);
        self.angle_label = Some(angle_label);
        self.window = Some(window);
        self.surface = Some(surface);
        self.renderer = Some(renderer);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                let (Some(renderer), Some(surface), Some(window)) = (
                    self.renderer.as_mut(),
                    self.surface.as_mut(),
                    self.window.as_ref(),
                ) else {
                    return;
                };

                renderer.render(
                    &mut self.ctx,
                    &mut self.font_system,
                    surface,
                    &mut self.scene,
                    [0.08, 0.08, 0.08, 1.0],
                );
            }

            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                let (Some(renderer), Some(surface)) =
                    (self.renderer.as_mut(), self.surface.as_mut())
                else {
                    return;
                };
                let window = self.window.as_ref().unwrap();
                let size = window.inner_size();
                let scale = window.scale_factor() as f32;
                let w = size.width as f32 / scale;
                let h = size.height as f32 / scale;
                surface.resize(&self.ctx, w, h, scale);
                renderer.resize(&self.ctx, surface, &mut self.scene);
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
            font_system,
            scene: SceneGraph::new(),
            window: None,
            surface: None,
            renderer: None,
            rotating_label: None,
            angle_label: None,
            start: Instant::now(),
        })
        .unwrap();
}
