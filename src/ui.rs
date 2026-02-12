use crate::{Color, FontId, Fonts, ShapeRenderer, TextRenderer};

pub struct Rect {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
    pub outline_color: Color,
    pub outline_thickness: f32,
    pub visible: bool,
}

pub struct Text {
    pub id: String,
    pub text: String,
    pub font_id: FontId,
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub font_size: f32,
    pub font_family: String,
    pub visible: bool,
}

pub struct Button {
    pub id: String,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub font_id: FontId,
    pub bg_color: Color,
    pub bg_color_hover: Color,
    pub text_color: Color,
    pub text_color_hover: Color,
    pub outline_color: Color,
    pub outline_color_hover: Color,
    pub outline_thickness: f32,
    pub padding: f32,
    pub visible: bool,
}

pub struct Ui {
    pub(crate) text_renderer: TextRenderer,
    pub(crate) shape_renderer: ShapeRenderer,
    pub fonts: Fonts,

    pub rects: Vec<Rect>,
    pub texts: Vec<Text>,
    pub buttons: Vec<Button>,

    pub dirty: bool,
}

impl Ui {
    pub fn new(text_renderer: TextRenderer, shape_renderer: ShapeRenderer, fonts: Fonts) -> Self {
        Self {
            text_renderer,
            shape_renderer,
            fonts,
            rects: Vec::new(),
            texts: Vec::new(),
            buttons: Vec::new(),
            dirty: false,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub(crate) fn take_dirty(&mut self) -> bool {
        let d = self.dirty;
        self.dirty = false;
        d
    }

    pub fn rect(
        &mut self,
        id: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Color,
        outline_color: Color,
        outline_thickness: f32,
    ) {
        if self.id_exists(id) {
            panic!("Element with id '{}' already exists!", id);
        }

        let new_rect = Rect {
            id: id.to_string(),
            x,
            y,
            w,
            h,
            color,
            outline_color,
            outline_thickness,
            visible: true,
        };
        self.rects.push(new_rect);
        self.mark_dirty();
    }

    pub fn text(&mut self, id: &str, text: &str, font_id: FontId, x: f32, y: f32, color: Color) {
        if self.id_exists(id) {
            panic!("Element with id '{}' already exists!", id);
        }

        let entry = self.fonts.get(font_id);
        let family = entry.family.clone();
        let size = entry.size;

        let new_text = Text {
            id: id.to_string(),
            text: text.to_string(),
            font_id,
            x,
            y,
            color,
            font_size: size,
            font_family: family,
            visible: true,
        };
        self.texts.push(new_text);
        self.mark_dirty();
    }

    pub fn button(&mut self, id: &str, text: &str, x: f32, y: f32) {
        if self.id_exists(id) {
            panic!("Element with id '{}' already exists!", id);
        }

        let padding = self.fonts.default_padding;

        let font_id = self.fonts.get_by_name("default").unwrap();
        let bg_color = Color::rgb(0.27, 0.51, 0.71);
        let text_color = Color::WHITE;
        let outline_color = Color::TRANSPARENT;
        let outline_thickness = 0.0;

        let (text_width, text_height) = self.fonts.measure(text, font_id);

        let button_width = text_width + padding * 2.0;
        let button_height = text_height + padding * 2.0;

        let bg_color_hover = Color::from_array(bg_color.to_array().map(|c| c * 0.8));
        let text_color_hover = Color::from_array(text_color.to_array().map(|c| c * 0.8));
        let outline_color_hover = Color::from_array(outline_color.to_array().map(|c| c * 0.8));

        let new_button = Button {
            id: id.to_string(),
            text: text.to_string(),
            x,
            y,
            w: button_width,
            h: button_height,
            font_id,
            bg_color,
            bg_color_hover,
            text_color,
            text_color_hover,
            outline_color,
            outline_color_hover,
            outline_thickness,
            padding,
            visible: true,
        };

        self.buttons.push(new_button);
        self.mark_dirty();
    }

    pub fn circle(
        &mut self,
        cx: f32,
        cy: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer
            .circle(cx, cy, radius, color, outline_color, outline_thickness);
    }

    pub fn rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer.rounded_rect(
            x,
            y,
            w,
            h,
            radius,
            color,
            outline_color,
            outline_thickness,
        );
    }

    fn id_exists(&self, id: &str) -> bool {
        self.rects.iter().any(|r| r.id == id)
            || self.texts.iter().any(|t| t.id == id)
            || self.buttons.iter().any(|b| b.id == id)
    }

    pub fn show(&mut self, id: &str) {
        for rect in &mut self.rects {
            if rect.id == id {
                rect.visible = true;
                self.mark_dirty();
                return;
            }
        }
        for text in &mut self.texts {
            if text.id == id {
                text.visible = true;
                self.mark_dirty();
                return;
            }
        }
        for button in &mut self.buttons {
            if button.id == id {
                button.visible = true;
                self.mark_dirty();
                return;
            }
        }
    }

    pub fn hide(&mut self, id: &str) {
        for rect in &mut self.rects {
            if rect.id == id {
                rect.visible = false;
                self.mark_dirty();
                return;
            }
        }
        for text in &mut self.texts {
            if text.id == id {
                text.visible = false;
                self.mark_dirty();
                return;
            }
        }
        for button in &mut self.buttons {
            if button.id == id {
                button.visible = false;
                self.mark_dirty();
                return;
            }
        }
    }

    pub fn toggle(&mut self, id: &str) {
        for rect in &mut self.rects {
            if rect.id == id {
                rect.visible = !rect.visible;
                self.mark_dirty();
                return;
            }
        }
        for text in &mut self.texts {
            if text.id == id {
                text.visible = !text.visible;
                self.mark_dirty();
                return;
            }
        }
        for button in &mut self.buttons {
            if button.id == id {
                button.visible = !button.visible;
                self.mark_dirty();
                return;
            }
        }
    }

    pub fn is_visible(&self, id: &str) -> bool {
        for rect in &self.rects {
            if rect.id == id {
                return rect.visible;
            }
        }
        for text in &self.texts {
            if text.id == id {
                return text.visible;
            }
        }
        for button in &self.buttons {
            if button.id == id {
                return button.visible;
            }
        }
        false
    }

}
