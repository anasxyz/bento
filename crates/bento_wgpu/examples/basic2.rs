#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use bento_wgpu::{GroupNode, RectNode, RenderContext, Scene, TextNode};
use cosmic_text::FontSystem;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

struct App {
    ctx: RenderContext,
    window: Option<Arc<Window>>,
    surface: Option<bento_wgpu::Surface<'static>>,
    renderer: Option<bento_wgpu::Renderer>,
    scene: Scene,
    font_system: FontSystem,
    bold: bool,
    highlight: bool,
    moved: bool,
    scroll_y: f32,
    group_id: Option<bento_shared::scene::SceneNodeId>,
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

        // create a group with offset
        let mut group = GroupNode::new();
        group.offset_x = 200.0;
        group.offset_y = 200.0;
        let group_id = self.scene.add_group(group);

        // create a rect at 0,0 — should appear at 200,200 due to group offset
        let mut rect2 = RectNode::new(0.0, 0.0, 100.0, 50.0);
        rect2.color = [1.0, 0.3, 0.3, 1.0];
        let rect2_id = self.scene.add_rect(rect2);

        let mut rect3 = RectNode::new(100.0, 50.0, 100.0, 50.0);
        rect3.color = [0.3, 0.3, 1.0, 1.0];
        let rect3_id = self.scene.add_rect(rect3);

        // add rect as child of group
        self.scene.add_to_group(group_id, rect2_id);
        self.scene.add_to_group(group_id, rect3_id);

        self.group_id = Some(group_id);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let (Some(renderer), Some(surface)) = (self.renderer.as_mut(), self.surface.as_mut())
        else {
            return;
        };

        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Space),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.bold = !self.bold;
                self.build_scene();
                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyH),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.highlight = !self.highlight;
                self.build_scene();
                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyM),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.moved = !self.moved;
                self.build_scene();
                self.window.as_ref().unwrap().request_redraw();
            }

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

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::ArrowUp),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.scroll_y -= 20.0;
                if let Some(id) = self.group_id {
                    if let Some(bento_shared::scene::Node::Group(g)) =
                        self.scene.nodes.get_mut(id.0)
                    {
                        g.offset_y = self.scroll_y;
                    }
                }
                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::ArrowDown),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.scroll_y += 20.0;
                if let Some(id) = self.group_id {
                    if let Some(bento_shared::scene::Node::Group(g)) =
                        self.scene.nodes.get_mut(id.0)
                    {
                        g.offset_y = self.scroll_y;
                    }
                }
                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}

impl App {
    fn build_scene(&mut self) {
        self.scene = Scene::new();

        let x = if self.moved { 200.0 } else { 60.0 };

        let mut text = TextNode::new("Press space to toggle bold on this word", x, 60.0, 18.0);
        text.color([0.0, 0.0, 0.0, 1.0]).max_width(400.0).z(1);

        if self.bold {
            text.add_weight(22, 32, 700);
        }

        if self.highlight {
            text.add_background(6, 11, [1.0, 0.8, 0.0, 0.4]);
        }

        self.scene.add_text(text);
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
            bold: false,
            highlight: false,
            moved: false,
            scroll_y: 200.0,
            group_id: None,
        })
        .unwrap();
}
