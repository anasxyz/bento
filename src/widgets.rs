// src/widgets.rs - Using Canvas directly

use crate::Canvas;

// src/shapes.rs - Shape builders (add to existing shapes.rs)

/// A rectangle shape
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4],
    pub outline_color: [f32; 4],
    pub outline_thickness: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color: [1.0, 1.0, 1.0, 1.0],
            outline_color: [0.0, 0.0, 0.0, 0.0],
            outline_thickness: 0.0,
        }
    }

    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn outline(mut self, color: [f32; 4], thickness: f32) -> Self {
        self.outline_color = color;
        self.outline_thickness = thickness;
        self
    }

    pub fn draw(&self, canvas: &mut crate::Canvas) {
        canvas.rect(
            self.x,
            self.y,
            self.width,
            self.height,
            self.color,
            self.outline_color,
            self.outline_thickness,
        );
    }
}

/// A circle shape
pub struct Circle {
    pub cx: f32,
    pub cy: f32,
    pub radius: f32,
    pub color: [f32; 4],
    pub outline_color: [f32; 4],
    pub outline_thickness: f32,
}

impl Circle {
    pub fn new(cx: f32, cy: f32, radius: f32) -> Self {
        Self {
            cx,
            cy,
            radius,
            color: [1.0, 1.0, 1.0, 1.0],
            outline_color: [0.0, 0.0, 0.0, 0.0],
            outline_thickness: 0.0,
        }
    }

    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn outline(mut self, color: [f32; 4], thickness: f32) -> Self {
        self.outline_color = color;
        self.outline_thickness = thickness;
        self
    }

    pub fn draw(&self, canvas: &mut crate::Canvas) {
        canvas.circle(
            self.cx,
            self.cy,
            self.radius,
            self.color,
            self.outline_color,
            self.outline_thickness,
        );
    }
}

/// A rounded rectangle shape
pub struct RoundedRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub radius: f32,
    pub color: [f32; 4],
    pub outline_color: [f32; 4],
    pub outline_thickness: f32,
}

impl RoundedRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32, radius: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            radius,
            color: [1.0, 1.0, 1.0, 1.0],
            outline_color: [0.0, 0.0, 0.0, 0.0],
            outline_thickness: 0.0,
        }
    }

    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn outline(mut self, color: [f32; 4], thickness: f32) -> Self {
        self.outline_color = color;
        self.outline_thickness = thickness;
        self
    }

    pub fn draw(&self, canvas: &mut crate::Canvas) {
        canvas.rounded_rect(
            self.x,
            self.y,
            self.width,
            self.height,
            self.radius,
            self.color,
            self.outline_color,
            self.outline_thickness,
        );
    }
}

/// A text label
pub struct Text {
    pub text: String,
    pub x: f32,
    pub y: f32,
}

impl Text {
    pub fn new(text: impl Into<String>, x: f32, y: f32) -> Self {
        Self {
            text: text.into(),
            x,
            y,
        }
    }

    pub fn draw(&self, canvas: &mut crate::Canvas) {
        canvas.text(&self.text, self.x, self.y);
    }
}

/// A clickable button widget
pub struct Button {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
    pub bg_color: [f32; 4],
    pub hover_color: [f32; 4],
    pub text_color: [f32; 4],
    pub border_color: [f32; 4],
    pub border_thickness: f32,
    pub corner_radius: f32,
    pub is_hovered: bool,
    pub is_pressed: bool,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: impl Into<String>) -> Self {
        Self {
            x,
            y,
            width,
            height,
            text: text.into(),
            bg_color: [0.2, 0.4, 0.8, 1.0],
            hover_color: [0.3, 0.5, 0.9, 1.0],
            text_color: [1.0, 1.0, 1.0, 1.0],
            border_color: [0.1, 0.2, 0.4, 1.0],
            border_thickness: 2.0,
            corner_radius: 8.0,
            is_hovered: false,
            is_pressed: false,
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Draw using canvas directly!
    pub fn draw(&self, canvas: &mut Canvas) {
        let color = if self.is_pressed {
            [
                self.bg_color[0] * 0.8,
                self.bg_color[1] * 0.8,
                self.bg_color[2] * 0.8,
                1.0,
            ]
        } else if self.is_hovered {
            self.hover_color
        } else {
            self.bg_color
        };

        // Direct canvas methods - no .scene!
        canvas.rounded_rect(
            self.x,
            self.y,
            self.width,
            self.height,
            self.corner_radius,
            color,
            self.border_color,
            self.border_thickness,
        );

        let text_x = self.x + 10.0;
        let text_y = self.y + self.height / 2.0 - 10.0;
        canvas.text(&self.text, text_x, text_y);
    }
}

/// A checkbox widget
pub struct Checkbox {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub label: String,
    pub checked: bool,
    pub bg_color: [f32; 4],
    pub check_color: [f32; 4],
    pub border_color: [f32; 4],
}

impl Checkbox {
    pub fn new(x: f32, y: f32, label: impl Into<String>) -> Self {
        Self {
            x,
            y,
            size: 20.0,
            label: label.into(),
            checked: false,
            bg_color: [1.0, 1.0, 1.0, 1.0],
            check_color: [0.2, 0.6, 0.2, 1.0],
            border_color: [0.5, 0.5, 0.5, 1.0],
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.size && y >= self.y && y <= self.y + self.size
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.rect(
            self.x,
            self.y,
            self.size,
            self.size,
            self.bg_color,
            self.border_color,
            2.0,
        );

        if self.checked {
            let margin = self.size * 0.2;
            canvas.rect(
                self.x + margin,
                self.y + margin,
                self.size - margin * 2.0,
                self.size - margin * 2.0,
                self.check_color,
                [0.0; 4],
                0.0,
            );
        }

        canvas.text(&self.label, self.x + self.size + 10.0, self.y);
    }
}

/// A slider widget
pub struct Slider {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub value: f32,
    pub track_color: [f32; 4],
    pub fill_color: [f32; 4],
    pub handle_color: [f32; 4],
    pub is_dragging: bool,
}

impl Slider {
    pub fn new(x: f32, y: f32, width: f32) -> Self {
        Self {
            x,
            y,
            width,
            value: 0.5,
            track_color: [0.3, 0.3, 0.3, 1.0],
            fill_color: [0.2, 0.6, 0.9, 1.0],
            handle_color: [1.0, 1.0, 1.0, 1.0],
            is_dragging: false,
        }
    }

    pub fn handle_x(&self) -> f32 {
        self.x + self.value * self.width
    }

    pub fn contains_handle(&self, x: f32, y: f32) -> bool {
        let handle_x = self.handle_x();
        let handle_radius = 8.0;
        let dx = x - handle_x;
        let dy = y - (self.y + 5.0);
        dx * dx + dy * dy <= handle_radius * handle_radius
    }

    pub fn set_value_from_x(&mut self, x: f32) {
        self.value = ((x - self.x) / self.width).clamp(0.0, 1.0);
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        let track_height = 4.0;
        let handle_radius = 8.0;

        canvas.rounded_rect(
            self.x,
            self.y,
            self.width,
            track_height,
            track_height / 2.0,
            self.track_color,
            [0.0; 4],
            0.0,
        );

        canvas.rounded_rect(
            self.x,
            self.y,
            self.value * self.width,
            track_height,
            track_height / 2.0,
            self.fill_color,
            [0.0; 4],
            0.0,
        );

        canvas.circle(
            self.handle_x(),
            self.y + track_height / 2.0,
            handle_radius,
            self.handle_color,
            [0.2, 0.2, 0.2, 1.0],
            2.0,
        );
    }
}

/// A card/panel widget
pub struct Card {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub bg_color: [f32; 4],
    pub border_color: [f32; 4],
    pub corner_radius: f32,
}

impl Card {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            bg_color: [1.0, 1.0, 1.0, 1.0],
            border_color: [0.85, 0.85, 0.85, 1.0],
            corner_radius: 12.0,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.rounded_rect(
            self.x,
            self.y,
            self.width,
            self.height,
            self.corner_radius,
            self.bg_color,
            self.border_color,
            1.0,
        );
    }
}

/// A progress bar widget
pub struct ProgressBar {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub progress: f32,
    pub bg_color: [f32; 4],
    pub fill_color: [f32; 4],
    pub border_color: [f32; 4],
}

impl ProgressBar {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            progress: 0.0,
            bg_color: [0.9, 0.9, 0.9, 1.0],
            fill_color: [0.2, 0.8, 0.4, 1.0],
            border_color: [0.7, 0.7, 0.7, 1.0],
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        let corner_radius = self.height / 2.0;

        canvas.rounded_rect(
            self.x,
            self.y,
            self.width,
            self.height,
            corner_radius,
            self.bg_color,
            self.border_color,
            1.0,
        );

        if self.progress > 0.0 {
            let fill_width = self.width * self.progress.clamp(0.0, 1.0);
            canvas.rounded_rect(
                self.x,
                self.y,
                fill_width,
                self.height,
                corner_radius,
                self.fill_color,
                [0.0; 4],
                0.0,
            );
        }
    }
}
