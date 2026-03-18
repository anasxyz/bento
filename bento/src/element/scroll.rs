use crate::color::Color;
use crate::element::base::Base;
use crate::element::element::{Element, EventResult};
use crate::element::layout::Layout;
use crate::input::{Key, Modifiers, MouseButton};
use crate::render::DrawCall;
use bento_derive::Element;

const SCROLL_SPEED: f32 = 40.0;
const SCROLLBAR_WIDTH: f32 = 6.0;
const SCROLLBAR_MIN_HEIGHT: f32 = 24.0;

#[derive(Element)]
pub struct ScrollContainer {
    base: Base,

    pub scroll_x: f32,
    pub scroll_y: f32,
    pub content_width: f32,
    pub content_height: f32,

    pub scroll_x_enabled: bool,
    pub scroll_y_enabled: bool,
    pub scroll_speed: f32,

    pub scrollbar_visible: bool,
    pub scrollbar_width: f32,
    pub scrollbar_color: Color,
    pub scrollbar_track_color: Color,

    pub bg_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,
    pub border_widths: [f32; 4],

    dragging: bool,
    drag_offset: f32,
}

impl ScrollContainer {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            scroll_x: 0.0,
            scroll_y: 0.0,
            content_width: 0.0,
            content_height: 0.0,
            scroll_x_enabled: false,
            scroll_y_enabled: true,
            scroll_speed: SCROLL_SPEED,
            scrollbar_visible: true,
            scrollbar_width: SCROLLBAR_WIDTH,
            scrollbar_color: Color::rgba(255, 255, 255, 80),
            scrollbar_track_color: Color::rgba(255, 255, 255, 20),
            bg_color: None,
            border_color: None,
            border_radius: None,
            border_widths: [0.0; 4],
            dragging: false,
            drag_offset: 0.0,
        }
    }

    pub fn set_bg_color(&mut self, color: Option<Color>) -> &mut Self {
        self.bg_color = color;
        self.base.dirty = true;
        self
    }
    pub fn set_border_color(&mut self, color: Option<Color>) -> &mut Self {
        self.border_color = color;
        self.base.dirty = true;
        self
    }
    pub fn set_border_radius(&mut self, radius: Option<f32>) -> &mut Self {
        self.border_radius = radius;
        self.base.dirty = true;
        self
    }
    pub fn set_border(&mut self, widths: [f32; 4]) -> &mut Self {
        self.border_widths = widths;
        self.base.dirty = true;
        self
    }
    pub fn set_scroll_speed(&mut self, speed: f32) -> &mut Self {
        self.scroll_speed = speed;
        self
    }
    pub fn set_scrollbar_visible(&mut self, v: bool) -> &mut Self {
        self.scrollbar_visible = v;
        self.base.dirty = true;
        self
    }
    pub fn set_scroll_x_enabled(&mut self, v: bool) -> &mut Self {
        self.scroll_x_enabled = v;
        self
    }
    pub fn set_scroll_y_enabled(&mut self, v: bool) -> &mut Self {
        self.scroll_y_enabled = v;
        self
    }
    pub fn scroll_to_top(&mut self) {
        self.scroll_y = 0.0;
        self.apply_transform();
    }
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_y = self.max_scroll_y();
        self.apply_transform();
    }
    pub fn set_scroll_y(&mut self, y: f32) {
        self.scroll_y = y.clamp(0.0, self.max_scroll_y());
        self.apply_transform();
    }
    pub fn set_content_size(&mut self, w: f32, h: f32) {
        self.content_width = w;
        self.content_height = h;
        self.base.dirty = true;
    }

    fn max_scroll_y(&self) -> f32 {
        (self.content_height - self.base.layout.h).max(0.0)
    }
    fn max_scroll_x(&self) -> f32 {
        (self.content_width - self.base.layout.w).max(0.0)
    }
    fn apply_transform(&mut self) {
        self.base.layout.transform = Some((-self.scroll_x, -self.scroll_y));
        self.base.dirty = true;
    }

    fn thumb_geometry(&self) -> (f32, f32, f32, f32) {
        let l = &self.base.layout;
        let track_x = l.x + l.w - self.scrollbar_width - 2.0;
        let track_y = l.y + 2.0;
        let track_h = l.h - 4.0;
        let thumb_ratio = (l.h / self.content_height).min(1.0);
        let thumb_h = (track_h * thumb_ratio).max(SCROLLBAR_MIN_HEIGHT);
        let thumb_y =
            track_y + (track_h - thumb_h) * (self.scroll_y / self.max_scroll_y().max(1.0));
        (track_x, thumb_y, self.scrollbar_width, thumb_h)
    }

    fn track_geometry(&self) -> (f32, f32, f32, f32) {
        let l = &self.base.layout;
        let track_x = l.x + l.w - self.scrollbar_width - 2.0;
        let track_y = l.y + 2.0;
        let track_h = l.h - 4.0;
        (track_x, track_y, self.scrollbar_width, track_h)
    }
}

impl Element for ScrollContainer {
    fn draw_calls(&self, clip: Option<[f32; 4]>, z: i32, opacity: f32) -> Vec<DrawCall> {
        let l = &self.base.layout;
        let mut calls = Vec::new();

        if let Some(bg) = self.bg_color {
            let mut color = bg.to_array();
            color[3] *= opacity;
            let mut border_color = self.border_color.unwrap_or(Color::BLACK).to_array();
            border_color[3] *= opacity;
            calls.push(DrawCall::Rect {
                x: l.x,
                y: l.y,
                w: l.w,
                h: l.h,
                color,
                radius: self.border_radius.unwrap_or(0.0),
                border_color,
                border_widths: self.border_widths,
                clip,
                z_index: z,
            });
        }

        if self.scrollbar_visible && self.scroll_y_enabled && self.content_height > l.h {
            let (track_x, track_y, track_w, track_h) = self.track_geometry();
            let mut track_color = self.scrollbar_track_color.to_array();
            track_color[3] *= opacity;
            calls.push(DrawCall::Rect {
                x: track_x,
                y: track_y,
                w: track_w,
                h: track_h,
                color: track_color,
                radius: self.scrollbar_width / 2.0,
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                clip,
                z_index: z + 1,
            });

            let (thumb_x, thumb_y, thumb_w, thumb_h) = self.thumb_geometry();
            let thumb_base = if self.dragging {
                Color::rgba(255, 255, 255, 130)
            } else {
                self.scrollbar_color
            };
            let mut thumb_color = thumb_base.to_array();
            thumb_color[3] *= opacity;
            calls.push(DrawCall::Rect {
                x: thumb_x,
                y: thumb_y,
                w: thumb_w,
                h: thumb_h,
                color: thumb_color,
                radius: self.scrollbar_width / 2.0,
                border_color: [0.0; 4],
                border_widths: [0.0; 4],
                clip,
                z_index: z + 2,
            });
        }

        calls
    }

    fn on_mouse_press(&mut self, x: f32, y: f32, button: MouseButton) -> EventResult {
        if button != MouseButton::Left {
            return EventResult::Propagate;
        }
        if !self.scrollbar_visible || !self.scroll_y_enabled {
            return EventResult::Propagate;
        }
        if self.content_height <= self.base.layout.h {
            return EventResult::Propagate;
        }

        let (thumb_x, thumb_y, thumb_w, thumb_h) = self.thumb_geometry();
        if x >= thumb_x && x <= thumb_x + thumb_w && y >= thumb_y && y <= thumb_y + thumb_h {
            self.dragging = true;
            self.drag_offset = y - thumb_y;
            self.base.dirty = true;
            return EventResult::Handled;
        }

        let (track_x, track_y, track_w, track_h) = self.track_geometry();
        if x >= track_x && x <= track_x + track_w && y >= track_y && y <= track_y + track_h {
            let thumb_ratio = (self.base.layout.h / self.content_height).min(1.0);
            let thumb_h = (track_h * thumb_ratio).max(SCROLLBAR_MIN_HEIGHT);
            let target_y = y - track_y - thumb_h / 2.0;
            let scroll_ratio = target_y / (track_h - thumb_h).max(1.0);
            self.scroll_y = (scroll_ratio * self.max_scroll_y()).clamp(0.0, self.max_scroll_y());
            self.apply_transform();
            return EventResult::Handled;
        }

        EventResult::Propagate
    }

    fn on_mouse_release(&mut self, _x: f32, _y: f32, _button: MouseButton) -> EventResult {
        if self.dragging {
            self.dragging = false;
            self.base.dirty = true;
            return EventResult::Handled;
        }
        EventResult::Propagate
    }

    fn on_mouse_move(&mut self, _x: f32, y: f32) -> EventResult {
        if !self.dragging {
            return EventResult::Propagate;
        }

        let (_, track_y, _, track_h) = self.track_geometry();
        let thumb_ratio = (self.base.layout.h / self.content_height).min(1.0);
        let thumb_h = (track_h * thumb_ratio).max(SCROLLBAR_MIN_HEIGHT);
        let thumb_top = y - self.drag_offset;
        let scroll_ratio = (thumb_top - track_y) / (track_h - thumb_h).max(1.0);
        self.scroll_y = (scroll_ratio * self.max_scroll_y()).clamp(0.0, self.max_scroll_y());
        self.apply_transform();
        EventResult::Handled
    }

    fn on_mouse_scroll(&mut self, delta_x: f32, delta_y: f32) -> EventResult {
        let mut changed = false;
        if self.scroll_y_enabled {
            let new_y =
                (self.scroll_y + delta_y * self.scroll_speed).clamp(0.0, self.max_scroll_y());
            if new_y != self.scroll_y {
                self.scroll_y = new_y;
                changed = true;
            }
        }
        if self.scroll_x_enabled {
            let new_x =
                (self.scroll_x + delta_x * self.scroll_speed).clamp(0.0, self.max_scroll_x());
            if new_x != self.scroll_x {
                self.scroll_x = new_x;
                changed = true;
            }
        }
        if changed {
            self.apply_transform();
            EventResult::Handled
        } else {
            EventResult::Propagate
        }
    }
}

pub fn sync_scroll_containers(ui: &mut crate::ui::Ui) {
    let handles: Vec<crate::element::handle::Handle<()>> = ui
        .slots
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            let s = s.as_ref()?;
            s.element
                .as_ref()
                .as_any()
                .downcast_ref::<ScrollContainer>()?;
            Some(crate::element::handle::Handle::new(i as u32, s.generation))
        })
        .collect();

    for handle in handles {
        let children = ui.children(handle).to_vec();
        let mut max_w: f32 = 0.0;
        let mut max_h: f32 = 0.0;
        for child in &children {
            if let Some(el) = ui.get_any(*child) {
                let l = el.layout();
                max_w = max_w.max(l.w);
                max_h = max_h.max(l.h);
            }
        }

        if let Some(el) = ui.get_any_mut(handle) {
            if let Some(sc) = el.as_any_mut().downcast_mut::<ScrollContainer>() {
                if sc.content_width != max_w || sc.content_height != max_h {
                    sc.content_width = max_w;
                    sc.content_height = max_h;
                    sc.scroll_y = sc.scroll_y.min(sc.max_scroll_y());
                    sc.scroll_x = sc.scroll_x.min(sc.max_scroll_x());
                }
            }
        }
    }
}
