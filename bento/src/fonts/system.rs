// system.rs
//
// Fonts is the user-facing font API.
// It wraps a &mut FontSystem borrowed from RenderContext — one per app.
// cosmic-text types never leak out of this module.

use super::attrs::FontAttrs;
use super::measure::measure_text;
use glyphon::FontSystem;

pub struct Fonts<'a> {
    font_system: &'a mut FontSystem,
}

impl<'a> Fonts<'a> {
    pub fn new(font_system: &'a mut FontSystem) -> Self {
        Self { font_system }
    }

    /// Load a custom font from raw bytes.
    /// The name is what you'll use in FontAttrs::family.
    pub fn load_font(&mut self, data: &[u8]) {
        self.font_system.db_mut().load_font_data(data.to_vec());
    }

    /// Load a custom font from a file path.
    pub fn load_font_file(&mut self, path: &str) {
        self.font_system.db_mut().load_font_file(path).ok();
    }

    /// Set the default sans-serif family used when "sans-serif" is specified.
    pub fn set_sans_serif(&mut self, family: &str) {
        self.font_system.db_mut().set_sans_serif_family(family);
    }

    /// Set the default monospace family.
    pub fn set_monospace(&mut self, family: &str) {
        self.font_system.db_mut().set_monospace_family(family);
    }

    /// Set the default serif family.
    pub fn set_serif(&mut self, family: &str) {
        self.font_system.db_mut().set_serif_family(family);
    }

    /// Measure text — returns (width, height) in logical pixels.
    /// This is what widgets call inside their measure() implementation.
    pub fn measure(&mut self, text: &str, attrs: &FontAttrs, max_width: Option<f32>) -> (f32, f32) {
        measure_text(self.font_system, text, attrs, max_width)
    }

    /// Direct access to the underlying FontSystem for advanced use.
    pub fn font_system_mut(&mut self) -> &mut FontSystem {
        self.font_system
    }
}
