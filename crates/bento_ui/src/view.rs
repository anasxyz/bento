use crate::reactive::owner::Owner;
use bento_wgpu::{DrawList, RectDraw, TextAlign, TextDraw};

pub trait View {
    fn render(&self, x: f32, y: f32, draw_list: &mut DrawList);
}

pub struct OwnedView {
    _owner: Owner,
    inner: Box<dyn View>,
}

impl OwnedView {
    pub fn new(owner: Owner, inner: impl View + 'static) -> Self {
        Self {
            _owner: owner,
            inner: Box::new(inner),
        }
    }
}

impl View for OwnedView {
    fn render(&self, x: f32, y: f32, draw_list: &mut DrawList) {
        self.inner.render(x, y, draw_list);
    }
}

pub struct Rect {
    color: Box<dyn Fn() -> [f32; 4]>,
    children: Vec<Box<dyn View>>,
}

impl Rect {
    pub fn color(mut self, f: impl Fn() -> [f32; 4] + 'static) -> Self {
        self.color = Box::new(f);
        self
    }

    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl View for Rect {
    fn render(&self, x: f32, y: f32, draw_list: &mut DrawList) {
        draw_list.push_rect(RectDraw {
            x,
            y,
            w: 100.0,
            h: 100.0,
            color: (self.color)(),
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
            clip: None,
            z: 0,
        });
        for child in &self.children {
            child.render(x, y, draw_list);
        }
    }
}

pub fn rect() -> Rect {
    Rect {
        color: Box::new(|| [0.0; 4]),
        children: Vec::new(),
    }
}

pub struct Text {
    content: Box<dyn Fn() -> String>,
}

impl View for Text {
    fn render(&self, x: f32, y: f32, draw_list: &mut DrawList) {
        draw_list.push_text(TextDraw {
            x,
            y,
            w: 100.0,
            h: 20.0,
            text: (self.content)(),
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
        });
    }
}

pub fn text(f: impl Fn() -> String + 'static) -> Text {
    Text {
        content: Box::new(f),
    }
}
