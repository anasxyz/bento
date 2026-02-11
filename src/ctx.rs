use crate::{FontId, Fonts, InputState, MouseState, ShapeRenderer, TextRenderer};

/// everything the user needs during setup and update
pub struct Ctx {
    pub(crate) text_renderer: TextRenderer,
    pub(crate) shape_renderer: ShapeRenderer,

    pub fonts: Fonts,
    pub mouse: MouseState,
    pub input: InputState,
    pub exit: bool,
}

impl Ctx {
    pub(crate) fn new(
        fonts: Fonts,
        text_renderer: TextRenderer,
        shape_renderer: ShapeRenderer,
    ) -> Self {
        Self {
            fonts,
            mouse: MouseState::default(),
            input: InputState::default(),
            exit: false,
            text_renderer,
            shape_renderer,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    // Drawing methods
    pub fn rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer
            .rect(x, y, w, h, color, outline_color, outline_thickness);
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

    pub fn text(&mut self, text: &str, font_id: FontId, x: f32, y: f32) {
        let entry = self.fonts.get(font_id);
        let family = entry.family.clone();
        let size = entry.size;
        self.text_renderer
            .draw(&mut self.fonts.font_system, family, size, text, x, y);
    }
}
