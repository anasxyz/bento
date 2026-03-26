#[derive(Debug, Clone, PartialEq)]
pub struct FontAttrs {
    pub family:      String,
    pub size:        f32,
    pub weight:      u16,
    pub italic:      bool,
    pub line_height: Option<f32>,  // none = auto (size * 1.4)
}

impl FontAttrs {
    pub fn new(family: &str, size: f32) -> Self {
        Self {
            family:      family.to_string(),
            size,
            weight:      400,
            italic:      false,
            line_height: None,
        }
    }

    pub fn line_height(&self) -> f32 {
        self.line_height.unwrap_or(self.size * 1.4)
    }
}

impl Default for FontAttrs {
    fn default() -> Self {
        Self::new("sans-serif", 14.0)
    }
}
