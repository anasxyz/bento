// src/app.rs

use std::sync::Arc;
use wgpu;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{ShapeRenderer, TextRenderer, Scene, DrawCommand};

// ============================================================================
// App - The main application window and GPU context
// ============================================================================

/// The main application that manages the window, GPU resources, and event loop.
/// 
/// ## Architecture
/// 
/// The App owns:
/// - Window and event loop (via winit)
/// - GPU device, queue, and surface (via wgpu)
/// - MSAA (anti-aliasing) texture for smooth rendering
/// 
/// It does NOT own the renderers or scene - those are created when `run()` is called.
pub struct App {
    /// The window event loop (taken when run() is called)
    event_loop: Option<EventLoop<()>>,
    
    /// The application window
    window: Arc<Window>,
    
    /// GPU device for creating resources
    device: wgpu::Device,
    
    /// GPU command queue for submitting work
    queue: wgpu::Queue,
    
    /// Surface to render to (the window's drawable area)
    surface: wgpu::Surface<'static>,
    
    /// Surface configuration (size, format, etc.)
    config: wgpu::SurfaceConfiguration,
    
    /// The pixel format of the surface
    surface_format: wgpu::TextureFormat,
    
    /// Multi-sample anti-aliasing texture (4x MSAA for smooth edges)
    msaa_texture: wgpu::Texture,
    
    /// View into the MSAA texture
    msaa_view: wgpu::TextureView,
    
    /// DPI scale factor (e.g., 2.0 for Retina displays)
    scale_factor: f64,
}

// ============================================================================
// Canvas - The drawing context passed to user code
// ============================================================================

/// The drawing context provided to the user's update function.
/// 
/// This is a lightweight struct that provides access to:
/// - The scene graph for adding drawing commands
/// - Window dimensions in logical coordinates
/// - The current DPI scale factor
pub struct Canvas<'a> {
    /// The scene graph to draw into
    pub scene: &'a mut Scene,
    
    /// Window width in logical coordinates (DPI-independent)
    pub width: f32,
    
    /// Window height in logical coordinates (DPI-independent)
    pub height: f32,
    
    /// DPI scale factor (e.g., 2.0 for Retina displays)
    pub scale_factor: f64,
}

// ============================================================================
// App Implementation
// ============================================================================

impl App {
    /// Create a new application window.
    /// 
    /// # Arguments
    /// * `title` - Window title
    /// * `width`, `height` - Initial window size in logical pixels
    /// 
    /// # Example
    /// ```
    /// let app = App::new("My App", 800, 600);
    /// ```
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        pollster::block_on(Self::new_async(title, width, height))
    }

    /// Internal async initialization (wgpu setup is async).
    async fn new_async(title: &str, width: u32, height: u32) -> Self {
        // Create window and event loop
        let event_loop = EventLoop::new().unwrap();
        let window = Arc::new(
            WindowBuilder::new()
                .with_title(title)
                .with_inner_size(winit::dpi::LogicalSize::new(width, height))
                .build(&event_loop)
                .unwrap(),
        );

        // Initialize GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        // Find a GPU adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Create device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        // Configure the surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];
        let physical_size = window.inner_size();
        let scale_factor = window.scale_factor();
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Create MSAA resources for anti-aliasing
        let msaa_texture = Self::create_msaa_texture(&device, &config, surface_format);
        let msaa_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            event_loop: Some(event_loop),
            window,
            device,
            queue,
            surface,
            config,
            surface_format,
            msaa_texture,
            msaa_view,
            scale_factor,
        }
    }

    /// Create a multi-sample anti-aliasing texture for smooth rendering.
    fn create_msaa_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        format: wgpu::TextureFormat,
    ) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("MSAA Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4, // 4x MSAA
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
    }

    /// Start the application event loop with a user-provided update function.
    /// 
    /// # The Update Function
    /// 
    /// Your update function receives a `Canvas` and is called when the scene is dirty.
    /// Use `canvas.scene` to add drawing commands:
    /// 
    /// ```
    /// app.run(|canvas| {
    ///     canvas.scene.rect(10.0, 10.0, 100.0, 50.0, [1.0, 0.0, 0.0, 1.0]);
    ///     canvas.scene.text("Hello!", 10.0, 70.0);
    /// });
    /// ```
    /// 
    /// # Retained Mode
    /// 
    /// The update function is only called when needed (initially and when dirty).
    /// This is more efficient than immediate mode where you redraw every frame.
    pub fn run<F>(mut self, mut update_fn: F)
    where
        F: FnMut(&mut Canvas) + 'static,
    {
        // Create renderers
        let logical_size = self.get_logical_size();
        let mut shape_renderer = ShapeRenderer::new(
            &self.device,
            self.surface_format,
            logical_size.0,
            logical_size.1,
        );

        let mut text_renderer = TextRenderer::new(&self.device, &self.queue, self.surface_format);
        text_renderer.resize(logical_size.0, logical_size.1, self.scale_factor);
        
        let mut scene = Scene::new();

        // Take ownership of event loop
        let event_loop = self.event_loop.take().unwrap();

        // Run the event loop
        let _ = event_loop.run(move |event, target| {
            target.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    self.handle_window_event(
                        event,
                        target,
                        &mut shape_renderer,
                        &mut text_renderer,
                        &mut scene,
                        &mut update_fn,
                    );
                }
                _ => {}
            }
        });
    }

    /// Handle window events (resize, redraw, close, etc.).
    fn handle_window_event<F>(
        &mut self,
        event: WindowEvent,
        target: &winit::event_loop::EventLoopWindowTarget<()>,
        shape_renderer: &mut ShapeRenderer,
        text_renderer: &mut TextRenderer,
        scene: &mut Scene,
        update_fn: &mut F,
    ) where
        F: FnMut(&mut Canvas),
    {
        match event {
            // DPI scale changed (e.g., moved to different monitor)
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.handle_scale_change(scale_factor, shape_renderer, text_renderer, scene);
            }

            // Window was resized
            WindowEvent::Resized(new_size) => {
                self.handle_resize(new_size, shape_renderer, text_renderer, scene);
            }

            // Time to redraw
            WindowEvent::RedrawRequested => {
                self.handle_redraw(shape_renderer, text_renderer, scene, update_fn);
            }

            // User closed the window
            WindowEvent::CloseRequested => {
                target.exit();
            }

            _ => {}
        }
    }

    /// Handle DPI scale factor changes.
    fn handle_scale_change(
        &mut self,
        scale_factor: f64,
        shape_renderer: &mut ShapeRenderer,
        text_renderer: &mut TextRenderer,
        scene: &mut Scene,
    ) {
        self.scale_factor = scale_factor;
        
        // Update surface configuration
        let physical_size = self.window.inner_size();
        self.config.width = physical_size.width;
        self.config.height = physical_size.height;
        self.surface.configure(&self.device, &self.config);

        // Recreate MSAA texture
        self.recreate_msaa_texture();

        // Update renderers
        let logical_size = self.get_logical_size();
        shape_renderer.resize(logical_size.0, logical_size.1);
        text_renderer.resize(logical_size.0, logical_size.1, self.scale_factor);
        
        // Mark scene dirty to trigger rebuild
        scene.mark_dirty();
        self.window.request_redraw();
    }

    /// Handle window resize.
    fn handle_resize(
        &mut self,
        new_size: winit::dpi::PhysicalSize<u32>,
        shape_renderer: &mut ShapeRenderer,
        text_renderer: &mut TextRenderer,
        scene: &mut Scene,
    ) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);

        // Recreate MSAA texture
        self.recreate_msaa_texture();

        // Update renderers
        let logical_size = self.get_logical_size();
        shape_renderer.resize(logical_size.0, logical_size.1);
        text_renderer.resize(logical_size.0, logical_size.1, self.scale_factor);
        
        // Mark scene dirty to trigger rebuild
        scene.mark_dirty();
        self.window.request_redraw();
    }

    /// Handle redraw request - this is where the magic happens!
    fn handle_redraw<F>(
        &mut self,
        shape_renderer: &mut ShapeRenderer,
        text_renderer: &mut TextRenderer,
        scene: &mut Scene,
        update_fn: &mut F,
    ) where
        F: FnMut(&mut Canvas),
    {
        // Step 1: Update scene if dirty (call user's update function)
        if scene.is_dirty() {
            scene.clear(); // Start with a clean slate
            
            let logical_size = self.get_logical_size();
            let mut canvas = Canvas {
                scene,
                width: logical_size.0,
                height: logical_size.1,
                scale_factor: self.scale_factor,
            };
            
            update_fn(&mut canvas); // User builds the scene
        }

        // Step 2: Render the scene
        self.render_scene(shape_renderer, text_renderer, scene);
        
        // Step 3: Mark scene as clean (won't rebuild until marked dirty again)
        scene.mark_clean();
    }

    /// Render the current scene to the screen.
    fn render_scene(
        &mut self,
        shape_renderer: &mut ShapeRenderer,
        text_renderer: &mut TextRenderer,
        scene: &Scene,
    ) {
        // Get the next frame to draw to
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            // Begin render pass
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.msaa_view,        // Render to MSAA texture
                    resolve_target: Some(&view),   // Resolve to screen
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Convert scene commands to renderer calls
            self.process_scene_commands(shape_renderer, text_renderer, scene);

            // Render shapes and text
            shape_renderer.render(&self.device, &self.queue, &mut pass);
            
            let logical_size = self.get_logical_size();
            text_renderer.render(
                logical_size.0,
                logical_size.1,
                self.scale_factor,
                &self.device,
                &self.queue,
                &mut pass,
            );
        }

        // Submit work to GPU and present frame
        self.queue.submit([encoder.finish()]);
        frame.present();
    }

    /// Convert scene drawing commands into renderer calls.
    fn process_scene_commands(
        &self,
        shape_renderer: &mut ShapeRenderer,
        text_renderer: &mut TextRenderer,
        scene: &Scene,
    ) {
        // Clear previous frame's data
        shape_renderer.clear();
        text_renderer.clear();
        
        let logical_size = self.get_logical_size();

        // Process each command in the scene
        for cmd in scene.commands() {
            match cmd {
                DrawCommand::Rect { x, y, w, h, color } => {
                    shape_renderer.rect(*x, *y, *w, *h, *color);
                }
                DrawCommand::Circle { cx, cy, radius, color } => {
                    shape_renderer.circle(*cx, *cy, *radius, *color);
                }
                DrawCommand::RoundedRect { x, y, w, h, radius, color } => {
                    shape_renderer.rounded_rect(*x, *y, *w, *h, *radius, *color);
                }
                DrawCommand::Text { text, x, y } => {
                    text_renderer.queue_text(
                        text,
                        *x,
                        *y,
                        logical_size.0,
                        logical_size.1,
                        self.scale_factor,
                    );
                }
            }
        }
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    /// Get window size in logical coordinates (DPI-independent).
    fn get_logical_size(&self) -> (f32, f32) {
        (
            (self.config.width as f64 / self.scale_factor) as f32,
            (self.config.height as f64 / self.scale_factor) as f32,
        )
    }

    /// Recreate the MSAA texture after a resize or DPI change.
    fn recreate_msaa_texture(&mut self) {
        self.msaa_texture = Self::create_msaa_texture(
            &self.device,
            &self.config,
            self.surface_format,
        );
        self.msaa_view = self.msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());
    }
}
