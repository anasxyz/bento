use bento_wgpu::TextMeasurer;
use bento_wgpu::{DrawCommand, RectDraw};

use crate::layout::Position;
use crate::{
    layout::Size,
    node::Node,
    tree,
    view::{View, ViewId},
};

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
            view: self,
            parent: None,
            children: Vec::new(),
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            handlers: Vec::new(),
            owners: Vec::new(),
            paint_dirty: true,
            cache: Vec::new(),
            paint_subscriber: None,
            layout_dirty: true,
            width: Size::Auto,
            height: Size::Auto,
            position: Position::Relative,
            last_available_w: -1.0,
            last_available_h: -1.0,
        })
    }
}

pub fn rect(color: impl Fn() -> [f32; 4] + 'static) -> Rect {
    Rect {
        color: Box::new(color),
        radius: 0.0,
    }
}
