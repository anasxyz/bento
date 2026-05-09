pub struct Base {
    pub(crate) dirty: bool,
}

impl Base {
    pub fn new() -> Self {
        Self { dirty: true }
    }
}

impl Default for Base {
    fn default() -> Self {
        Self::new()
    }
}

pub trait HasBase {
    fn base(&self) -> &Base;
    fn base_mut(&mut self) -> &mut Base;
}
