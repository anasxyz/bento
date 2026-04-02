use super::attrs::FontAttrs;
use super::cache::FontCache;
use super::measure::measure_text;
use cosmic_text::FontSystem;

pub struct Fonts {
    pub(crate) font_system: FontSystem,
    pub(crate) cache: FontCache,
}

impl Fonts {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            cache: FontCache::new(),
        }
    }

    pub fn load_font(&mut self, data: &[u8]) {
        self.font_system.db_mut().load_font_data(data.to_vec());
        self.cache.clear();
    }

    pub fn load_font_file(&mut self, path: &str) {
        self.font_system.db_mut().load_font_file(path).ok();
        self.cache.clear();
    }

    pub fn set_sans_serif(&mut self, family: &str) {
        self.font_system.db_mut().set_sans_serif_family(family);
        self.cache.clear();
    }

    pub fn set_monospace(&mut self, family: &str) {
        self.font_system.db_mut().set_monospace_family(family);
        self.cache.clear();
    }

    pub fn set_serif(&mut self, family: &str) {
        self.font_system.db_mut().set_serif_family(family);
        self.cache.clear();
    }

    pub fn measure(&mut self, text: &str, attrs: &FontAttrs, max_width: Option<f32>) -> (f32, f32) {
        measure_text(
            &mut self.font_system,
            &mut self.cache,
            text,
            attrs,
            max_width,
        )
    }
}

impl Default for Fonts {
    fn default() -> Self {
        Self::new()
    }
}
