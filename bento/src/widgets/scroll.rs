use crate::color::Color;
use crate::input::MouseButton;
use crate::widget::{AsAny, Base, HasBase, Widget};
use bento_derive::Widget;
use bento_wgpu::{ClipId, RectId, SceneGraph, SceneNodeId, TransformId};

const SCROLLBAR_SIZE: f32 = 8.0;
const THUMB_MIN_SIZE: f32 = 20.0;
const SCROLLBAR_TRACK_COLOR: [f32; 4] = [0.15, 0.15, 0.15, 1.0];
const SCROLLBAR_THUMB_COLOR: [f32; 4] = [0.45, 0.45, 0.45, 1.0];
const SCROLLBAR_THUMB_HOVER: [f32; 4] = [0.65, 0.65, 0.65, 1.0];

#[derive(Widget)]
pub struct ScrollContainer {
    pub base: Base,
    pub scroll_y: f32,
    pub scroll_x: f32,
    color: Color,

    // own dimensions set in sync
    width: f32,
    height: f32,

    // vertical drag state
    dragging_v: bool,
    drag_start_y: f32,
    drag_start_scroll_y: f32,

    // horizontal drag state
    dragging_h: bool,
    drag_start_x: f32,
    drag_start_scroll_x: f32,

    // computed in sync
    // used by event hooks for hit testing
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

    // scene nodes
    rect_id: Option<RectId>,
    clip_id: Option<ClipId>,
    transform_id: Option<TransformId>,
    v_track_id: Option<RectId>,
    v_thumb_id: Option<RectId>,
    h_track_id: Option<RectId>,
    h_thumb_id: Option<RectId>,
}

impl ScrollContainer {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            scroll_y: 0.0,
            scroll_x: 0.0,
            color: Color::TRANSPARENT,
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

            rect_id: None,
            clip_id: None,
            transform_id: None,
            v_track_id: None,
            v_thumb_id: None,
            h_track_id: None,
            h_thumb_id: None,
        }
    }

    pub fn set_color(&mut self, c: Color) -> &mut Self {
        self.color = c;
        self
    }
}

impl Default for ScrollContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ScrollContainer {
    fn build(&mut self, scene: &mut SceneGraph) {
        self.rect_id = Some(scene.add_rect());
        self.clip_id = Some(scene.add_clip());
        self.transform_id = Some(scene.add_transform());
        self.v_track_id = Some(scene.add_rect());
        self.v_thumb_id = Some(scene.add_rect());
        self.h_track_id = Some(scene.add_rect());
        self.h_thumb_id = Some(scene.add_rect());

        let clip = self.clip_id.unwrap();
        let transform = self.transform_id.unwrap();
        scene.add_child(SceneNodeId(clip.0), SceneNodeId(transform.0));
    }

    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, h: f32) {
        self.width = w;
        self.height = h;

        let content_h = self.base.content_height;
        let content_w = self.base.content_width;
        let show_v = content_h > h;
        let show_h = content_w > w;

        let inner_w = if show_v { w - SCROLLBAR_SIZE } else { w };
        let inner_h = if show_h { h - SCROLLBAR_SIZE } else { h };

        // clamp scroll
        let max_scroll_y = (content_h - inner_h).max(0.0);
        let max_scroll_x = (content_w - inner_w).max(0.0);
        self.scroll_y = self.scroll_y.clamp(0.0, max_scroll_y);
        self.scroll_x = self.scroll_x.clamp(0.0, max_scroll_x);

        // background
        if let Some(id) = self.rect_id {
            let n = scene.rect_mut(id);
            n.set_rect(x, y, w, h);
            n.set_color(self.color.to_array());
            n.set_visible(true);
        }

        // clip to inner area
        if let Some(id) = self.clip_id {
            scene.clip_mut(id).set_rect(x, y, inner_w, inner_h);
        }

        // scroll transform
        if let Some(id) = self.transform_id {
            scene
                .transform_mut(id)
                .set_offset(-self.scroll_x, -self.scroll_y);
        }

        // vertical scrollbar
        if show_v {
            let track_h = inner_h;
            let thumb_h = (inner_h / content_h * track_h).max(THUMB_MIN_SIZE);
            let thumb_y = if max_scroll_y > 0.0 {
                y + (self.scroll_y / max_scroll_y) * (track_h - thumb_h)
            } else {
                y
            };

            self.v_track_x = x + w - SCROLLBAR_SIZE;
            self.v_track_y = y;
            self.v_track_h = track_h;
            self.v_thumb_y = thumb_y;
            self.v_thumb_h = thumb_h;

            if let Some(id) = self.v_track_id {
                let n = scene.rect_mut(id);
                n.set_rect(self.v_track_x, y, SCROLLBAR_SIZE, track_h);
                n.set_color(SCROLLBAR_TRACK_COLOR);
                n.set_visible(true);
            }
            if let Some(id) = self.v_thumb_id {
                let n = scene.rect_mut(id);
                n.set_rect(self.v_track_x + 1.0, thumb_y, SCROLLBAR_SIZE - 2.0, thumb_h);
                n.set_color(if self.dragging_v {
                    SCROLLBAR_THUMB_HOVER
                } else {
                    SCROLLBAR_THUMB_COLOR
                });
                n.set_radius(3.0);
                n.set_visible(true);
            }
        } else {
            if let Some(id) = self.v_track_id {
                scene.rect_mut(id).set_visible(false);
            }
            if let Some(id) = self.v_thumb_id {
                scene.rect_mut(id).set_visible(false);
            }
        }

        // horizontal scrollbar
        if show_h {
            let track_w = inner_w;
            let thumb_w = (inner_w / content_w * track_w).max(THUMB_MIN_SIZE);
            let thumb_x = if max_scroll_x > 0.0 {
                x + (self.scroll_x / max_scroll_x) * (track_w - thumb_w)
            } else {
                x
            };

            self.h_track_x = x;
            self.h_track_y = y + h - SCROLLBAR_SIZE;
            self.h_track_w = track_w;
            self.h_thumb_x = thumb_x;
            self.h_thumb_w = thumb_w;

            if let Some(id) = self.h_track_id {
                let n = scene.rect_mut(id);
                n.set_rect(x, self.h_track_y, track_w, SCROLLBAR_SIZE);
                n.set_color(SCROLLBAR_TRACK_COLOR);
                n.set_visible(true);
            }
            if let Some(id) = self.h_thumb_id {
                let n = scene.rect_mut(id);
                n.set_rect(thumb_x, self.h_track_y + 1.0, thumb_w, SCROLLBAR_SIZE - 2.0);
                n.set_color(if self.dragging_h {
                    SCROLLBAR_THUMB_HOVER
                } else {
                    SCROLLBAR_THUMB_COLOR
                });
                n.set_radius(3.0);
                n.set_visible(true);
            }
        } else {
            if let Some(id) = self.h_track_id {
                scene.rect_mut(id).set_visible(false);
            }
            if let Some(id) = self.h_thumb_id {
                scene.rect_mut(id).set_visible(false);
            }
        }
    }

    fn children_attachment_node(&self) -> Option<SceneNodeId> {
        self.transform_id.map(|id| SceneNodeId(id.0))
    }

    fn on_mouse_press(&mut self, mx: f32, my: f32, button: MouseButton) {
        if button != MouseButton::Left {
            return;
        }

        // vertical thumb drag
        if mx >= self.v_track_x
            && mx <= self.v_track_x + SCROLLBAR_SIZE
            && my >= self.v_thumb_y
            && my <= self.v_thumb_y + self.v_thumb_h
        {
            self.dragging_v = true;
            self.drag_start_y = my;
            self.drag_start_scroll_y = self.scroll_y;
            return;
        }

        // click on vertical track, jump to position
        if mx >= self.v_track_x
            && mx <= self.v_track_x + SCROLLBAR_SIZE
            && my >= self.v_track_y
            && my <= self.v_track_y + self.v_track_h
        {
            let max_scroll = (self.base.content_height - self.height).max(0.0);
            let ratio = (my - self.v_track_y) / self.v_track_h;
            self.scroll_y = (ratio * max_scroll).clamp(0.0, max_scroll);
            return;
        }

        // horizontal thumb drag
        if my >= self.h_track_y
            && my <= self.h_track_y + SCROLLBAR_SIZE
            && mx >= self.h_thumb_x
            && mx <= self.h_thumb_x + self.h_thumb_w
        {
            self.dragging_h = true;
            self.drag_start_x = mx;
            self.drag_start_scroll_x = self.scroll_x;
            return;
        }

        // click on horizontal track, jump to position
        if my >= self.h_track_y
            && my <= self.h_track_y + SCROLLBAR_SIZE
            && mx >= self.h_track_x
            && mx <= self.h_track_x + self.h_track_w
        {
            let max_scroll = (self.base.content_width - self.width).max(0.0);
            let ratio = (mx - self.h_track_x) / self.h_track_w;
            self.scroll_x = (ratio * max_scroll).clamp(0.0, max_scroll);
        }
    }

    fn on_mouse_move(&mut self, mx: f32, my: f32) {
        if self.dragging_v {
            let track_h = self.v_track_h - self.v_thumb_h;
            let max_scroll = (self.base.content_height - self.height).max(0.0);
            if track_h > 0.0 {
                let delta = my - self.drag_start_y;
                let scroll_delta = delta / track_h * max_scroll;
                self.scroll_y = (self.drag_start_scroll_y + scroll_delta).clamp(0.0, max_scroll);
            }
        }
        if self.dragging_h {
            let track_w = self.h_track_w - self.h_thumb_w;
            let max_scroll = (self.base.content_width - self.width).max(0.0);
            if track_w > 0.0 {
                let delta = mx - self.drag_start_x;
                let scroll_delta = delta / track_w * max_scroll;
                self.scroll_x = (self.drag_start_scroll_x + scroll_delta).clamp(0.0, max_scroll);
            }
        }
    }

    fn on_mouse_release(&mut self, _mx: f32, _my: f32, _button: MouseButton) {
        self.dragging_v = false;
        self.dragging_h = false;
    }

    fn on_mouse_scroll(&mut self, dx: f32, dy: f32) {
        let max_scroll_y = (self.base.content_height - self.height).max(0.0);
        let max_scroll_x = (self.base.content_width - self.width).max(0.0);
        if max_scroll_y > 0.0 {
            self.scroll_y = (self.scroll_y + dy * 20.0).clamp(0.0, max_scroll_y);
        }
        if max_scroll_x > 0.0 {
            self.scroll_x = (self.scroll_x + dx * 20.0).clamp(0.0, max_scroll_x);
        }
    }
}
