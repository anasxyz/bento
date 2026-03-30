use crate::color::Color;
use crate::fonts::Fonts;
use crate::layout::Overflow;
use crate::widget::{AsAny, Base, HasBase, Widget};
use bento_derive::Widget;
use bento_wgpu::{ClipId, RectId, SceneGraph, SceneNodeId, TransformId};

#[derive(Widget)]
pub struct Rect {
    pub base: Base,
    color: Color,
    radius: f32,
    border_color: Color,
    border_widths: [f32; 4],
    rect_id: Option<RectId>,
    transform_id: Option<TransformId>,
    clip_id: Option<ClipId>,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            color: Color::TRANSPARENT,
            radius: 0.0,
            border_color: Color::TRANSPARENT,
            border_widths: [0.0; 4],
            rect_id: None,
            transform_id: None,
            clip_id: None,
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
}

impl Default for Rect {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Rect {
    fn build(&mut self, scene: &mut SceneGraph) {
        let transform = scene.add_transform();
        let rect = scene.add_rect();
        scene.add_child(SceneNodeId(transform.0), SceneNodeId(rect.0));
        self.transform_id = Some(transform);
        self.rect_id = Some(rect);

        // always create a clip node
        // only activated when overflow is Hidden
        let clip = scene.add_clip();
        scene.add_child(SceneNodeId(transform.0), SceneNodeId(clip.0));
        self.clip_id = Some(clip);
    }

    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, h: f32) {
        if let Some(id) = self.rect_id {
            let node = scene.rect_mut(id);
            node.set_rect(x, y, w, h);
            node.set_color(self.color.to_array());
            node.set_radius(self.radius);
            node.set_border_color(self.border_color.to_array());
            node.set_border_widths(self.border_widths);
            node.set_visible(true);
        }

        // update clip region based on overflow setting
        if let Some(id) = self.clip_id {
            match self.base.layout.overflow {
                Overflow::Hidden => {
                    scene.clip_mut(id).set_rect(x, y, w, h);
                }
                _ => {
                    // disable clipping by setting an enormous rect
                    scene
                        .clip_mut(id)
                        .set_rect(-100000.0, -100000.0, 200000.0, 200000.0);
                }
            }
        }
    }

    fn children_attachment_node(&self) -> Option<SceneNodeId> {
        // children attach to the clip node so they get clipped when overflow is Hidden
        self.clip_id.map(|id| SceneNodeId(id.0))
    }
}
