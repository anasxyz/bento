// reusable scrollbar primitive, its not a traditional element
// owns scroll position and drag state and the owner just reads scroll_y  amd scroll_x to offset their content
// owner forwards mouse events to the scrollbar methods

use crate::color::Color;
use crate::render::DrawCall;

const MIN_THUMB_SIZE: f32 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dragging {
    None,
    Vertical,
    Horizontal,
}

#[derive(Clone)]
pub struct Scrollbar {
    // scroll position
    pub scroll_x: f32,
    pub scroll_y: f32,

    // content and viewport dimensions
    pub content_width: f32,
    pub content_height: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,

    // axes
    pub vertical: bool,
    pub horizontal: bool,

    // appearance
    pub width: f32,
    pub color: Color,
    pub track_color: Color,
    pub visible: bool,

    // scroll speed
    pub speed: f32,

    // drag state
    dragging: Dragging,
    drag_offset: f32,
}

impl Scrollbar {
    pub fn new() -> Self {
        Self {
            scroll_x: 0.0,
            scroll_y: 0.0,
            content_width: 0.0,
            content_height: 0.0,
            viewport_width: 0.0,
            viewport_height: 0.0,
            vertical: true,
            horizontal: false,
            width: 12.0,
            color: Color::rgba(255, 255, 255, 80),
            track_color: Color::rgba(255, 255, 255, 20),
            visible: true,
            speed: 40.0,
            dragging: Dragging::None,
            drag_offset: 0.0,
        }
    }

    pub fn max_scroll_y(&self) -> f32 {
        (self.content_height - self.viewport_height).max(0.0)
    }
    pub fn max_scroll_x(&self) -> f32 {
        (self.content_width - self.viewport_width).max(0.0)
    }
    pub fn is_dragging(&self) -> bool {
        self.dragging != Dragging::None
    }

    pub fn set_scroll_y(&mut self, y: f32) {
        self.scroll_y = y.clamp(0.0, self.max_scroll_y());
    }
    pub fn set_scroll_x(&mut self, x: f32) {
        self.scroll_x = x.clamp(0.0, self.max_scroll_x());
    }
    pub fn scroll_to_y(&mut self, y: f32) {
        self.set_scroll_y(y);
    }
    pub fn scroll_to_top(&mut self) {
        self.scroll_y = 0.0;
    }
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_y = self.max_scroll_y();
    }

    // geometry

    fn v_track(&self, rx: f32, ry: f32, rw: f32, rh: f32) -> (f32, f32, f32, f32) {
        // 2.0 is gap from right
        let x = rx + rw - self.width - 0.0;
        // 2.0 is gap from top
        let y = ry + 0.0;
        let h = rh
            - if self.horizontal && self.content_width > self.viewport_width {
                self.width + 0.0 // 4.0 is gap from top + gap from bottom
            } else {
                0.0
            };
        (x, y, self.width, h)
    }

    fn v_thumb(&self, rx: f32, ry: f32, rw: f32, rh: f32) -> (f32, f32, f32, f32) {
        let (tx, ty, tw, th) = self.v_track(rx, ry, rw, rh);
        let ratio = (self.viewport_height / self.content_height).min(1.0);
        let thumb_h = (th * ratio).max(MIN_THUMB_SIZE);
        let thumb_y = ty + (th - thumb_h) * (self.scroll_y / self.max_scroll_y().max(1.0));
        (tx, thumb_y, tw, thumb_h)
    }

    fn h_track(&self, rx: f32, ry: f32, rw: f32, rh: f32) -> (f32, f32, f32, f32) {
        // let x = rx + 2.0 (2.0 is gap from left)
        let x = rx + 2.0;
        // let y = ry + rh - self.width - 2.0 (2.0 is gap from bottom)
        let y = ry + rh - self.width - 2.0;
        let w = rw
            - if self.vertical && self.content_height > self.viewport_height {
                self.width + 4.0
            } else {
                4.0
            };
        (x, y, w, self.width)
    }

    fn h_thumb(&self, rx: f32, ry: f32, rw: f32, rh: f32) -> (f32, f32, f32, f32) {
        let (tx, ty, tw, th) = self.h_track(rx, ry, rw, rh);
        let ratio = (self.viewport_width / self.content_width).min(1.0);
        let thumb_w = (tw * ratio).max(MIN_THUMB_SIZE);
        let thumb_x = tx + (tw - thumb_w) * (self.scroll_x / self.max_scroll_x().max(1.0));
        (thumb_x, ty, thumb_w, th)
    }

    // drawing

    // rect = (x, y, w, h) of the element that owns this scrollbar
    pub fn draw_calls(
        &self,
        rect: (f32, f32, f32, f32),
        clip: Option<[f32; 4]>,
        z: i32,
        opacity: f32,
    ) -> Vec<DrawCall> {
        let (rx, ry, rw, rh) = rect;
        let mut calls = Vec::new();
        if !self.visible {
            return calls;
        }
        let r = 0.0;

        let show_v = self.vertical && self.content_height > self.viewport_height;
        let show_h = self.horizontal && self.content_width > self.viewport_width;

        if show_v {
            let track = self.v_track(rx, ry, rw, rh);
            let thumb = self.v_thumb(rx, ry, rw, rh);
            let mut tc = self.track_color.to_array();
            tc[3] *= opacity;
            calls.push(DrawCall::Rect {
                x: track.0,
                y: track.1,
                w: track.2,
                h: track.3,
                color: tc,
                radius: r,
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                clip,
                z_index: z,
            });
            let thumb_base = if self.dragging == Dragging::Vertical {
                Color::rgba(255, 255, 255, 130)
            } else {
                self.color
            };
            let mut sc = thumb_base.to_array();
            sc[3] *= opacity;
            calls.push(DrawCall::Rect {
                x: thumb.0,
                y: thumb.1,
                w: thumb.2,
                h: thumb.3,
                color: sc,
                radius: r,
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                clip,
                z_index: z + 1,
            });
        }

        if show_h {
            let track = self.h_track(rx, ry, rw, rh);
            let thumb = self.h_thumb(rx, ry, rw, rh);
            let mut tc = self.track_color.to_array();
            tc[3] *= opacity;
            calls.push(DrawCall::Rect {
                x: track.0,
                y: track.1,
                w: track.2,
                h: track.3,
                color: tc,
                radius: r,
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                clip,
                z_index: z,
            });
            let thumb_base = if self.dragging == Dragging::Horizontal {
                Color::rgba(255, 255, 255, 130)
            } else {
                self.color
            };
            let mut sc = thumb_base.to_array();
            sc[3] *= opacity;
            calls.push(DrawCall::Rect {
                x: thumb.0,
                y: thumb.1,
                w: thumb.2,
                h: thumb.3,
                color: sc,
                radius: r,
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                clip,
                z_index: z + 1,
            });
        }

        calls
    }

    // input
    // owner forwards mouse events here
    // returns true if the event was consumed (owner should return Handled)

    pub fn on_mouse_press(&mut self, rect: (f32, f32, f32, f32), x: f32, y: f32) -> bool {
        let (rx, ry, rw, rh) = rect;

        if self.vertical && self.content_height > self.viewport_height {
            let (tx, ty, tw, th) = self.v_thumb(rx, ry, rw, rh);
            if x >= tx && x <= tx + tw && y >= ty && y <= ty + th {
                self.dragging = Dragging::Vertical;
                self.drag_offset = y - ty;
                return true;
            }
            let (trx, try_, trw, trh) = self.v_track(rx, ry, rw, rh);
            if x >= trx && x <= trx + trw && y >= try_ && y <= try_ + trh {
                let ratio = (self.viewport_height / self.content_height).min(1.0);
                let th2 = (trh * ratio).max(MIN_THUMB_SIZE);
                let target = y - try_ - th2 / 2.0;
                self.scroll_y = (target / (trh - th2).max(1.0) * self.max_scroll_y())
                    .clamp(0.0, self.max_scroll_y());
                return true;
            }
        }

        if self.horizontal && self.content_width > self.viewport_width {
            let (tx, ty, tw, th) = self.h_thumb(rx, ry, rw, rh);
            if x >= tx && x <= tx + tw && y >= ty && y <= ty + th {
                self.dragging = Dragging::Horizontal;
                self.drag_offset = x - tx;
                return true;
            }
            let (trx, try_, trw, trh) = self.h_track(rx, ry, rw, rh);
            if x >= trx && x <= trx + trw && y >= try_ && y <= try_ + trh {
                let ratio = (self.viewport_width / self.content_width).min(1.0);
                let tw2 = (trw * ratio).max(MIN_THUMB_SIZE);
                let target = x - trx - tw2 / 2.0;
                self.scroll_x = (target / (trw - tw2).max(1.0) * self.max_scroll_x())
                    .clamp(0.0, self.max_scroll_x());
                return true;
            }
        }

        false
    }

    pub fn on_mouse_release(&mut self) -> bool {
        if self.dragging != Dragging::None {
            self.dragging = Dragging::None;
            return true;
        }
        false
    }

    pub fn on_mouse_move(&mut self, rect: (f32, f32, f32, f32), x: f32, y: f32) -> bool {
        let (rx, ry, rw, rh) = rect;
        match self.dragging {
            Dragging::Vertical => {
                let (_, try_, _, trh) = self.v_track(rx, ry, rw, rh);
                let ratio = (self.viewport_height / self.content_height).min(1.0);
                let thumb_h = (trh * ratio).max(MIN_THUMB_SIZE);
                let thumb_top = y - self.drag_offset;
                let scroll_ratio = (thumb_top - try_) / (trh - thumb_h).max(1.0);
                self.scroll_y =
                    (scroll_ratio * self.max_scroll_y()).clamp(0.0, self.max_scroll_y());
                true
            }
            Dragging::Horizontal => {
                let (trx, _, trw, _) = self.h_track(rx, ry, rw, rh);
                let ratio = (self.viewport_width / self.content_width).min(1.0);
                let thumb_w = (trw * ratio).max(MIN_THUMB_SIZE);
                let thumb_left = x - self.drag_offset;
                let scroll_ratio = (thumb_left - trx) / (trw - thumb_w).max(1.0);
                self.scroll_x =
                    (scroll_ratio * self.max_scroll_x()).clamp(0.0, self.max_scroll_x());
                true
            }
            Dragging::None => false,
        }
    }

    pub fn on_scroll(&mut self, delta_x: f32, delta_y: f32) -> bool {
        let mut changed = false;
        if self.vertical {
            let new_y = (self.scroll_y + delta_y * self.speed).clamp(0.0, self.max_scroll_y());
            if new_y != self.scroll_y {
                self.scroll_y = new_y;
                changed = true;
            }
        }
        if self.horizontal {
            let new_x = (self.scroll_x + delta_x * self.speed).clamp(0.0, self.max_scroll_x());
            if new_x != self.scroll_x {
                self.scroll_x = new_x;
                changed = true;
            }
        }
        changed
    }
}
