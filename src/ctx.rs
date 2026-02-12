use crate::{Color, FontId, Fonts, InputState, MouseState, ShapeRenderer, TextRenderer, ui::Ui};

/// everything the user needs during setup and update
pub struct Ctx {
    pub ui: Ui,

    pub mouse: MouseState,
    pub input: InputState,
    pub exit: bool,

    dirty: bool,
}

impl Ctx {
    pub(crate) fn new(
        fonts: Fonts,
        text_renderer: TextRenderer,
        shape_renderer: ShapeRenderer,
    ) -> Self {
        Self {
            ui: Ui::new(text_renderer, shape_renderer, fonts),
            mouse: MouseState::default(),
            input: InputState::default(),
            exit: false,
            dirty: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn mark_dirty(&mut self) {
        self.ui.dirty = true;
    }

    pub(crate) fn take_dirty(&mut self) -> bool {
        let d = self.ui.dirty;
        self.ui.dirty = false;
        d
    }

    pub fn is_hovered(&self, id: &str) -> bool {
        for rect in &self.ui.rects {
            if rect.id == id {
                return rect.visible && self.mouse.is_over(rect.x, rect.y, rect.w, rect.h);
            }
        }

        for button in &self.ui.buttons {
            if button.id == id {
                return button.visible && self.mouse.is_over(button.x, button.y, button.w, button.h);
            }
        }

        false
    }

    pub fn is_clicked(&self, id: &str) -> bool {
        self.mouse.left_just_pressed && self.is_hovered(id)
    }

    pub fn is_right_clicked(&self, id: &str) -> bool {
        self.mouse.right_just_pressed && self.is_hovered(id)
    }

    pub fn render_all(&mut self) {
        self.render_rects();
        self.render_texts();
        self.render_buttons();
    }

    pub fn render_rects(&mut self) {
        for rect in &self.ui.rects {
            if !rect.visible {
                continue;
            }
            self.ui.shape_renderer.rect(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                rect.color.to_array(),
                rect.outline_color.to_array(),
                rect.outline_thickness,
            );
        }
    }

    pub fn render_texts(&mut self) {
        for text in &self.ui.texts {
            if !text.visible {
                continue;
            }
            self.ui.text_renderer.draw(
                &mut self.ui.fonts.font_system,
                text.font_family.clone(),
                text.font_size,
                &text.text,
                text.x,
                text.y,
                text.color,
            );
        }
    }

    pub fn render_buttons(&mut self) {
        let button_data: Vec<_> = self
            .ui
            .buttons
            .iter()
            .filter(|button| button.visible)
            .map(|button| {
                let entry = self.ui.fonts.get(button.font_id);
                let hovered = self.is_hovered(&button.id);

                let (bg_color, text_color, outline_color) = if hovered {
                    (
                        button.bg_color_hover,
                        button.text_color_hover,
                        button.outline_color_hover,
                    )
                } else {
                    (button.bg_color, button.text_color, button.outline_color)
                };

                (
                    button.x,
                    button.y,
                    button.w,
                    button.h,
                    bg_color,
                    text_color,
                    outline_color,
                    button.outline_thickness,
                    button.padding,
                    button.text.clone(),
                    entry.family.clone(),
                    entry.size,
                )
            })
            .collect();

        for (
            x,
            y,
            w,
            h,
            bg_color,
            text_color,
            outline_color,
            outline_thickness,
            padding,
            text,
            family,
            size,
        ) in button_data
        {
            self.ui.shape_renderer.rect(
                x,
                y,
                w,
                h,
                bg_color.to_array(),
                outline_color.to_array(),
                outline_thickness,
            );

            let text_x = x + padding;
            let text_y = y + padding;

            self.ui.text_renderer.draw(
                &mut self.ui.fonts.font_system,
                family,
                size,
                &text,
                text_x,
                text_y,
                text_color,
            );
        }
    }
}
