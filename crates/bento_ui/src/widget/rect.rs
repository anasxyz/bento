use bento_shared::{RectNode, SceneNode, SceneNodeId};

use crate::{ui::Ui, widget::Widget};

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
        }
    }

    pub fn id(&self) -> SceneNodeId {
        self.id
            .expect("Rect not added to Ui yet — call ui.add() first")
    }

    pub fn set_color(&mut self, ui: &mut Ui, color: [f32; 4]) {
        self.color = color;
        if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.id()) {
            r.color = color;
            ui.needs_redraw = true;
        }
    }

    pub fn set_pos(&mut self, ui: &mut Ui, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.id()) {
            r.x = x;
            r.y = y;
            ui.needs_redraw = true;
        }
    }

    pub fn set_size(&mut self, ui: &mut Ui, w: f32, h: f32) {
        self.w = w;
        self.h = h;
        if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.id()) {
            r.w = w;
            r.h = h;
            ui.needs_redraw = true;
        }
    }

    pub fn set_radius(&mut self, ui: &mut Ui, r: f32) {
        self.radii = [r; 4];
        if let Some(SceneNode::Rect(n)) = ui.scene_mut().get_mut(self.id()) {
            n.radii = [r; 4];
            ui.needs_redraw = true;
        }
    }

    pub fn set_border(&mut self, ui: &mut Ui, color: [f32; 4], width: f32) {
        self.border_color = color;
        self.border_widths = [width; 4];
        if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.id()) {
            r.border_color = color;
            r.border_widths = [width; 4];
            ui.needs_redraw = true;
        }
    }

    pub fn set_opacity(&mut self, ui: &mut Ui, opacity: f32) {
        self.opacity = opacity;
        if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.id()) {
            r.opacity = opacity;
            ui.needs_redraw = true;
        }
    }

    pub fn set_z(&mut self, ui: &mut Ui, z: i32) {
        self.z = z;
        if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.id()) {
            r.z = z;
            ui.needs_redraw = true;
        }
    }
}

impl Widget for Rect {
    fn build(&mut self, ui: &mut Ui) {
        let mut node = RectNode::new(self.x, self.y, self.w, self.h);
        node.color = self.color;
        node.radii = self.radii;
        node.border_color = self.border_color;
        node.border_widths = self.border_widths;
        node.opacity = self.opacity;
        node.z = self.z;
        self.id = Some(ui.scene_mut().add_rect(node));
    }

    fn remove(&mut self, ui: &mut Ui) {
        if let Some(id) = self.id {
            ui.scene_mut().remove(id);
        }
    }
}
