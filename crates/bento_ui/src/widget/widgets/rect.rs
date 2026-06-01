use crate::ui::layout::Size;
use crate::widget::{Canvas, Widget};
use bento_wgpu::RectDraw;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub width: Size,
    pub height: Size,
    pub color: [f32; 4],
    pub z: i32,
}

impl Rect {
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w,
            h,
            width: Size::Fixed(w),
            height: Size::Fixed(h),
            color: [0.0, 0.0, 0.0, 1.0],
            z: 0,
        }
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn set_w(&mut self, w: f32) {
        self.w = w;
        self.width = Size::Fixed(w);
    }
    pub fn set_h(&mut self, h: f32) {
        self.h = h;
        self.height = Size::Fixed(h);
    }
    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
}

impl Widget for Rect {
    fn name(&self) -> &str {
        "Rect"
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
        canvas.draw_list.push_rect(RectDraw {
            x: canvas.x,
            y: canvas.y,
            w: self.w,
            h: self.h,
            color: self.color,
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            rotate: canvas.rotate,
            scale_x: canvas.scale_x,
            scale_y: canvas.scale_y,
            opacity: canvas.opacity,
            clip: canvas.clip,
            z: canvas.z,
        });
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
}
