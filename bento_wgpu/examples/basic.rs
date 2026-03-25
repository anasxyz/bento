use bento_wgpu::{RenderContext, Surface, SceneGraph, Renderer};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
use std::sync::Arc;

struct App {
    window:   Option<Arc<Window>>,
    surface:  Option<Surface<'static>>,
    renderer: Option<Renderer>,
    scene:    SceneGraph,
    ctx:      RenderContext,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop.create_window(Window::default_attributes()
                .with_title("bento_wgpu test")
                .with_inner_size(winit::dpi::LogicalSize::new(800u32, 600u32)))
                .unwrap()
        );

        let size  = window.inner_size();
        let scale = window.scale_factor() as f32;
        let w = size.width  as f32 / scale;
        let h = size.height as f32 / scale;

        // window arc kept alive for the duration of surface
        let surface = Surface::new(
            &self.ctx,
            Arc::clone(&window),
            w, h, scale,
        );
        let renderer = Renderer::new(&self.ctx, &surface);

        // build scene 

        // background panel
        let bg = self.scene.add_rect();
        self.scene.rect_mut(bg).set_rect(50.0, 50.0, 300.0, 200.0);
        self.scene.rect_mut(bg).set_color([0.2, 0.3, 0.8, 1.0]);
        self.scene.rect_mut(bg).set_radius(8.0);
        self.scene.rect_mut(bg).set_visible(true);

        // shadow behind it
        let shadow = self.scene.add_shadow();
        self.scene.shadow_mut(shadow).set_rect(50.0, 50.0, 300.0, 200.0);
        self.scene.shadow_mut(shadow).set_blur(16.0);
        self.scene.shadow_mut(shadow).set_offset(0.0, 4.0);
        self.scene.shadow_mut(shadow).set_visible(true);

        // label
        let label = self.scene.add_text();
        self.scene.text_mut(label).set_pos(70.0, 70.0);
        self.scene.text_mut(label).set_content("hello bento_wgpu");
        self.scene.text_mut(label).set_family("sans-serif");
        self.scene.text_mut(label).set_size(20.0);
        self.scene.text_mut(label).set_color([1.0, 1.0, 1.0, 1.0]);
        self.scene.text_mut(label).set_visible(true);

        self.window   = Some(window);
        self.surface  = Some(surface);
        self.renderer = Some(renderer);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                let (Some(renderer), Some(surface), Some(window)) =
                    (self.renderer.as_mut(), self.surface.as_mut(), self.window.as_ref())
                else { return };

                renderer.render(&self.ctx, surface, &mut self.scene, [0.1, 0.1, 0.1, 1.0]);
                window.request_redraw();
            }
            WindowEvent::Resized(_) => {
                let (Some(renderer), Some(surface), Some(window)) =
                    (self.renderer.as_mut(), self.surface.as_mut(), self.window.as_ref())
                else { return };

                let size  = window.inner_size();
                let scale = window.scale_factor() as f32;
                surface.resize(&self.ctx, size.width as f32 / scale, size.height as f32 / scale, scale);
                renderer.resize(&self.ctx, surface);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}

fn main() {
    let ctx = pollster::block_on(RenderContext::new());
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut App {
        ctx,
        scene:    SceneGraph::new(),
        window:   None,
        surface:  None,
        renderer: None,
    }).unwrap();
}
