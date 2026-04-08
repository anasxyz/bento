use bento_wgpu::{ImageKey, RenderContext, Renderer, SceneGraph, Surface};
use cosmic_text::FontSystem;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

fn load_svg(path: &str, width: u32, height: u32) -> (Vec<u8>, u32, u32) {
    let tree = resvg::usvg::Tree::from_data(
        &std::fs::read(path).unwrap(),
        &resvg::usvg::Options::default(),
    )
    .unwrap();

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height).unwrap();
    let scale_x = width as f32 / tree.size().width();
    let scale_y = height as f32 / tree.size().height();
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(scale_x, scale_y),
        &mut pixmap.as_mut(),
    );

    // resvg outputs premultiplied RGBA — un-premultiply for straight alpha upload
    let mut rgba = pixmap.take();
    for pixel in rgba.chunks_exact_mut(4) {
        let a = pixel[3];
        if a > 0 {
            pixel[0] = ((pixel[0] as u16 * 255) / a as u16).min(255) as u8;
            pixel[1] = ((pixel[1] as u16 * 255) / a as u16).min(255) as u8;
            pixel[2] = ((pixel[2] as u16 * 255) / a as u16).min(255) as u8;
        }
    }

    (rgba, width, height)
}

struct App {
    ctx: RenderContext,
    font_system: FontSystem,
    scene: SceneGraph,
    window: Option<Arc<Window>>,
    surface: Option<Surface<'static>>,
    renderer: Option<Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("bento_wgpu test")
                        .with_inner_size(winit::dpi::LogicalSize::new(800u32, 600u32)),
                )
                .unwrap(),
        );

        let size = window.inner_size();
        let scale = window.scale_factor() as f32;
        let w = size.width as f32 / scale;
        let h = size.height as f32 / scale;

        let surface = Surface::new(&self.ctx, Arc::clone(&window), w, h, scale);
        let mut renderer = Renderer::new(&self.ctx, &surface);

        let shadow = self.scene.add_shadow();
        self.scene
            .shadow_mut(shadow)
            .set_rect(50.0, 50.0, 300.0, 200.0);
        self.scene.shadow_mut(shadow).set_blur(16.0);
        self.scene.shadow_mut(shadow).set_offset(0.0, 4.0);
        self.scene.shadow_mut(shadow).set_visible(true);
        self.scene.add_child(self.scene.root, shadow.to_scene());

        let bg = self.scene.add_rect();
        self.scene.rect_mut(bg).set_rect(50.0, 50.0, 300.0, 200.0);
        self.scene.rect_mut(bg).set_color([0.2, 0.3, 0.8, 1.0]);
        self.scene.rect_mut(bg).set_radius(8.0);
        self.scene.rect_mut(bg).set_visible(true);
        self.scene.add_child(self.scene.root, bg.to_scene());

        let label = self.scene.add_text();
        self.scene.text_mut(label).set_pos(70.0, 70.0);
        self.scene.text_mut(label).set_content("hello bento_wgpu");
        self.scene.text_mut(label).set_family("sans-serif");
        self.scene.text_mut(label).set_size(20.0);
        self.scene.text_mut(label).set_color([1.0, 1.0, 1.0, 1.0]);
        self.scene.text_mut(label).set_visible(true);
        self.scene.add_child(self.scene.root, label.to_scene());

        let img = image::open("/home/anas/Claude-logo.jpeg")
            .expect("could not open assets/photo.png")
            .into_rgba8();
        let (img_w, img_h) = img.dimensions();
        let photo_key = ImageKey(1);
        renderer.upload_image(&self.ctx, photo_key, img.as_raw(), img_w, img_h);

        let img_node = self.scene.add_image();
        self.scene
            .image_mut(img_node)
            .set_rect(380.0, 50.0, 150.0, 150.0);
        self.scene.image_mut(img_node).set_image_key(photo_key);
        self.scene.image_mut(img_node).set_radius(12.0);
        self.scene.image_mut(img_node).set_visible(true);
        self.scene.add_child(self.scene.root, img_node.to_scene());

        let (svg_rgba, svg_w, svg_h) = load_svg("/home/anas/rust-icon.svg", 24, 24);
        let svg_key = ImageKey(2);
        renderer.upload_image(&self.ctx, svg_key, &svg_rgba, svg_w, svg_h);

        let svg_node = self.scene.add_image();
        self.scene
            .image_mut(svg_node)
            .set_rect(550.0, 50.0, 24.0, 24.0);
        self.scene.image_mut(svg_node).set_image_key(svg_key);
        self.scene.image_mut(svg_node).set_visible(true);
        self.scene.add_child(self.scene.root, svg_node.to_scene());

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
                    [0.1, 0.1, 0.1, 1.0],
                );
                window.request_redraw();
            }

            WindowEvent::Resized(_) => {
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
        })
        .unwrap();
}
