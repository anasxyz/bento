use crate::color::Color;
use crate::input::MouseButton;
use crate::layout::Overflow;
use crate::ui::{HoverEnd, MouseMove, Press, Release, Scroll, Ui};
use crate::widget::{AsAny, Base, Handle, HasBase, Widget};
use crate::widgets::scroll_state::ScrollState;
use bento_derive::Widget;
use bento_wgpu::{ClipId, RectId, SceneGraph, SceneNodeId, TransformId};

#[derive(Widget)]
pub struct Container {
    pub base: Base,

    // visual
    color: Color,
    radius: f32,
    border_color: Color,
    border_widths: [f32; 4],

    // scroll is only used when overflow is set tp Scroll
    pub scroll: ScrollState,

    // scene nodes
    rect_id: Option<RectId>,
    clip_id: Option<ClipId>,
    transform_id: Option<TransformId>,
    v_track_id: Option<RectId>,
    v_thumb_id: Option<RectId>,
    h_track_id: Option<RectId>,
    h_thumb_id: Option<RectId>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            color: Color::TRANSPARENT,
            radius: 0.0,
            border_color: Color::TRANSPARENT,
            border_widths: [0.0; 4],
            scroll: ScrollState::new(),
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
        self.base.render_dirty = true;
        self
    }
    pub fn set_radius(&mut self, r: f32) -> &mut Self {
        self.radius = r;
        self.base.render_dirty = true;
        self
    }
    pub fn set_border_color(&mut self, c: Color) -> &mut Self {
        self.border_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_border_widths(&mut self, w: [f32; 4]) -> &mut Self {
        self.border_widths = w;
        self.base.render_dirty = true;
        self
    }
    pub fn set_scroll_speed(&mut self, v: f32) -> &mut Self {
        self.scroll.set_scroll_speed(v);
        self
    }
    pub fn set_scrollbar_width(&mut self, v: f32) -> &mut Self {
        self.scroll.set_scrollbar_width(v);
        self
    }
    pub fn set_track_color(&mut self, c: Color) -> &mut Self {
        self.scroll.set_track_color(c);
        self
    }
    pub fn set_thumb_color(&mut self, c: Color) -> &mut Self {
        self.scroll.set_thumb_color(c);
        self
    }
    pub fn set_thumb_hover_color(&mut self, c: Color) -> &mut Self {
        self.scroll.set_thumb_hover_color(c);
        self
    }
    pub fn set_thumb_radius(&mut self, v: f32) -> &mut Self {
        self.scroll.set_thumb_radius(v);
        self
    }

    fn is_scroll(&self) -> bool {
        self.base.layout.overflow == Overflow::Scroll
    }

    fn hide_scrollbars(&self, scene: &mut SceneGraph) {
        if let Some(id) = self.v_track_id {
            scene.rect_mut(id).set_visible(false);
        }
        if let Some(id) = self.v_thumb_id {
            scene.rect_mut(id).set_visible(false);
        }
        if let Some(id) = self.h_track_id {
            scene.rect_mut(id).set_visible(false);
        }
        if let Some(id) = self.h_thumb_id {
            scene.rect_mut(id).set_visible(false);
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Container {
    fn build(&mut self, scene: &mut SceneGraph) {
        self.rect_id = Some(scene.add_rect());
        self.clip_id = Some(scene.add_clip());
        self.transform_id = Some(scene.add_transform());
        self.v_track_id = Some(scene.add_rect());
        self.v_thumb_id = Some(scene.add_rect());
        self.h_track_id = Some(scene.add_rect());
        self.h_thumb_id = Some(scene.add_rect());

        // only transform is inside clip
        // scrollbars and bg rect are top level
        let clip = self.clip_id.unwrap();
        let transform = self.transform_id.unwrap();
        scene.add_child(SceneNodeId(clip.0), SceneNodeId(transform.0));
    }

    fn register(&mut self, handle: Handle<()>, ui: &mut Ui) {
        let h = Handle::<Container>::new(handle.id, handle.generation);

        ui.on::<Container, Scroll>(h, |ui, this, e| {
            if !this.is_scroll() {
                return;
            }
            this.scroll
                .on_scroll(e.x, e.y, this.base.content_width, this.base.content_height);
            this.base.render_dirty = true;
        });

        ui.on::<Container, Press>(h, |ui, this, e| {
            if !this.is_scroll() {
                return;
            }
            this.scroll.on_press(
                e.x,
                e.y,
                MouseButton::Left,
                this.base.content_width,
                this.base.content_height,
            );
            this.base.render_dirty = true;
        });

        ui.on::<Container, Release>(h, |ui, this, e| {
            if !this.is_scroll() {
                return;
            }
            this.scroll.on_release();
            this.base.render_dirty = true;
        });

        ui.on::<Container, HoverEnd>(h, |ui, this, e| {
            if !this.is_scroll() {
                return;
            }
            this.scroll.on_leave();
            this.base.render_dirty = true;
        });

        ui.on::<Container, MouseMove>(h, |ui, this, e| {
            if !this.is_scroll() {
                return;
            }
            let hover_changed =
                this.scroll
                    .on_move(e.x, e.y, this.base.content_width, this.base.content_height);
            if hover_changed || this.scroll.is_dragging() {
                this.base.render_dirty = true;
            }
        });
    }

    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, h: f32, layer: u32) {
        if let Some(id) = self.rect_id {
            let n = scene.rect_mut(id);
            n.set_rect(x, y, w, h);
            n.set_color(self.color.to_array());
            n.set_radius(self.radius);
            n.set_border_color(self.border_color.to_array());
            n.set_border_widths(self.border_widths);
            n.set_z(layer as i32);
            n.set_visible(true);
        }

        match self.base.layout.overflow {
            Overflow::Visible => {
                if let Some(id) = self.clip_id {
                    scene
                        .clip_mut(id)
                        .set_rect(-100000.0, -100000.0, 200000.0, 200000.0);
                }
                if let Some(id) = self.transform_id {
                    scene.transform_mut(id).set_offset(0.0, 0.0);
                }
                self.hide_scrollbars(scene);
            }
            Overflow::Hidden => {
                if let Some(id) = self.clip_id {
                    scene.clip_mut(id).set_rect(x, y, w, h);
                }
                if let Some(id) = self.transform_id {
                    scene.transform_mut(id).set_offset(0.0, 0.0);
                }
                self.hide_scrollbars(scene);
            }
            Overflow::Scroll => {
                self.scroll.sync(
                    scene,
                    x,
                    y,
                    w,
                    h,
                    self.base.content_width,
                    self.base.content_height,
                    self.clip_id.unwrap(),
                    self.transform_id.unwrap(),
                    self.v_track_id.unwrap(),
                    self.v_thumb_id.unwrap(),
                    self.h_track_id.unwrap(),
                    self.h_thumb_id.unwrap(),
                    layer,
                );
            }
        }
    }

    fn children_attachment_node(&self) -> Option<SceneNodeId> {
        self.transform_id.map(|id| SceneNodeId(id.0))
    }

    fn is_interactive(&self) -> bool {
        self.is_scroll()
    }
}
