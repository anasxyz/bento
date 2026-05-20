use bento_shared::{RectNode, SceneNode, SceneNodeId};

use crate::{
    ui::Ui,
    widget::{Widget, WidgetHandle},
};

pub struct Rect {
    id: Option<SceneNodeId>,

    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub opacity: f32,
    pub z: i32,

    pub dirty: bool,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            id: None,
            x,
            y,
            w,
            h,
            color: [1.0, 1.0, 1.0, 1.0],
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            opacity: 1.0,
            z: 1,
            dirty: false,
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
        self.dirty = true;
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.dirty = true;
    }

    pub fn set_size(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
        self.dirty = true;
    }

    pub fn set_radius(&mut self, r: f32) {
        self.radii = [r; 4];
        self.dirty = true;
    }

    pub fn set_border(&mut self, color: [f32; 4], width: f32) {
        self.border_color = color;
        self.border_widths = [width; 4];
        self.dirty = true;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
        self.dirty = true;
    }

    pub fn set_z(&mut self, z: i32) {
        self.z = z;
        self.dirty = true;
    }
}

impl Widget for Rect {
    fn root(&self) -> Option<SceneNodeId> {
        self.id
    }

    fn name(&self) -> &str {
        "Rect"
    }

    fn build(&mut self, ui: &mut Ui, handle: WidgetHandle<()>) {
        let handle: WidgetHandle<Rect> = WidgetHandle::new(handle.id, handle.generation);

        let mut node = RectNode::new(self.x, self.y, self.w, self.h);
        node.color = self.color;
        node.radii = self.radii;
        node.border_color = self.border_color;
        node.border_widths = self.border_widths;
        node.opacity = self.opacity;
        node.z = self.z;
        self.id = Some(ui.scene_mut().add_rect(node));
    }

    fn update(&mut self, ui: &mut Ui) {
        if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.root().unwrap()) {
            r.color = self.color;
            r.x = self.x;
            r.y = self.y;
            r.w = self.w;
            r.h = self.h;
            r.radii = self.radii;
            r.border_color = self.border_color;
            r.border_widths = self.border_widths;
            r.opacity = self.opacity;
            r.z = self.z;
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
