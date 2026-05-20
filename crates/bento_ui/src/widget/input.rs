use crate::{
    KeyPress, Ui,
    widget::{Widget, WidgetHandle},
};
use bento_shared::{GroupNode, RectNode, SceneNode, SceneNodeId, TextNode};

pub struct Input {
    id: Option<SceneNodeId>,
    bg: Option<SceneNodeId>,
    text_id: Option<SceneNodeId>,
    cursor_id: Option<SceneNodeId>,
    dirty: bool,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub value: String,
    pub placeholder: String,
    pub font_size: f32,
    pub bg_color: [f32; 4],
    pub text_color: [f32; 4],
    pub border_color: [f32; 4],
    pub focused_border_color: [f32; 4],
    slot_id: Option<u32>,
}

impl Input {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            id: None,
            bg: None,
            text_id: None,
            cursor_id: None,
            dirty: false,
            x,
            y,
            w,
            h,
            value: String::new(),
            placeholder: String::new(),
            font_size: 14.0,
            bg_color: [0.15, 0.15, 0.15, 1.0],
            text_color: [1.0, 1.0, 1.0, 1.0],
            border_color: [0.4, 0.4, 0.4, 1.0],
            focused_border_color: [0.4, 0.7, 1.0, 1.0],
            slot_id: None,
        }
    }

    pub fn placeholder(mut self, text: &str) -> Self {
        self.placeholder = text.to_string();
        self
    }

    pub fn is_focused(&self, ui: &Ui) -> bool {
        ui.focused() == self.slot_id
    }
}

impl Widget for Input {
    fn root(&self) -> Option<SceneNodeId> {
        self.id
    }
    fn name(&self) -> &str {
        "Input"
    }
    fn focusable(&self) -> bool {
        true
    }
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn build(&mut self, ui: &mut Ui, handle: WidgetHandle<()>) {
        self.slot_id = Some(handle.id);

        let mut g = GroupNode::new();
        g.offset_x = self.x;
        g.offset_y = self.y;
        g.x = self.x;
        g.y = self.y;
        g.w = self.w;
        g.h = self.h;
        let root = ui.scene_mut().add_group(g);

        let mut bg = RectNode::new(0.0, 0.0, self.w, self.h);
        bg.color = self.bg_color;
        bg.border_color = self.border_color;
        bg.border_widths = [1.0; 4];
        bg.radii = [4.0; 4];
        let bg_id = ui.scene_mut().add_rect(bg);

        let padding = 8.0;
        let ty = self.h / 2.0 - self.font_size / 2.0;
        let mut text = TextNode::new(&self.placeholder, padding, ty, self.font_size);
        text.color = [0.5, 0.5, 0.5, 1.0];
        let text_id = ui.scene_mut().add_text(text);

        let cursor_h = self.h - 10.0;
        let mut cursor = RectNode::new(padding, (self.h - cursor_h) / 2.0, 2.0, cursor_h);
        cursor.color = [1.0, 1.0, 1.0, 1.0];
        cursor.opacity = 0.0;
        let cursor_id = ui.scene_mut().add_rect(cursor);

        ui.scene_mut().append(root, bg_id);
        ui.scene_mut().append(root, text_id);
        ui.scene_mut().append(root, cursor_id);

        self.id = Some(root);
        self.bg = Some(bg_id);
        self.text_id = Some(text_id);
        self.cursor_id = Some(cursor_id);

        let input_handle: WidgetHandle<Input> = WidgetHandle::new(handle.id, handle.generation);

        ui.listen(root, move |e: &KeyPress, ui| {
            println!("keypress: {:?}", e.key);
            let focused = ui.focused() == Some(input_handle.id);
            println!("focused: {}", focused);
            if !focused {
                return;
            }
            let inp = ui.get_mut(input_handle).unwrap();
            match e.key {
                crate::Key::Backspace => {
                    inp.value.pop();
                    inp.dirty = true;
                }
                _ => {
                    if let Some(ch) = e.ch {
                        if !ch.is_control() {
                            inp.value.push(ch);
                            inp.dirty = true;
                        }
                    }
                }
            }
        });
    }

    fn update(&mut self, ui: &mut Ui) {
        let focused = ui.focused() == self.slot_id;
        let padding = 8.0;

        let display = if self.value.is_empty() {
            self.placeholder.clone()
        } else {
            self.value.clone()
        };
        let color = if self.value.is_empty() {
            [0.5, 0.5, 0.5, 1.0]
        } else {
            self.text_color
        };

        if let Some(SceneNode::Text(t)) = ui.scene_mut().get_mut(self.text_id.unwrap()) {
            t.text = display;
            t.color = color;
        }

        let cursor_x = padding + self.value.len() as f32 * self.font_size * 0.6;
        if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.cursor_id.unwrap()) {
            r.x = cursor_x;
            r.opacity = if focused { 1.0 } else { 0.0 };
        }

        if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(self.bg.unwrap()) {
            r.border_color = if focused {
                self.focused_border_color
            } else {
                self.border_color
            };
        }

        ui.needs_redraw = true;
        self.dirty = false;
    }

    fn remove(&mut self, ui: &mut Ui) {
        if let Some(id) = self.id {
            ui.scene_mut().remove(id);
        }
    }
}
