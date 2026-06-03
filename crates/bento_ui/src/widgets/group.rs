use bento_wgpu::{DrawCommand, RectDraw};

use crate::{
    layout::{CrossAxis, Direction, MainAxis, Size},
    node::Node,
    tree,
    view::{View, ViewId},
};

pub struct Group {
    children: Vec<Box<dyn View>>,
    pub direction: Direction,
    pub gap: f32,
    pub padding: f32,
    pub main_axis: MainAxis,
    pub cross_axis: CrossAxis,
}

impl Group {
    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn direction(mut self, d: Direction) -> Self {
        self.direction = d;
        self
    }
    pub fn gap(mut self, g: f32) -> Self {
        self.gap = g;
        self
    }
    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }
    pub fn main_axis(mut self, m: MainAxis) -> Self {
        self.main_axis = m;
        self
    }
    pub fn cross_axis(mut self, c: CrossAxis) -> Self {
        self.cross_axis = c;
        self
    }
}

impl View for Group {
    fn name(&self) -> &'static str {
        "Group"
    }

    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        vec![DrawCommand::Rect(RectDraw {
            x,
            y,
            w,
            h,
            color: [0.8, 0.2, 0.2, 1.0],
            radii: [0.0; 4],
            border_color: [0.0, 0.0, 0.0, 0.8],
            border_widths: [3.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: None,
            z: 0,
        })]
    }

    fn build(self: Box<Self>) -> ViewId {
        let direction = self.direction;
        let gap = self.gap;
        let padding = self.padding;
        let main_axis = self.main_axis;
        let cross_axis = self.cross_axis;
        let child_ids: Vec<ViewId> = self.children.into_iter().map(|c| c.build()).collect();

        let id = tree::add_node(Node {
            view: Box::new(Group {
                children: Vec::new(),
                direction,
                gap,
                padding,
                main_axis,
                cross_axis,
            }),
            parent: None,
            children: Vec::new(),
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            handlers: Vec::new(),
            owner: None,
            paint_dirty: true,
            cache: Vec::new(),
            paint_subscriber: None,
            layout_dirty: true,
            width: Size::Fill,
            height: Size::Fill,
            direction,
            gap,
            padding,
            main_axis,
            cross_axis,
        });

        for child_id in child_ids {
            tree::append_child(id, child_id);
        }

        id
    }
}

pub fn group() -> Group {
    Group {
        children: Vec::new(),
        direction: Direction::Column,
        gap: 0.0,
        padding: 0.0,
        main_axis: MainAxis::Start,
        cross_axis: CrossAxis::Start,
    }
}
