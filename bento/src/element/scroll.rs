use crate::color::Color;
use crate::element::base::Base;
use crate::element::element::{Element, EventResult};
use crate::element::layout::Layout;
use crate::input::{Key, Modifiers, MouseButton};
use crate::render::DrawCall;
use bento_derive::Element;

const SCROLL_SPEED: f32 = 40.0;
const SCROLLBAR_WIDTH: f32 = 6.0;
const SCROLLBAR_MIN_SIZE: f32 = 24.0;
const LERP_SPEED: f32 = 8.0;
const ANIM_EPSILON: f32 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq)]
enum DragAxis {
    Vertical,
    Horizontal,
}

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
    pub smooth_scroll: bool,

    // smooth scroll targets
    // actual scroll_x/y lerps toward these
    target_scroll_x: f32,
    target_scroll_y: f32,

    pub scrollbar_visible: bool,
    pub scrollbar_width: f32,
    pub scrollbar_color: Color,
    pub scrollbar_track_color: Color,

    pub bg_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,
    pub border_widths: [f32; 4],

    dragging: Option<DragAxis>,
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
            smooth_scroll: false,
            target_scroll_x: 0.0,
            target_scroll_y: 0.0,
            scrollbar_visible: true,
            scrollbar_width: SCROLLBAR_WIDTH,
            scrollbar_color: Color::rgba(255, 255, 255, 80),
            scrollbar_track_color: Color::rgba(255, 255, 255, 20),
            bg_color: None,
            border_color: None,
            border_radius: None,
            border_widths: [0.0; 4],
            dragging: None,
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
    pub fn set_smooth_scroll(&mut self, v: bool) -> &mut Self {
        self.smooth_scroll = v;
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
        self.set_scroll_y(0.0);
    }
    pub fn scroll_to_bottom(&mut self) {
        self.set_scroll_y(self.max_scroll_y());
    }
    pub fn scroll_to_left(&mut self) {
        self.set_scroll_x(0.0);
    }
    pub fn scroll_to_right(&mut self) {
        self.set_scroll_x(self.max_scroll_x());
    }
    // programmatic scroll
    // always instant, syncs both target and actual
    pub fn set_scroll_y(&mut self, y: f32) {
        let y = y.clamp(0.0, self.max_scroll_y());
        self.scroll_y = y;
        self.target_scroll_y = y;
        self.apply_transform();
    }
    pub fn set_scroll_x(&mut self, x: f32) {
        let x = x.clamp(0.0, self.max_scroll_x());
        self.scroll_x = x;
        self.target_scroll_x = x;
        self.apply_transform();
    }
    pub fn set_content_size(&mut self, w: f32, h: f32) {
        self.content_width = w;
        self.content_height = h;
        self.base.dirty = true;
    }

    pub fn is_animating(&self) -> bool {
        self.smooth_scroll
            && ((self.scroll_y - self.target_scroll_y).abs() > ANIM_EPSILON
                || (self.scroll_x - self.target_scroll_x).abs() > ANIM_EPSILON)
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

    // instantly set position without affecting smooth target
    fn set_position_instant(&mut self, x: f32, y: f32) {
        self.scroll_x = x;
        self.scroll_y = y;
        self.target_scroll_x = x;
        self.target_scroll_y = y;
        self.apply_transform();
    }

    fn v_track(&self) -> (f32, f32, f32, f32) {
        let l = &self.base.layout;
        let x = l.x + l.w - self.scrollbar_width - 2.0;
        let y = l.y + 2.0;
        let h = l.h
            - if self.scroll_x_enabled && self.content_width > l.w {
                self.scrollbar_width + 4.0
            } else {
                4.0
            };
        (x, y, self.scrollbar_width, h)
    }

    fn v_thumb(&self) -> (f32, f32, f32, f32) {
        let (tx, ty, tw, th) = self.v_track();
        let ratio = (self.base.layout.h / self.content_height).min(1.0);
        let thumb_h = (th * ratio).max(SCROLLBAR_MIN_SIZE);
        let thumb_y = ty + (th - thumb_h) * (self.scroll_y / self.max_scroll_y().max(1.0));
        (tx, thumb_y, tw, thumb_h)
    }

    fn h_track(&self) -> (f32, f32, f32, f32) {
        let l = &self.base.layout;
        let x = l.x + 2.0;
        let y = l.y + l.h - self.scrollbar_width - 2.0;
        let w = l.w
            - if self.scroll_y_enabled && self.content_height > l.h {
                self.scrollbar_width + 4.0
            } else {
                4.0
            };
        (x, y, w, self.scrollbar_width)
    }

    fn h_thumb(&self) -> (f32, f32, f32, f32) {
        let (tx, ty, tw, th) = self.h_track();
        let ratio = (self.base.layout.w / self.content_width).min(1.0);
        let thumb_w = (tw * ratio).max(SCROLLBAR_MIN_SIZE);
        let thumb_x = tx + (tw - thumb_w) * (self.scroll_x / self.max_scroll_x().max(1.0));
        (thumb_x, ty, thumb_w, th)
    }

    fn draw_scrollbar(
        &self,
        calls: &mut Vec<DrawCall>,
        track: (f32, f32, f32, f32),
        thumb: (f32, f32, f32, f32),
        clip: Option<[f32; 4]>,
        z: i32,
        opacity: f32,
        active: bool,
    ) {
        let r = self.scrollbar_width / 2.0;
        let mut track_color = self.scrollbar_track_color.to_array();
        track_color[3] *= opacity;
        calls.push(DrawCall::Rect {
            x: track.0,
            y: track.1,
            w: track.2,
            h: track.3,
            color: track_color,
            radius: r,
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            clip,
            z_index: z + 1,
        });
        let thumb_base = if active {
            Color::rgba(255, 255, 255, 130)
        } else {
            self.scrollbar_color
        };
        let mut thumb_color = thumb_base.to_array();
        thumb_color[3] *= opacity;
        calls.push(DrawCall::Rect {
            x: thumb.0,
            y: thumb.1,
            w: thumb.2,
            h: thumb.3,
            color: thumb_color,
            radius: r,
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            clip,
            z_index: z + 2,
        });
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
            self.draw_scrollbar(
                &mut calls,
                self.v_track(),
                self.v_thumb(),
                clip,
                z,
                opacity,
                self.dragging == Some(DragAxis::Vertical),
            );
        }
        if self.scrollbar_visible && self.scroll_x_enabled && self.content_width > l.w {
            self.draw_scrollbar(
                &mut calls,
                self.h_track(),
                self.h_thumb(),
                clip,
                z,
                opacity,
                self.dragging == Some(DragAxis::Horizontal),
            );
        }

        calls
    }

    fn on_mouse_press(&mut self, x: f32, y: f32, button: MouseButton) -> EventResult {
        if button != MouseButton::Left {
            return EventResult::Propagate;
        }
        if !self.scrollbar_visible {
            return EventResult::Propagate;
        }
        let l = &self.base.layout;

        if self.scroll_y_enabled && self.content_height > l.h {
            let (thumb_x, thumb_y, thumb_w, thumb_h) = self.v_thumb();
            if x >= thumb_x && x <= thumb_x + thumb_w && y >= thumb_y && y <= thumb_y + thumb_h {
                self.dragging = Some(DragAxis::Vertical);
                self.drag_offset = y - thumb_y;
                self.base.dirty = true;
                return EventResult::Handled;
            }
            let (track_x, track_y, track_w, track_h) = self.v_track();
            if x >= track_x && x <= track_x + track_w && y >= track_y && y <= track_y + track_h {
                let ratio = (l.h / self.content_height).min(1.0);
                let th = (track_h * ratio).max(SCROLLBAR_MIN_SIZE);
                let target = y - track_y - th / 2.0;
                let new_y = (target / (track_h - th).max(1.0) * self.max_scroll_y())
                    .clamp(0.0, self.max_scroll_y());
                self.set_position_instant(self.scroll_x, new_y);
                return EventResult::Handled;
            }
        }

        if self.scroll_x_enabled && self.content_width > l.w {
            let (thumb_x, thumb_y, thumb_w, thumb_h) = self.h_thumb();
            if x >= thumb_x && x <= thumb_x + thumb_w && y >= thumb_y && y <= thumb_y + thumb_h {
                self.dragging = Some(DragAxis::Horizontal);
                self.drag_offset = x - thumb_x;
                self.base.dirty = true;
                return EventResult::Handled;
            }
            let (track_x, track_y, track_w, track_h) = self.h_track();
            if x >= track_x && x <= track_x + track_w && y >= track_y && y <= track_y + track_h {
                let ratio = (l.w / self.content_width).min(1.0);
                let tw = (track_w * ratio).max(SCROLLBAR_MIN_SIZE);
                let target = x - track_x - tw / 2.0;
                let new_x = (target / (track_w - tw).max(1.0) * self.max_scroll_x())
                    .clamp(0.0, self.max_scroll_x());
                self.set_position_instant(new_x, self.scroll_y);
                return EventResult::Handled;
            }
        }

        EventResult::Propagate
    }

    fn on_mouse_release(&mut self, _x: f32, _y: f32, _button: MouseButton) -> EventResult {
        if self.dragging.is_some() {
            self.dragging = None;
            self.base.dirty = true;
            return EventResult::Handled;
        }
        EventResult::Propagate
    }

    fn on_mouse_move(&mut self, x: f32, y: f32) -> EventResult {
        match self.dragging {
            Some(DragAxis::Vertical) => {
                let (_, track_y, _, track_h) = self.v_track();
                let ratio = (self.base.layout.h / self.content_height).min(1.0);
                let thumb_h = (track_h * ratio).max(SCROLLBAR_MIN_SIZE);
                let thumb_top = y - self.drag_offset;
                let scroll_ratio = (thumb_top - track_y) / (track_h - thumb_h).max(1.0);
                let new_y = (scroll_ratio * self.max_scroll_y()).clamp(0.0, self.max_scroll_y());
                self.set_position_instant(self.scroll_x, new_y);
                EventResult::Handled
            }
            Some(DragAxis::Horizontal) => {
                let (track_x, _, track_w, _) = self.h_track();
                let ratio = (self.base.layout.w / self.content_width).min(1.0);
                let thumb_w = (track_w * ratio).max(SCROLLBAR_MIN_SIZE);
                let thumb_left = x - self.drag_offset;
                let scroll_ratio = (thumb_left - track_x) / (track_w - thumb_w).max(1.0);
                let new_x = (scroll_ratio * self.max_scroll_x()).clamp(0.0, self.max_scroll_x());
                self.set_position_instant(new_x, self.scroll_y);
                EventResult::Handled
            }
            None => EventResult::Propagate,
        }
    }

    fn on_mouse_scroll(&mut self, delta_x: f32, delta_y: f32) -> EventResult {
        let mut changed = false;
        if self.scroll_y_enabled {
            let new_y = (self.target_scroll_y + delta_y * self.scroll_speed)
                .clamp(0.0, self.max_scroll_y());
            if new_y != self.target_scroll_y {
                self.target_scroll_y = new_y;
                changed = true;
            }
        }
        if self.scroll_x_enabled {
            let new_x = (self.target_scroll_x + delta_x * self.scroll_speed)
                .clamp(0.0, self.max_scroll_x());
            if new_x != self.target_scroll_x {
                self.target_scroll_x = new_x;
                changed = true;
            }
        }
        if changed {
            if !self.smooth_scroll {
                // instant
                // sync actual to target immediately
                self.scroll_y = self.target_scroll_y;
                self.scroll_x = self.target_scroll_x;
            }
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
                    sc.target_scroll_y = sc.target_scroll_y.min(sc.max_scroll_y());
                    sc.target_scroll_x = sc.target_scroll_x.min(sc.max_scroll_x());
                }
            }
        }
    }
}

pub fn tick_scroll_containers(ui: &mut crate::ui::Ui, dt: f32) {
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
        if let Some(el) = ui.get_any_mut(handle) {
            if let Some(sc) = el.as_any_mut().downcast_mut::<ScrollContainer>() {
                if !sc.smooth_scroll {
                    continue;
                }

                let factor = (LERP_SPEED * dt).min(1.0);
                let mut changed = false;

                let diff_y = sc.target_scroll_y - sc.scroll_y;
                if diff_y.abs() > ANIM_EPSILON {
                    sc.scroll_y += diff_y * factor;
                    changed = true;
                } else if diff_y.abs() > 0.0 {
                    sc.scroll_y = sc.target_scroll_y;
                    changed = true;
                }

                let diff_x = sc.target_scroll_x - sc.scroll_x;
                if diff_x.abs() > ANIM_EPSILON {
                    sc.scroll_x += diff_x * factor;
                    changed = true;
                } else if diff_x.abs() > 0.0 {
                    sc.scroll_x = sc.target_scroll_x;
                    changed = true;
                }

                if changed {
                    sc.apply_transform();
                }
            }
        }
    }
}

pub fn is_scroll_animating(ui: &crate::ui::Ui) -> bool {
    ui.slots.iter().filter_map(|s| s.as_ref()).any(|s| {
        s.element
            .as_ref()
            .as_any()
            .downcast_ref::<ScrollContainer>()
            .map(|sc| sc.is_animating())
            .unwrap_or(false)
    })
}
