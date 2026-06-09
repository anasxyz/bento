use std::any::Any;

use crate::layout::LayoutProps;
use crate::node::{self, Node};
use crate::reactive::value::Reactive;
use crate::tree;
use crate::views::{View, ViewId};
use bento_wgpu::{DrawCommand, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};

pub struct Text {
    content: Reactive<String>,
}

impl Text {
    pub fn new() -> Self {
        Self {
            content: "".into(),
        }
    }
}

impl View for Text {
    fn name(&self) -> &'static str {
        "Text"
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn build(self: Box<Self>) -> ViewId {
        let view = self;

        let node = Node::with_name("Text (Primitive)");

        let id = tree::add_node(node, view);

        id
    }

    fn measure(&mut self, measurer: &mut TextMeasurer) -> (f32, f32) {
        // println!("[measure] text");
        let text = (self.content.get_clone());
        let r = measurer.measure(TextMeasureRequest {
            text: &text,
            font_family: "",
            size: 14.0,
            weight: 400,
            italic: false,
            letter_spacing: 0.0,
            line_height: None,
            tab_width: 4,
            max_width: None,
            weight_ranges: &[],
            italic_ranges: &[],
            font_family_ranges: &[],
        });
        (r.width, r.height)
    }

    fn render(&mut self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        let text = (self.content.get_clone());
        vec![DrawCommand::Text(TextDraw {
            x,
            y,
            w,
            h,
            text,
            size: 14.0,
            color: [1.0, 1.0, 1.0, 1.0],
            weight: 400,
            italic: false,
            font_family: String::new(),
            max_width: None,
            line_height: None,
            tab_width: 4,
            letter_spacing: 0.0,
            align: TextAlign::Left,
            opacity: 1.0,
            clip: None,
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            z: 0,
            color_ranges: vec![],
            background_ranges: vec![],
            underline_ranges: vec![],
            strikethrough_ranges: vec![],
            weight_ranges: vec![],
            italic_ranges: vec![],
            font_family_ranges: vec![],
        })]
    }
}

pub fn text(f: impl Into<Reactive<String>>) -> Text {
    Text { content: f.into() }
}
