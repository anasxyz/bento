pub mod input;
pub mod keyboard;
pub mod mouse;

use keyboard::Keyboard;
use mouse::Mouse;

pub struct InputState {
    pub mouse: Mouse,
    pub keyboard: Keyboard,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
        }
    }
}
