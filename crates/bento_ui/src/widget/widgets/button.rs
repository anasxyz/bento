use std::any::Any;

use crate::ui::layout::Size;
use crate::widget::WidgetHandle;
use crate::{Click, HoverEnter, HoverLeave, Ui};
use crate::{Widget, widget::Canvas};
use bento_wgpu::{RectDraw, TextDraw};
use bento_wgpu::{TextAlign, TextMeasureRequest, TextMeasurer};

pub struct Button {
    x: f32,
    y: f32,
    w: f32,
    h: f32,

    text_w: f32,
    text_h: f32,
    hovered: bool,

    pub width: Size,
    pub height: Size,

    pub text: String,
    pub text_color: [f32; 4],
    pub font_size: f32,
    pub font_family: String,

    pub bg_color: [f32; 4],
    pub bg_hover_color: [f32; 4],
    pub z: i32,
    pub padding: f32,
    pub border_width: [f32; 4],
    pub border_color: [f32; 4],
    pub border_radius: [f32; 4],
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,

            text_w: 0.0,
            text_h: 0.0,
            hovered: false,

            width: Size::Auto,
            height: Size::Auto,

            text: text.to_string(),
            text_color: [1.0, 1.0, 1.0, 1.0],
            font_size: 14.0,
            font_family: "".to_string(),

            bg_color: [0.2, 0.2, 0.2, 1.0],
            bg_hover_color: [0.3, 0.3, 0.3, 1.0],
            z: 0,
            padding: 16.0,
            border_width: [0.0; 4],
            border_color: [0.0; 4],
            border_radius: [0.0; 4],
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
    pub fn set_text_color(&mut self, color: [f32; 4]) {
        self.text_color = color;
    }
    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size;
    }
    pub fn set_font_family(&mut self, family: &str) {
        self.font_family = family.to_string();
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.bg_color = color;
    }
    pub fn set_hover_color(&mut self, color: [f32; 4]) {
        self.bg_hover_color = color;
    }
    pub fn set_padding(&mut self, padding: f32) {
        self.padding = padding;
    }
    pub fn set_border_width(&mut self, width: [f32; 4]) {
        self.border_width = width;
    }
    pub fn set_border_color(&mut self, color: [f32; 4]) {
        self.border_color = color;
    }
    pub fn set_border_radius(&mut self, radius: [f32; 4]) {
        self.border_radius = radius;
    }
}

impl Widget for Button {
    fn name(&self) -> &str {
        "Button"
    }

    fn build(ui: &mut Ui, handle: WidgetHandle<()>) {
        let handle = handle.typed::<Button>();
        ui.listen(handle, move |_: &HoverEnter, ui: &mut Ui| {
            if let Some(b) = ui.get_mut(handle) {
                b.hovered = true;
            }
            ui.request_redraw();
        });
        ui.listen(handle, move |_: &HoverLeave, ui: &mut Ui| {
            if let Some(b) = ui.get_mut(handle) {
                b.hovered = false;
            }
            ui.request_redraw();
        });
    }

    fn update(ui: &mut Ui, handle: WidgetHandle<()>) {
        let handle = handle.typed::<Button>();
        let mut measurer = std::mem::take(&mut ui.measurer);
        if let Some(b) = ui.get_mut(handle) {
            let result = measurer.measure(TextMeasureRequest {
                text: &b.text,
                font_family: "",
                size: b.font_size,
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
            b.text_w = result.width;
            b.text_h = result.height;
            if matches!(b.width, Size::Auto) {
                b.w = result.width + b.padding * 2.0;
            }
            if matches!(b.height, Size::Auto) {
                b.h = result.height + b.padding * 2.0;
            }
        }
        ui.measurer = measurer;
    }

    fn set_size(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
    }

    fn width_sizing(&self) -> &Size {
        &self.width
    }
    fn height_sizing(&self) -> &Size {
        &self.height
    }

    fn size(&self) -> (f32, f32) {
        (self.w, self.h)
    }
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    fn z(&self) -> i32 {
        self.z
    }

    fn render(&mut self, canvas: &mut Canvas) {
        let lw = self.text_w;
        let lh = self.text_h;
        let color = if self.hovered {
            self.bg_hover_color
        } else {
            self.bg_color
        };
        canvas.draw_list.push_rect(RectDraw {
            x: canvas.x,
            y: canvas.y,
            w: self.w,
            h: self.h,
            color: color,
            radii: self.border_radius,
            border_color: self.border_color,
            border_widths: self.border_width,
            rotate: canvas.rotate,
            scale_x: canvas.scale_x,
            scale_y: canvas.scale_y,
            opacity: canvas.opacity,
            clip: canvas.clip,
            z: canvas.z,
        });
        canvas.draw_list.push_text(TextDraw {
            x: canvas.x + (self.w - lw) / 2.0,
            y: canvas.y + (self.h - lh) / 2.0,
            w: lw,
            h: lh,
            text: self.text.clone(),
            size: self.font_size,
            color: self.text_color,
            weight: 400,
            italic: false,
            font_family: self.font_family.clone(),
            max_width: None,
            line_height: None,
            tab_width: 4,
            letter_spacing: 0.0,
            align: TextAlign::Left,
            opacity: canvas.opacity,
            clip: canvas.clip,
            rotate: canvas.rotate,
            scale_x: canvas.scale_x,
            scale_y: canvas.scale_y,
            z: canvas.z + 1,
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
