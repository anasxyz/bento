use std::time::Instant;

const DOUBLE_CLICK_MS: u128 = 300;

#[derive(Debug, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// state for a single mouse button
#[derive(Debug)]
pub struct ButtonState {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
    pub just_double_clicked: bool,
    pub click_x: f32,
    pub click_y: f32,
    click_count: u32,
    last_click: Instant,
}

impl ButtonState {
    fn new() -> Self {
        Self {
            pressed: false,
            just_pressed: false,
            just_released: false,
            just_double_clicked: false,
            click_x: 0.0,
            click_y: 0.0,
            click_count: 0,
            last_click: Instant::now(),
        }
    }

    pub(crate) fn on_press(&mut self, x: f32, y: f32) {
        self.pressed = true;
        self.just_pressed = true;
        self.click_x = x;
        self.click_y = y;

        let now = Instant::now();
        if now.duration_since(self.last_click).as_millis() < DOUBLE_CLICK_MS {
            self.click_count += 1;
        } else {
            self.click_count = 1;
        }
        self.just_double_clicked = self.click_count >= 2;
        if self.just_double_clicked {
            self.click_count = 0;
        }
        self.last_click = now;
    }

    pub(crate) fn on_release(&mut self) {
        self.pressed = false;
        self.just_released = true;
    }

    pub(crate) fn reset(&mut self) {
        self.just_pressed = false;
        self.just_released = false;
        self.just_double_clicked = false;
    }
}

/// full mouse state for one frame
#[derive(Debug)]
pub struct MouseState {
    pub x: f32,
    pub y: f32,
    pub left: ButtonState,
    pub right: ButtonState,
    pub middle: ButtonState,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub just_scrolled: bool,
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            left: ButtonState::new(),
            right: ButtonState::new(),
            middle: ButtonState::new(),
            scroll_x: 0.0,
            scroll_y: 0.0,
            just_scrolled: false,
        }
    }

    pub(crate) fn on_move(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub(crate) fn on_press(&mut self, button: &MouseButton) {
        let (x, y) = (self.x, self.y);
        match button {
            MouseButton::Left => self.left.on_press(x, y),
            MouseButton::Right => self.right.on_press(x, y),
            MouseButton::Middle => self.middle.on_press(x, y),
        }
    }

    pub(crate) fn on_release(&mut self, button: &MouseButton) {
        match button {
            MouseButton::Left => self.left.on_release(),
            MouseButton::Right => self.right.on_release(),
            MouseButton::Middle => self.middle.on_release(),
        }
    }

    pub(crate) fn on_scroll(&mut self, x: f32, y: f32) {
        self.scroll_x = x;
        self.scroll_y = y;
        self.just_scrolled = true;
    }

    /// reset per frame flags
    /// call at the end of each frame
    pub(crate) fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
        self.middle.reset();
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
        self.just_scrolled = false;
    }
}

impl Default for MouseState {
    fn default() -> Self {
        Self::new()
    }
}
