use std::any::Any;

use bento_wgpu::{DrawCommand, RectDraw};

use crate::layout::LayoutProps;
use crate::node::{self, Node};
use crate::reactive::value::Reactive;
use crate::tree;
use crate::views::{View, ViewConfig, ViewId};

pub struct Rect {
    pub color: Reactive<[f32; 4]>,
    pub radius: Reactive<f32>,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            color: [0.0, 0.0, 0.0, 0.0].into(),
            radius: 0.0_f32.into(),
        }
    }

    pub fn color(mut self, v: impl Into<Reactive<[f32; 4]>>) -> Self {
        self.color = v.into();
        self
    }
    pub fn radius(mut self, v: impl Into<Reactive<f32>>) -> Self {
        self.radius = v.into();
        self
    }
}

impl View for Rect {
    fn name(&self) -> &'static str {
        "Rect"
    }

    fn render(&mut self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        vec![DrawCommand::Rect(RectDraw {
            x,
            y,
            w,
            h,
            color: (self.color.get()),
            radii: [self.radius.get(); 4],
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
        let view = Box::new(Self::new());

        let node = Node::with_name("Rect (Primitive)");

        let id = tree::add_node(node, view);

        id
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ViewConfig<Rect> {
    pub fn color(mut self, v: impl Into<Reactive<[f32; 4]>>) -> Self {
        self.inner.color = v.into();
        self
    }
    pub fn radius(mut self, v: impl Into<Reactive<f32>>) -> Self {
        self.inner.radius = v.into();
        self
    }
}

pub fn rect() -> Rect {
    Rect {
        color: [0.0, 0.0, 0.0, 0.0].into(),
        radius: 0.0_f32.into(),
    }
}
