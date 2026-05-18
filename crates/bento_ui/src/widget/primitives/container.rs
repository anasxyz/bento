use crate::{AsAny, Ui, Widget, WidgetHandle};
use bento_shared::TextMeasurer;
use bento_shared::{RectNode, SceneNode, SceneNodeId};
use std::any::Any;

pub struct Container {
    handle: WidgetHandle<Container>,
    pub dirty: bool,

    x: f32,
    y: f32,
    w: f32,
    h: f32,

    color: Option<[f32; 4]>,

    clip: bool,
    offset_x: f32,
    offset_y: f32,

    group_id: Option<SceneNodeId>,
    rect_id: Option<SceneNodeId>,
}

impl Container {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            handle: WidgetHandle::default(),
            dirty: true,
            x,
            y,
            w,
            h,
            color: None,
            clip: false,
            offset_x: 0.0,
            offset_y: 0.0,
            group_id: None,
            rect_id: None,
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = Some(color);
        self.dirty = true;
    }

    pub fn clear_color(&mut self) {
        self.color = None;
        self.dirty = true;
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn w(&self) -> f32 {
        self.w
    }
    pub fn h(&self) -> f32 {
        self.h
    }

    pub fn offset_x(&self) -> f32 {
        self.offset_x
    }
    pub fn offset_y(&self) -> f32 {
        self.offset_y
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
        self.dirty = true;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
        self.dirty = true;
    }
    pub fn set_w(&mut self, w: f32) {
        self.w = w;
        self.dirty = true;
    }
    pub fn set_h(&mut self, h: f32) {
        self.h = h;
        self.dirty = true;
    }

    pub fn set_clip(&mut self, clip: bool) {
        self.clip = clip;
        self.dirty = true;
    }
    pub fn set_offset(&mut self, x: f32, y: f32) {
        self.offset_x = x;
        self.offset_y = y;
        self.dirty = true;
    }
}

impl Widget for Container {
    fn name(&self) -> &str {
        "Container"
    }

    fn set_handle(&mut self, id: u32, generation: u32) {
        self.handle = WidgetHandle::new(id, generation);
    }

    fn build(&mut self, ui: &mut Ui) {
        let scene = ui.scene_mut();
        self.group_id = Some(scene.add_group(|g, s| {
            g.x = self.x;
            g.y = self.y;
            g.offset_x = self.offset_x;
            g.offset_y = self.offset_y;
            g.clip = if self.clip {
                Some([self.x, self.y, self.w, self.h])
            } else {
                None
            };
            if let Some(color) = self.color {
                self.rect_id = Some(s.add_rect({
                    let mut r = RectNode::new(self.x, self.y, self.w, self.h);
                    r.color = color;
                    r
                }));
            }
        }));
    }

    fn update(&mut self, ui: &mut Ui, _measurer: &mut dyn TextMeasurer) {
        if let Some(id) = self.rect_id {
            if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(id) {
                r.x = self.x;
                r.y = self.y;
                r.w = self.w;
                r.h = self.h;
                r.color = self.color.unwrap_or([0.0; 4]);
            }
        }

        if let Some(id) = self.group_id {
            if let Some(SceneNode::Group(g)) = ui.scene_mut().get_mut(id) {
                g.offset_x = self.offset_x;
                g.offset_y = self.offset_y;
                g.clip = if self.clip {
                    Some([self.x, self.y, self.w, self.h])
                } else {
                    None
                };
            }
        }
    }

    fn remove(&mut self, ui: &mut Ui) {
        if let Some(id) = self.group_id {
            ui.scene_mut().remove(id);
        }
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn hoverable(&self) -> bool {
        false
    }
    fn focusable(&self) -> bool {
        false
    }

    fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.w, self.h)
    }

    fn scene_root(&self) -> Option<SceneNodeId> {
        self.group_id
    }
}

impl AsAny for Container {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
