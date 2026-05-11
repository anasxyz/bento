use super::Mouse;

pub struct Input {
    pub mouse: Mouse,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse: Mouse::new(),
        }
    }
}
