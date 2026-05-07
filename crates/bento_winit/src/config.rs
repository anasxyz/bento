pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub clear_color: [f32; 4],
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "demo".to_string(),
            width: 800,
            height: 600,
            clear_color: [0.1, 0.1, 0.1, 1.0],
        }
    }
}
