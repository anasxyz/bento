use std::{cell::Cell, rc::Rc};

use bento_shared::{GroupNode, RectNode, SceneNode, SceneNodeId};

use crate::{
    Change, Click, MouseDown, MouseMove, MouseUp, Ui,
    widget::{Widget, WidgetHandle},
};

pub struct Slider {
    id: Option<SceneNodeId>,
    thumb: Option<SceneNodeId>,
    track: Option<SceneNodeId>,
    dirty: bool,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub value: f32, // 0.0 - 1.0
    pub min: f32,
    pub max: f32,
    changed: bool,
}

impl Slider {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            id: None,
            thumb: None,
            track: None,
            dirty: false,
            x,
            y,
            w,
            h,
            value: 0.0,
            min: 0.0,
            max: 1.0,
            changed: false,
        }
    }

    pub fn thumb_id(&self) -> SceneNodeId {
        self.thumb.expect("Slider not added yet")
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
        self.dirty = true;
        self.changed = true;
    }
}

impl Widget for Slider {
    fn root(&self) -> Option<SceneNodeId> {
        self.id
    }

    fn name(&self) -> &str {
        "Slider"
    }

    fn build(&mut self, ui: &mut Ui, handle: WidgetHandle<()>) {
        let thumb_w = 20.0;
        let thumb_h = self.h;

        let mut g = GroupNode::new();
        g.offset_x = self.x;
        g.offset_y = self.y;
        let root = ui.scene_mut().add_group(g);

        let mut track = RectNode::new(0.0, self.h / 2.0 - 2.0, self.w, 5.0);
        track.color([0.3, 0.3, 0.3, 1.0]);
        track.radius(3.0);
        let track_id = ui.scene_mut().add_rect(track);

        let thumb_x = (self.value - self.min) / (self.max - self.min) * (self.w - thumb_w);
        let mut thumb = RectNode::new(thumb_x, 0.0, thumb_w, thumb_h);
        thumb.color([1.0, 1.0, 1.0, 1.0]);
        thumb.radius(10.0);
        let thumb_id = ui.scene_mut().add_rect(thumb);

        ui.scene_mut().append(root, track_id);
        ui.scene_mut().append(root, thumb_id);

        self.id = Some(root);
        self.track = Some(track_id);
        self.thumb = Some(thumb_id);

        let slider_handle: WidgetHandle<Slider> = WidgetHandle::new(handle.id, handle.generation);
        let thumb_id = self.thumb.unwrap();
        let dragging = Rc::new(Cell::new(false));

        let dragging_clone = dragging.clone();
        ui.listen(thumb_id, move |e: &MouseDown, ui| {
            dragging_clone.set(true);
        });

        let dragging_clone = dragging.clone();
        ui.listen_global(move |e: &MouseMove, ui| {
            if dragging_clone.get() && ui.input.mouse.left.pressed {
                let mx = ui.input.mouse.x;
                let s = ui.get_mut(slider_handle).unwrap();
                let t = (mx - s.x) / s.w;
                s.set_value(t.clamp(0.0, 1.0));
                let val = s.value;
                let root = s.root().unwrap();
                ui.send(root, Change { value: val });
            }
        });

        let dragging_clone = dragging.clone();
        ui.listen_global(move |e: &MouseUp, ui| {
            dragging_clone.set(false);
        });

        let track_id = self.track.unwrap();

        let dragging_clone = dragging.clone();
        ui.listen(track_id, move |e: &MouseDown, ui| {
            dragging_clone.set(true);
            let s = ui.get_mut(slider_handle).unwrap();
            let t = (e.x - s.x) / s.w;
            s.set_value(t.clamp(0.0, 1.0));
        });
    }

    fn update(&mut self, ui: &mut Ui) {
        let thumb_w = 20.0;
        let thumb_x = (self.value - self.min) / (self.max - self.min) * (self.w - thumb_w);
        if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.thumb.unwrap()) {
            r.x = thumb_x;
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
