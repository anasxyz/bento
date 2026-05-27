use bento_wgpu::RectDraw;

use crate::Ui;
use crate::events::types::{MouseDown, MouseMove, MouseScroll, MouseUp};
use crate::layout::{Layout, Size};
use crate::widget::{Canvas, Widget, WidgetHandle};

pub struct Group {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub width: Size,
    pub height: Size,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub layout: Layout,
    pub z: i32,
    pub background: Option<[f32; 4]>,
    pub draggable: bool,
    pub scrollable: bool,
    pub clip: bool,

    dragging: bool,
    drag_offset_x: f32,
    drag_offset_y: f32,
}

impl Group {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            width: Size::Auto,
            height: Size::Auto,
            scroll_x: 0.0,
            scroll_y: 0.0,
            layout: Layout::None,
            z: 0,
            background: None,
            draggable: false,
            scrollable: false,
            dragging: false,
            drag_offset_x: 0.0,
            drag_offset_y: 0.0,
            clip: false,
        }
    }

    pub fn set_x(&mut self, x: f32) {
        if self.x == x {
            return;
        }
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        if self.y == y {
            return;
        }
        self.y = y;
    }
    pub fn set_scroll_x(&mut self, x: f32) {
        if self.scroll_x == x {
            return;
        }
        self.scroll_x = x;
    }
    pub fn set_scroll_y(&mut self, y: f32) {
        if self.scroll_y == y {
            return;
        }
        self.scroll_y = y;
    }
    pub fn set_z(&mut self, z: i32) {
        if self.z == z {
            return;
        }
        self.z = z;
    }
}

impl Widget for Group {
    fn name(&self) -> &str {
        "Group"
    }
    fn build(&mut self, ui: &mut Ui, handle: WidgetHandle<()>) {
        let handle = handle.typed::<Group>();

        ui.listen(handle, move |ev: &MouseDown, ui: &mut Ui| {
            if let Some(g) = ui.get_mut(handle) {
                if !g.draggable {
                    return;
                }
                g.drag_offset_x = ev.x - g.x;
                g.drag_offset_y = ev.y - g.y;
                g.dragging = true;
                ui.capture_mouse(handle);
            }
        });

        ui.listen(handle, move |ev: &MouseMove, ui: &mut Ui| {
            if let Some(g) = ui.get_mut(handle) {
                if !g.dragging {
                    return;
                }
                g.x = ev.x - g.drag_offset_x;
                g.y = ev.y - g.drag_offset_y;
            }
            ui.request_layout(handle);
            ui.request_redraw();
        });

        ui.listen(handle, move |_: &MouseUp, ui: &mut Ui| {
            if let Some(g) = ui.get_mut(handle) {
                g.dragging = false;
            }
            ui.release_mouse();
        });
        ui.listen(handle, move |ev: &MouseScroll, ui: &mut Ui| {
            if let Some(g) = ui.get_mut(handle) {
                if !g.scrollable {
                    return;
                }
                g.scroll_x += ev.x * 20.0;
                g.scroll_y += ev.y * 20.0;
            }
            ui.request_redraw();
        });
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
        if let Some(color) = self.background {
            canvas.draw_list.push_rect(RectDraw {
                x: canvas.x,
                y: canvas.y,
                w: self.w,
                h: self.h,
                color,
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
