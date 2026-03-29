pub(crate) mod cursor;
mod keyboard;
mod mouse;

pub use cursor::{Cursor, map_cursor};
pub use keyboard::{Key, KeyState, Modifiers};
pub use mouse::{MouseButton, MouseState};

/// all input state for one window.
/// lives on BentoWindow, updated each frame by the runner
pub struct InputState {
    pub mouse: MouseState,
    pub keyboard: KeyState,
    pub cursor: Cursor,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse: MouseState::new(),
            keyboard: KeyState::new(),
            cursor: Cursor::Default,
        }
    }

    /// reset per frame flags
    /// call at the end of each frame
    pub fn reset(&mut self) {
        self.mouse.reset();
        self.keyboard.reset();
        self.cursor = Cursor::Default;
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
