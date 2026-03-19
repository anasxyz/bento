use crate::color::Color;
use crate::element::base::Base;
use crate::element::element::{Element, EventResult};
use crate::element::handle::Handle;
use crate::element::layout::Layout;
use crate::element::scrollbar::Scrollbar;
use crate::input::{Key, Modifiers, MouseButton};
use crate::render::DrawCall;
use crate::ui::Ui;
use bento_derive::Element;

const LERP_SPEED: f32 = 16.0;
const ANIM_EPSILON: f32 = 0.05;

#[derive(Element)]
pub struct ScrollContainer {
    base: Base,

    pub bar: Scrollbar,
    pub smooth_scroll: bool,

    // smooth scroll targets
    target_scroll_x: f32,
    target_scroll_y: f32,

    pub bg_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,
    pub border_widths: [f32; 4],
}

impl ScrollContainer {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            bar: Scrollbar::new(),
            smooth_scroll: false,
            target_scroll_x: 0.0,
            target_scroll_y: 0.0,
            bg_color: None,
            border_color: None,
            border_radius: None,
            border_widths: [0.0; 4],
        }
    }

    // ── convenience passthrough setters ──────────────────────────────────────
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
        self.bar.speed = speed;
        self
    }
    pub fn set_smooth_scroll(&mut self, v: bool) -> &mut Self {
        self.smooth_scroll = v;
        self
    }
    pub fn set_scrollbar_visible(&mut self, v: bool) -> &mut Self {
        self.bar.visible = v;
        self.base.dirty = true;
        self
    }
    pub fn set_scroll_x_enabled(&mut self, v: bool) -> &mut Self {
        self.bar.horizontal = v;
        self
    }
    pub fn set_scroll_y_enabled(&mut self, v: bool) -> &mut Self {
        self.bar.vertical = v;
        self
    }
    pub fn set_scroll_y(&mut self, y: f32) {
        self.bar.set_scroll_y(y);
        self.target_scroll_y = self.bar.scroll_y;
        self.apply_transform();
    }
    pub fn set_scroll_x(&mut self, x: f32) {
        self.bar.set_scroll_x(x);
        self.target_scroll_x = self.bar.scroll_x;
        self.apply_transform();
    }
    pub fn scroll_to_top(&mut self) {
        self.set_scroll_y(0.0);
    }
    pub fn scroll_to_bottom(&mut self) {
        self.set_scroll_y(self.bar.max_scroll_y());
    }

    // convenience reads
    pub fn scroll_x(&self) -> f32 {
        self.bar.scroll_x
    }
    pub fn scroll_y(&self) -> f32 {
        self.bar.scroll_y
    }

    pub fn is_animating(&self) -> bool {
        self.smooth_scroll
            && ((self.bar.scroll_y - self.target_scroll_y).abs() > ANIM_EPSILON
                || (self.bar.scroll_x - self.target_scroll_x).abs() > ANIM_EPSILON)
    }

    fn rect(&self) -> (f32, f32, f32, f32) {
        let l = &self.base.layout;
        (l.x, l.y, l.w, l.h)
    }

    fn apply_transform(&mut self) {
        self.base.layout.transform = Some((-self.bar.scroll_x, -self.bar.scroll_y));
        self.base.dirty = true;
    }
}

impl Element for ScrollContainer {
    fn draw_calls(&self, clip: Option<[f32; 4]>, z: i32, opacity: f32) -> Vec<DrawCall> {
        let l = &self.base.layout;
        let mut calls = Vec::new();

        if let Some(bg) = self.bg_color {
            let mut color = bg.to_array();
            color[3] *= opacity;
            let mut bc = self.border_color.unwrap_or(Color::BLACK).to_array();
            bc[3] *= opacity;
            calls.push(DrawCall::Rect {
                x: l.x,
                y: l.y,
                w: l.w,
                h: l.h,
                color,
                radius: self.border_radius.unwrap_or(0.0),
                border_color: bc,
                border_widths: self.border_widths,
                clip,
                z_index: z,
            });
        }

        calls.extend(self.bar.draw_calls(self.rect(), clip, z + 1, opacity));
        calls
    }

    fn on_mouse_press(
        &mut self,
        _ui: &mut Ui,
        _handle: Handle<()>,
        x: f32,
        y: f32,
        button: MouseButton,
    ) -> EventResult {
        if button != MouseButton::Left {
            return EventResult::Propagate;
        }
        if self.bar.on_mouse_press(self.rect(), x, y) {
            self.base.dirty = true;
            EventResult::Handled
        } else {
            EventResult::Propagate
        }
    }

    fn on_mouse_release(
        &mut self,
        _ui: &mut Ui,
        _handle: Handle<()>,
        _x: f32,
        _y: f32,
        _button: MouseButton,
    ) -> EventResult {
        if self.bar.on_mouse_release() {
            self.base.dirty = true;
            EventResult::Handled
        } else {
            EventResult::Propagate
        }
    }

    fn on_mouse_move(&mut self, _ui: &mut Ui, _handle: Handle<()>, x: f32, y: f32) -> EventResult {
        if self.bar.on_mouse_move(self.rect(), x, y) {
            self.apply_transform();
            EventResult::Handled
        } else {
            EventResult::Propagate
        }
    }

    fn on_mouse_scroll(
        &mut self,
        ui: &mut Ui,
        handle: Handle<()>,
        delta_x: f32,
        delta_y: f32,
    ) -> EventResult {
        let changed = if self.smooth_scroll {
            let mut changed = false;
            if self.bar.vertical {
                let new_y = (self.target_scroll_y + delta_y * self.bar.speed)
                    .clamp(0.0, self.bar.max_scroll_y());
                if new_y != self.target_scroll_y {
                    self.target_scroll_y = new_y;
                    changed = true;
                }
            }
            if self.bar.horizontal {
                let new_x = (self.target_scroll_x + delta_x * self.bar.speed)
                    .clamp(0.0, self.bar.max_scroll_x());
                if new_x != self.target_scroll_x {
                    self.target_scroll_x = new_x;
                    changed = true;
                }
            }
            if changed {
                self.base.dirty = true;
            }
            changed
        } else {
            let changed = self.bar.on_scroll(delta_x, delta_y);
            if changed {
                self.apply_transform();
            }
            changed
        };

        if changed {
            ui.emit(
                handle,
                crate::event::Event::Scroll {
                    x: self.bar.scroll_x,
                    y: self.bar.scroll_y,
                },
            );
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
                let vw = sc.base.layout.w;
                let vh = sc.base.layout.h;
                sc.bar.viewport_width = vw;
                sc.bar.viewport_height = vh;
                if sc.bar.content_width != max_w || sc.bar.content_height != max_h {
                    sc.bar.content_width = max_w;
                    sc.bar.content_height = max_h;
                    sc.bar.scroll_y = sc.bar.scroll_y.min(sc.bar.max_scroll_y());
                    sc.bar.scroll_x = sc.bar.scroll_x.min(sc.bar.max_scroll_x());
                    sc.target_scroll_y = sc.target_scroll_y.min(sc.bar.max_scroll_y());
                    sc.target_scroll_x = sc.target_scroll_x.min(sc.bar.max_scroll_x());
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

                let diff_y = sc.target_scroll_y - sc.bar.scroll_y;
                if diff_y.abs() > ANIM_EPSILON {
                    sc.bar.scroll_y += diff_y * factor;
                    changed = true;
                } else if diff_y.abs() > 0.0 {
                    sc.bar.scroll_y = sc.target_scroll_y;
                    changed = true;
                }

                let diff_x = sc.target_scroll_x - sc.bar.scroll_x;
                if diff_x.abs() > ANIM_EPSILON {
                    sc.bar.scroll_x += diff_x * factor;
                    changed = true;
                } else if diff_x.abs() > 0.0 {
                    sc.bar.scroll_x = sc.target_scroll_x;
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
