use super::Mouse;
use super::Keyboard;

pub struct Input {
    pub mouse: Mouse,
    pub keyboard: Keyboard,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
        }
    }
}
