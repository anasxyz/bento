use super::scroll_state::ScrollState;
use crate::color::Color;
use crate::ui::Ui;
use crate::widget::{AsAny, Base, Handle, HasBase, Widget};
use bento_derive::Widget;
use bento_wgpu::{ClipId, RectId, SceneGraph, SceneNodeId, TransformId};

#[derive(Widget)]
pub struct ScrollContainer {
    pub base: Base,
    pub scroll: ScrollState,
    color: Color,

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
            scroll: ScrollState::new(),
            color: Color::TRANSPARENT,
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
    pub fn set_scroll_x_enabled(&mut self, v: bool) -> &mut Self {
        self.scroll.set_scroll_x_enabled(v);
        self.base.render_dirty = true;
        self
    }
    pub fn set_scroll_y_enabled(&mut self, v: bool) -> &mut Self {
        self.scroll.set_scroll_y_enabled(v);
        self.base.render_dirty = true;
        self
    }
    pub fn set_scrollbar_width(&mut self, v: f32) -> &mut Self {
        self.scroll.set_scrollbar_width(v);
        self.base.render_dirty = true;
        self
    }
    pub fn set_scroll_speed(&mut self, v: f32) -> &mut Self {
        self.scroll.set_scroll_speed(v);
        self.base.render_dirty = true;
        self
    }
    pub fn set_track_color(&mut self, c: Color) -> &mut Self {
        self.scroll.set_track_color(c);
        self.base.render_dirty = true;
        self
    }
    pub fn set_thumb_color(&mut self, c: Color) -> &mut Self {
        self.scroll.set_thumb_color(c);
        self.base.render_dirty = true;
        self
    }
    pub fn set_thumb_hover_color(&mut self, c: Color) -> &mut Self {
        self.scroll.set_thumb_hover_color(c);
        self.base.render_dirty = true;
        self
    }
    pub fn set_thumb_radius(&mut self, v: f32) -> &mut Self {
        self.scroll.set_thumb_radius(v);
        self.base.render_dirty = true;
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

    fn register(&mut self, handle: Handle<()>, ui: &mut Ui) {
        let h = Handle::<ScrollContainer>::new(handle.id, handle.generation);

        ui.on_scroll(h, |ui, this, e| {
            this.scroll
                .on_scroll(e.x, e.y, this.base.content_width, this.base.content_height);
            this.base.render_dirty = true;
        });

        ui.on_press(h, |ui, this, e| {
            this.scroll.on_press(
                e.x,
                e.y,
                crate::input::MouseButton::Left,
                this.base.content_width,
                this.base.content_height,
            );
            this.base.render_dirty = true;
        });

        ui.on_release(h, |ui, this, e| {
            this.scroll.on_release();
            this.base.render_dirty = true;
        });

        ui.on_hover_end(h, |ui, this, e| {
            this.scroll.on_leave();
            this.base.render_dirty = true;
        });

        ui.on_mouse_move(h, |ui, this, e| {
            let hover_changed =
                this.scroll
                    .on_move(e.x, e.y, this.base.content_width, this.base.content_height);
            if hover_changed || this.scroll.is_dragging() {
                this.base.render_dirty = true;
            }
        });
    }

    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, h: f32) {
        if let Some(id) = self.rect_id {
            let n = scene.rect_mut(id);
            n.set_rect(x, y, w, h);
            n.set_color(self.color.to_array());
            n.set_visible(true);
        }
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
        );
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn children_attachment_node(&self) -> Option<SceneNodeId> {
        self.transform_id.map(|id| SceneNodeId(id.0))
    }
}
