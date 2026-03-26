use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MeasureKey {
    pub text: String,
    pub family: String,
    pub weight: u16,
    pub italic: bool,
    pub size_x10: u32,
    pub max_width: u32,
}

pub struct FontCache {
    pub(crate) entries: HashMap<MeasureKey, (f32, f32)>,
}

impl FontCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, key: &MeasureKey) -> Option<(f32, f32)> {
        self.entries.get(key).copied()
    }

    pub fn insert(&mut self, key: MeasureKey, value: (f32, f32)) {
        self.entries.insert(key, value);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for FontCache {
    fn default() -> Self {
        Self::new()
    }
}
