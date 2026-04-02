use crate::color::Color;
use crate::ui::{Change, Hover, HoverEnd, Press, Ui};
use crate::widget::{AsAny, Base, Handle, HasBase, Widget};
use bento_derive::Widget;
use bento_wgpu::{RectId, SceneGraph, SceneNodeId, TransformId};

#[derive(Widget)]
pub struct Checkbox {
    pub base: Base,
    pub checked: bool,

    // style
    size: f32,
    radius: f32,
    color: Color,
    checked_color: Color,
    hover_color: Color,
    check_color: Color,
    border_color: Color,
    border_width: f32,

    // state
    hovered: bool,

    // scene nodes
    transform_id: Option<TransformId>,
    bg_id: Option<RectId>,
    check_id: Option<RectId>,
}

impl Checkbox {
    pub fn new() -> Self {
        Self {
            base: Base::new(),
            checked: false,
            size: 18.0,
            radius: 4.0,
            color: Color::rgb(30, 30, 36),
            checked_color: Color::rgb(99, 102, 241),
            hover_color: Color::rgb(50, 50, 60),
            check_color: Color::WHITE,
            border_color: Color::rgb(80, 80, 100),
            border_width: 1.5,
            hovered: false,
            transform_id: None,
            bg_id: None,
            check_id: None,
        }
    }

    pub fn set_checked(&mut self, v: bool) -> &mut Self {
        self.checked = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_size(&mut self, v: f32) -> &mut Self {
        self.size = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_radius(&mut self, v: f32) -> &mut Self {
        self.radius = v;
        self.base.render_dirty = true;
        self
    }
    pub fn set_color(&mut self, c: Color) -> &mut Self {
        self.color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_checked_color(&mut self, c: Color) -> &mut Self {
        self.checked_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_hover_color(&mut self, c: Color) -> &mut Self {
        self.hover_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_check_color(&mut self, c: Color) -> &mut Self {
        self.check_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_border_color(&mut self, c: Color) -> &mut Self {
        self.border_color = c;
        self.base.render_dirty = true;
        self
    }
    pub fn set_border_width(&mut self, v: f32) -> &mut Self {
        self.border_width = v;
        self.base.render_dirty = true;
        self
    }

    fn current_bg_color(&self) -> Color {
        if self.checked {
            self.checked_color
        } else if self.hovered {
            self.hover_color
        } else {
            self.color
        }
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Checkbox {
    fn build(&mut self, scene: &mut SceneGraph) {
        self.transform_id = Some(scene.add_transform());
        self.bg_id = Some(scene.add_rect());
        self.check_id = Some(scene.add_rect());

        let transform = self.transform_id.unwrap();
        let bg = self.bg_id.unwrap();
        let check = self.check_id.unwrap();

        scene.add_child(SceneNodeId(transform.0), SceneNodeId(bg.0));
        scene.add_child(SceneNodeId(transform.0), SceneNodeId(check.0));
    }

    fn register(&mut self, handle: Handle<()>, ui: &mut Ui) {
        let h = Handle::<Checkbox>::new(handle.id, handle.generation);

        ui.on::<Checkbox, Hover>(h, |ui, this, e| {
            this.hovered = true;
            this.base.cursor = crate::input::Cursor::Pointer;
            this.base.render_dirty = true;
        });

        ui.on::<Checkbox, HoverEnd>(h, |ui, this, e| {
            this.hovered = false;
            this.base.cursor = crate::input::Cursor::Default;
            this.base.render_dirty = true;
        });

        ui.on::<Checkbox, Press>(h, move |ui, this, e| {
            this.checked = !this.checked;
            this.base.render_dirty = true;
            let value = this.checked.to_string();
            ui.emit(handle, Change::new(value));
        });
    }

    fn sync(&mut self, scene: &mut SceneGraph, x: f32, y: f32, w: f32, h: f32, layer: u32) {
        let visible = self.base.visible;
        if let Some(id) = self.bg_id {
            let n = scene.rect_mut(id);
            n.set_rect(x, y, self.size, self.size);
            n.set_color(self.current_bg_color().to_array());
            n.set_radius(self.radius);
            n.set_border_color(if self.checked {
                Color::TRANSPARENT.to_array()
            } else {
                self.border_color.to_array()
            });
            n.set_border_widths([self.border_width; 4]);
            n.set_z(layer as i32);
            n.set_visible(visible);
        }

        if let Some(id) = self.check_id {
            let n = scene.rect_mut(id);
            if self.checked && visible {
                let pad = self.size * 0.25;
                n.set_rect(
                    x + pad,
                    y + pad,
                    self.size - pad * 2.0,
                    self.size - pad * 2.0,
                );
                n.set_color(self.check_color.to_array());
                n.set_radius((self.radius - pad).max(1.0));
                n.set_z(layer as i32);
                n.set_visible(true);
            } else {
                n.set_visible(false);
            }
        }
    }

    fn children_attachment_node(&self) -> Option<SceneNodeId> {
        self.transform_id.map(|id| SceneNodeId(id.0))
    }

    fn is_interactive(&self) -> bool {
        true
    }

    fn measure(
        &mut self,
        _fonts: &mut crate::fonts::Fonts,
        _max_width: Option<f32>,
    ) -> Option<(f32, f32)> {
        Some((self.size, self.size))
    }

    fn has_measure(&self) -> bool {
        true
    }
}
