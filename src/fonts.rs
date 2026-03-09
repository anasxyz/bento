use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Weight, fontdb};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(pub(crate) usize);

pub struct FontEntry {
    pub family: String,
    pub size: f32,
}

pub struct Fonts {
    pub(crate) font_system: FontSystem,
    entries: Vec<FontEntry>,
    measure_cache: HashMap<(String, String, u32), (f32, f32)>,
    name_to_id: HashMap<String, FontId>,
    pub(crate) default: Option<FontId>,
    fonts_loaded: bool,
}

pub struct FontBuilder<'a> {
    fonts: &'a mut Fonts,
    id: FontId,
}

impl<'a> FontBuilder<'a> {
    pub fn default(self) -> FontId {
        self.fonts.default = Some(self.id);
        self.id
    }
}

impl Fonts {
    pub fn new() -> Self {
        let db = fontdb::Database::new();
        let mut font_system = FontSystem::new_with_locale_and_db("en-US".to_string(), db);
        font_system.db_mut().load_system_fonts();
        Self {
            font_system,
            entries: Vec::new(),
            measure_cache: HashMap::new(),
            name_to_id: HashMap::new(),
            default: None,
            fonts_loaded: true,
        }
    }

    pub fn add(&mut self, name: &str, family: &str, size: f32) -> FontBuilder<'_> {
        if !self.fonts_loaded {
            self.fonts_loaded = true;
            self.font_system.db_mut().load_system_fonts();
        }
        let id = if let Some(&existing) = self.name_to_id.get(name) {
            existing
        } else {
            let id = FontId(self.entries.len());
            self.entries.push(FontEntry {
                family: family.to_string(),
                size,
            });
            self.name_to_id.insert(name.to_string(), id);
            id
        };
        FontBuilder { fonts: self, id }
    }

    pub fn get(&self, id: FontId) -> &FontEntry {
        &self.entries[id.0]
    }

    pub fn get_by_name(&self, name: &str) -> Option<FontId> {
        self.name_to_id.get(name).copied()
    }

    pub fn default_id(&self) -> Option<FontId> {
        self.default
    }

    pub fn resolve(&self, name: Option<&str>) -> Option<FontId> {
        match name {
            Some(n) => self.get_by_name(n).or(self.default),
            None => self.default,
        }
    }

    // measure at the font's default size and weight 400
    pub fn measure(&mut self, text: &str, id: FontId) -> (f32, f32) {
        let entry = &self.entries[id.0];
        let family = entry.family.clone();
        let size = entry.size;
        self.measure_sized(text, &family, size, 400, false, None)
    }

    // measure at an explicit size, weight, and italic
    // all three must match what's used when rendering, otherwise layout will be off
    pub fn measure_sized(
        &mut self,
        text: &str,
        family: &str,
        size: f32,
        weight: u16,
        italic: bool,
        max_width: Option<f32>,
    ) -> (f32, f32) {
        let key = (
            format!(
                "{}:{}:{}:{}",
                family,
                weight,
                italic as u8,
                max_width.map(|w| w as u32).unwrap_or(0)
            ),
            text.to_string(),
            (size * 10.0) as u32,
        );
        if let Some(&cached) = self.measure_cache.get(&key) {
            return cached;
        }
        let line_height = size * 1.4;
        let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(size, line_height));
        buffer.set_size(&mut self.font_system, max_width, None);
        buffer.set_text(
            &mut self.font_system,
            text,
            &Attrs::new()
                .family(Family::Name(family))
                .weight(Weight(weight))
                .style(if italic {
                    glyphon::Style::Italic
                } else {
                    glyphon::Style::Normal
                }),
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(&mut self.font_system, false);
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        for run in buffer.layout_runs() {
            width = width.max(run.line_w);
            height += line_height;
        }
        let result = (width, height);
        self.measure_cache.insert(key, result);
        result
    }
}
