use crate::{
    Ui,
    widget::{Widget, WidgetHandle},
};
use bento_shared::{GroupNode, RectNode, SceneNode, SceneNodeId};

pub struct Container {
    id: Option<SceneNodeId>,
    bg: Option<SceneNodeId>,
    dirty: bool,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub clip: bool,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Container {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            id: None,
            bg: None,
            dirty: false,
            x,
            y,
            w,
            h,
            color: [0.2, 0.2, 0.2, 1.0],
            clip: false,
            offset_x: 0.0,
            offset_y: 0.0,
        }
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

    pub fn id(&self) -> Option<SceneNodeId> {
        self.id
    }

    pub fn append(&self, ui: &mut Ui, child: SceneNodeId) {
        ui.scene_mut().append(self.id.unwrap(), child);
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
        self.dirty = true;
    }
    pub fn set_offset(&mut self, x: f32, y: f32) {
        self.offset_x = x;
        self.offset_y = y;
        self.dirty = true;
    }
    pub fn set_clip(&mut self, clip: bool) {
        self.clip = clip;
        self.dirty = true;
    }
}

impl Widget for Container {
    fn id(&self) -> Option<SceneNodeId> {
        self.id
    }

    fn build(&mut self, ui: &mut Ui, _handle: WidgetHandle<()>) {
        let mut bg = RectNode::new(self.x, self.y, self.w, self.h);
        bg.color = self.color;
        let bg_id = ui.scene_mut().add_rect(bg);

        let mut g = GroupNode::new();
        g.x = self.x;
        g.y = self.y;
        g.w = self.w;
        g.h = self.h;
        g.offset_x = self.offset_x;
        g.offset_y = self.offset_y;
        if self.clip {
            g.clip = Some([self.x, self.y, self.w, self.h]);
        }
        let root = ui.scene_mut().add_group(g);

        self.id = Some(root);
        self.bg = Some(bg_id);
    }

    fn update(&mut self, ui: &mut Ui) {
        if let Some(SceneNode::Group(g)) = ui.scene_mut().get_mut(self.id.unwrap()) {
            g.offset_x = self.offset_x;
            g.offset_y = self.offset_y;
            g.clip = if self.clip {
                Some([self.x, self.y, self.w, self.h])
            } else {
                None
            };
            ui.needs_redraw = true;
        }
        if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.bg.unwrap()) {
            r.color = self.color;
            ui.needs_redraw = true;
        }
        self.dirty = false;
    }

    fn remove(&mut self, ui: &mut Ui) {
        if let Some(id) = self.id {
            ui.scene_mut().remove(id);
        }
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}
