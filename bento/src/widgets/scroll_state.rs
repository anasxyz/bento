use crate::color::Color;
use crate::input::MouseButton;
use bento_wgpu::{ClipId, RectId, SceneGraph, TransformId};

const DEFAULT_SCROLLBAR_WIDTH: f32 = 8.0;
const DEFAULT_THUMB_MIN_SIZE: f32 = 20.0;
const DEFAULT_SCROLL_SPEED: f32 = 20.0;
const DEFAULT_TRACK_COLOR: Color = Color::new(0.15, 0.15, 0.15, 1.0);
const DEFAULT_THUMB_COLOR: Color = Color::new(0.45, 0.45, 0.45, 1.0);
const DEFAULT_THUMB_ACTIVE: Color = Color::new(0.65, 0.65, 0.65, 1.0);
const DEFAULT_THUMB_RADIUS: f32 = 3.0;

pub struct ScrollState {
    pub scroll_x: f32,
    pub scroll_y: f32,

    // config
    pub scroll_x_enabled: bool,
    pub scroll_y_enabled: bool,
    pub scrollbar_width: f32,
    pub scroll_speed: f32,
    pub track_color: Color,
    pub thumb_color: Color,
    pub thumb_active_color: Color,
    pub thumb_radius: f32,

    // own dimensions
    width: f32,
    height: f32,

    // vertical drag
    dragging_v: bool,
    drag_start_y: f32,
    drag_start_scroll_y: f32,

    // horizontal drag
    dragging_h: bool,
    drag_start_x: f32,
    drag_start_scroll_x: f32,

    // computed in sync
    v_track_x: f32,
    v_track_y: f32,
    v_track_h: f32,
    v_thumb_y: f32,
    v_thumb_h: f32,

    h_track_x: f32,
    h_track_y: f32,
    h_track_w: f32,
    h_thumb_x: f32,
    h_thumb_w: f32,
}

impl ScrollState {
    pub fn new() -> Self {
        Self {
            scroll_x: 0.0,
            scroll_y: 0.0,

            scroll_x_enabled: true,
            scroll_y_enabled: true,
            scrollbar_width: DEFAULT_SCROLLBAR_WIDTH,
            scroll_speed: DEFAULT_SCROLL_SPEED,
            track_color: DEFAULT_TRACK_COLOR,
            thumb_color: DEFAULT_THUMB_COLOR,
            thumb_active_color: DEFAULT_THUMB_ACTIVE,
            thumb_radius: DEFAULT_THUMB_RADIUS,

            width: 0.0,
            height: 0.0,

            dragging_v: false,
            drag_start_y: 0.0,
            drag_start_scroll_y: 0.0,

            dragging_h: false,
            drag_start_x: 0.0,
            drag_start_scroll_x: 0.0,

            v_track_x: 0.0,
            v_track_y: 0.0,
            v_track_h: 0.0,
            v_thumb_y: 0.0,
            v_thumb_h: 0.0,
            h_track_x: 0.0,
            h_track_y: 0.0,
            h_track_w: 0.0,
            h_thumb_x: 0.0,
            h_thumb_w: 0.0,
        }
    }

    // config setters 

    pub fn set_scroll_x_enabled(&mut self, v: bool) -> &mut Self {
        self.scroll_x_enabled = v;
        self
    }
    pub fn set_scroll_y_enabled(&mut self, v: bool) -> &mut Self {
        self.scroll_y_enabled = v;
        self
    }
    pub fn set_scrollbar_width(&mut self, v: f32) -> &mut Self {
        self.scrollbar_width = v;
        self
    }
    pub fn set_scroll_speed(&mut self, v: f32) -> &mut Self {
        self.scroll_speed = v;
        self
    }
    pub fn set_track_color(&mut self, c: Color) -> &mut Self {
        self.track_color = c;
        self
    }
    pub fn set_thumb_color(&mut self, c: Color) -> &mut Self {
        self.thumb_color = c;
        self
    }
    pub fn set_thumb_active_color(&mut self, c: Color) -> &mut Self {
        self.thumb_active_color = c;
        self
    }
    pub fn set_thumb_radius(&mut self, v: f32) -> &mut Self {
        self.thumb_radius = v;
        self
    }

    // event handlers 

    pub fn on_scroll(&mut self, dx: f32, dy: f32, content_w: f32, content_h: f32) {
        if self.scroll_y_enabled {
            let max_y = (content_h - self.height).max(0.0);
            if max_y > 0.0 {
                self.scroll_y = (self.scroll_y + dy * self.scroll_speed).clamp(0.0, max_y);
            }
        }
        if self.scroll_x_enabled {
            let max_x = (content_w - self.width).max(0.0);
            if max_x > 0.0 {
                self.scroll_x = (self.scroll_x + dx * self.scroll_speed).clamp(0.0, max_x);
            }
        }
    }

    pub fn on_press(
        &mut self,
        mx: f32,
        my: f32,
        button: MouseButton,
        content_w: f32,
        content_h: f32,
    ) {
        if button != MouseButton::Left {
            return;
        }

        if self.scroll_y_enabled {
            // vertical thumb drag
            if mx >= self.v_track_x
                && mx <= self.v_track_x + self.scrollbar_width
                && my >= self.v_thumb_y
                && my <= self.v_thumb_y + self.v_thumb_h
            {
                self.dragging_v = true;
                self.drag_start_y = my;
                self.drag_start_scroll_y = self.scroll_y;
                return;
            }
            // vertical track click
            if mx >= self.v_track_x
                && mx <= self.v_track_x + self.scrollbar_width
                && my >= self.v_track_y
                && my <= self.v_track_y + self.v_track_h
            {
                let max_scroll = (content_h - self.height).max(0.0);
                let ratio = (my - self.v_track_y) / self.v_track_h;
                self.scroll_y = (ratio * max_scroll).clamp(0.0, max_scroll);
                return;
            }
        }

        if self.scroll_x_enabled {
            // horizontal thumb drag
            if my >= self.h_track_y
                && my <= self.h_track_y + self.scrollbar_width
                && mx >= self.h_thumb_x
                && mx <= self.h_thumb_x + self.h_thumb_w
            {
                self.dragging_h = true;
                self.drag_start_x = mx;
                self.drag_start_scroll_x = self.scroll_x;
                return;
            }
            // horizontal track click
            if my >= self.h_track_y
                && my <= self.h_track_y + self.scrollbar_width
                && mx >= self.h_track_x
                && mx <= self.h_track_x + self.h_track_w
            {
                let max_scroll = (content_w - self.width).max(0.0);
                let ratio = (mx - self.h_track_x) / self.h_track_w;
                self.scroll_x = (ratio * max_scroll).clamp(0.0, max_scroll);
            }
        }
    }

    pub fn on_move(&mut self, mx: f32, my: f32, content_w: f32, content_h: f32) {
        if self.dragging_v {
            let track_h = self.v_track_h - self.v_thumb_h;
            let max_scroll = (content_h - self.height).max(0.0);
            if track_h > 0.0 {
                let delta = my - self.drag_start_y;
                let scroll_delta = delta / track_h * max_scroll;
                self.scroll_y = (self.drag_start_scroll_y + scroll_delta).clamp(0.0, max_scroll);
            }
        }
        if self.dragging_h {
            let track_w = self.h_track_w - self.h_thumb_w;
            let max_scroll = (content_w - self.width).max(0.0);
            if track_w > 0.0 {
                let delta = mx - self.drag_start_x;
                let scroll_delta = delta / track_w * max_scroll;
                self.scroll_x = (self.drag_start_scroll_x + scroll_delta).clamp(0.0, max_scroll);
            }
        }
    }

    pub fn on_release(&mut self) {
        self.dragging_v = false;
        self.dragging_h = false;
    }

    // sync

    pub fn sync(
        &mut self,
        scene: &mut SceneGraph,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        content_w: f32,
        content_h: f32,
        clip_id: ClipId,
        transform_id: TransformId,
        v_track_id: RectId,
        v_thumb_id: RectId,
        h_track_id: RectId,
        h_thumb_id: RectId,
    ) -> (f32, f32) {
        self.width = w;
        self.height = h;

        let sw = self.scrollbar_width;
        let show_v = self.scroll_y_enabled && content_h > h;
        let show_h = self.scroll_x_enabled && content_w > w;
        let inner_w = if show_v { w - sw } else { w };
        let inner_h = if show_h { h - sw } else { h };

        let max_y = (content_h - inner_h).max(0.0);
        let max_x = (content_w - inner_w).max(0.0);
        self.scroll_y = self.scroll_y.clamp(0.0, max_y);
        self.scroll_x = self.scroll_x.clamp(0.0, max_x);

        scene.clip_mut(clip_id).set_rect(x, y, inner_w, inner_h);
        scene
            .transform_mut(transform_id)
            .set_offset(-self.scroll_x, -self.scroll_y);

        // vertical scrollbar
        if show_v {
            let track_h = inner_h;
            let thumb_h = (inner_h / content_h * track_h).max(DEFAULT_THUMB_MIN_SIZE);
            let thumb_y = if max_y > 0.0 {
                y + (self.scroll_y / max_y) * (track_h - thumb_h)
            } else {
                y
            };

            self.v_track_x = x + w - sw;
            self.v_track_y = y;
            self.v_track_h = track_h;
            self.v_thumb_y = thumb_y;
            self.v_thumb_h = thumb_h;

            let n = scene.rect_mut(v_track_id);
            n.set_rect(self.v_track_x, y, sw, track_h);
            n.set_color(self.track_color.to_array());
            n.set_visible(true);

            let n = scene.rect_mut(v_thumb_id);
            n.set_rect(self.v_track_x + 1.0, thumb_y, sw - 2.0, thumb_h);
            n.set_color(if self.dragging_v {
                self.thumb_active_color.to_array()
            } else {
                self.thumb_color.to_array()
            });
            n.set_radius(self.thumb_radius);
            n.set_visible(true);
        } else {
            scene.rect_mut(v_track_id).set_visible(false);
            scene.rect_mut(v_thumb_id).set_visible(false);
        }

        // horizontal scrollbar
        if show_h {
            let track_w = inner_w;
            let thumb_w = (inner_w / content_w * track_w).max(DEFAULT_THUMB_MIN_SIZE);
            let thumb_x = if max_x > 0.0 {
                x + (self.scroll_x / max_x) * (track_w - thumb_w)
            } else {
                x
            };

            self.h_track_x = x;
            self.h_track_y = y + h - sw;
            self.h_track_w = track_w;
            self.h_thumb_x = thumb_x;
            self.h_thumb_w = thumb_w;

            let n = scene.rect_mut(h_track_id);
            n.set_rect(x, self.h_track_y, track_w, sw);
            n.set_color(self.track_color.to_array());
            n.set_visible(true);

            let n = scene.rect_mut(h_thumb_id);
            n.set_rect(thumb_x, self.h_track_y + 1.0, thumb_w, sw - 2.0);
            n.set_color(if self.dragging_h {
                self.thumb_active_color.to_array()
            } else {
                self.thumb_color.to_array()
            });
            n.set_radius(self.thumb_radius);
            n.set_visible(true);
        } else {
            scene.rect_mut(h_track_id).set_visible(false);
            scene.rect_mut(h_thumb_id).set_visible(false);
        }

        (inner_w, inner_h)
    }
}

impl Default for ScrollState {
    fn default() -> Self {
        Self::new()
    }
}
