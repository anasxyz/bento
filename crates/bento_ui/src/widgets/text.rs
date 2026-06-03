use bento_wgpu::{DrawCommand, DrawList, TextAlign, TextDraw, TextMeasureRequest, TextMeasurer};

use crate::{
    layout::{CrossAxis, Direction, MainAxis, Size},
    node::{Node, NodeType, TextNode},
    tree,
    ui::Ui,
    view::{View, ViewId},
};

pub struct Text {
    content: Box<dyn Fn() -> String>,
}

impl View for Text {
    fn name(&self) -> &'static str {
        "Text"
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
            owner: None,
            paint_dirty: true,
            cache: Vec::new(),
            paint_subscriber: None,
            layout_dirty: true,
            width: Size::Auto,
            height: Size::Auto,
            last_available_w: -1.0,
            last_available_h: -1.0,
        })
    }

    fn measure(&self, measurer: &mut TextMeasurer) -> (f32, f32) {
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
            text: text,
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
