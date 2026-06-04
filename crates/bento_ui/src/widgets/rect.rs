use crate::layout::LayoutProps;
use crate::node::{self, Node};
use crate::tree;
use crate::view::{View, ViewId};
use bento_wgpu::{DrawCommand, RectDraw};

pub struct Rect {
    pub color: Box<dyn Fn() -> [f32; 4]>,
    pub radius: f32,
}

impl View for Rect {
    fn name(&self) -> &'static str {
        "Rect"
    }

    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        vec![DrawCommand::Rect(RectDraw {
            x,
            y,
            w,
            h,
            color: (self.color)(),
            radii: [self.radius; 4],
            border_color: [0.0, 0.0, 0.0, 1.0],
            border_widths: [1.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: None,
            z: 0,
        })]
    }

    fn build(self: Box<Self>) -> ViewId {
        tree::add_node(Node {
            name: Some("Rect (Primitive)"),
            view: self,
            taffy_id: node::placeholder_taffy_id(),
            parent: None,
            children: Vec::new(),
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            layout: LayoutProps::default(),
            handlers: Vec::new(),
            owners: Vec::new(),
            paint_dirty: true,
            cache: Vec::new(),
            paint_subscriber: None,
        })
    }
}

pub fn rect(color: impl Fn() -> [f32; 4] + 'static) -> Rect {
    Rect {
        color: Box::new(color),
        radius: 0.0,
    }
}
