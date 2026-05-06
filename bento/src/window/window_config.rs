use crate::color::Color;

#[derive(Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub clear_color: Color,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "demo".to_string(),
            width: 800,
            height: 600,
            clear_color: Color::rgb(10, 10, 10),
        }
    }
}
