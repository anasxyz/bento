use crate::layout::LayoutProps;
use crate::node::{self, Node};
use crate::tree;
use crate::views::{View, ViewId};
use bento_wgpu::{DrawCommand, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};

pub struct Text {
    content: Box<dyn Fn() -> String>,
}

impl View for Text {
    fn name(&self) -> &'static str {
        "Text"
    }

    fn build(self: Box<Self>) -> ViewId {
        tree::add_node(Node {
            name: Some("Text (Primitive)"),
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
            scroll_x: 0.0,
            scroll_y: 0.0,
            scrollable: false,
            clip: false,
        })
    }

    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
        // println!("[measure] text");
        let text = (self.content)();
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

    fn render(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
        let text = (self.content)();
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

pub fn text(f: impl Fn() -> String + 'static) -> Text {
    Text {
        content: Box::new(f),
    }
}
