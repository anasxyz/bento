#[derive(Debug, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

use crate::element::layout::Layout;

const DRAG_THRESHOLD: f32 = 4.0;
const DOUBLE_CLICK_MS: u128 = 300;

#[derive(Debug)]
pub struct MouseState {
    pub x: f32,
    pub y: f32,

    pub left_pressed: bool,
    pub left_just_pressed: bool,
    pub left_just_released: bool,
    pub left_just_double_clicked: bool,

    pub right_pressed: bool,
    pub right_just_pressed: bool,
    pub right_just_released: bool,

    pub middle_pressed: bool,
    pub middle_just_pressed: bool,
    pub middle_just_released: bool,

    pub is_dragging: bool,
    pub drag_start_x: f32,
    pub drag_start_y: f32,

    pub left_click_x: f32,
    pub left_click_y: f32,
    pub right_click_x: f32,
    pub right_click_y: f32,
    pub middle_click_x: f32,
    pub middle_click_y: f32,

    left_click_count: u32,
    left_last_click: std::time::Instant,
    right_click_count: u32,
    right_last_click: std::time::Instant,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            left_pressed: false,
            left_just_pressed: false,
            left_just_released: false,
            left_just_double_clicked: false,
            right_pressed: false,
            right_just_pressed: false,
            right_just_released: false,
            middle_pressed: false,
            middle_just_pressed: false,
            middle_just_released: false,
            is_dragging: false,
            drag_start_x: 0.0,
            drag_start_y: 0.0,
            left_click_x: 0.0,
            left_click_y: 0.0,
            right_click_x: 0.0,
            right_click_y: 0.0,
            middle_click_x: 0.0,
            middle_click_y: 0.0,
            left_click_count: 0,
            left_last_click: std::time::Instant::now(),
            right_click_count: 0,
            right_last_click: std::time::Instant::now(),
        }
    }
}

impl MouseState {
    pub fn on_left_press(&mut self) {
        self.left_pressed = true;
        self.left_just_pressed = true;
        self.left_click_x = self.x;
        self.left_click_y = self.y;

        let now = std::time::Instant::now();
        if now.duration_since(self.left_last_click).as_millis() < DOUBLE_CLICK_MS {
            self.left_click_count += 1;
        } else {
            self.left_click_count = 1;
        }
        self.left_just_double_clicked = self.left_click_count >= 2;
        if self.left_just_double_clicked {
            self.left_click_count = 0;
        }
        self.left_last_click = now;
    }

    pub fn on_left_release(&mut self) {
        self.left_pressed = false;
        self.left_just_released = true;
    }

    pub fn on_right_press(&mut self) {
        self.right_pressed = true;
        self.right_just_pressed = true;
        self.right_click_x = self.x;
        self.right_click_y = self.y;

        let now = std::time::Instant::now();
        if now.duration_since(self.right_last_click).as_millis() < DOUBLE_CLICK_MS {
            self.right_click_count += 1;
        } else {
            self.right_click_count = 1;
        }
        self.right_last_click = now;
    }

    pub fn on_right_release(&mut self) {
        self.right_pressed = false;
        self.right_just_released = true;
    }

    pub fn on_middle_press(&mut self) {
        self.middle_pressed = true;
        self.middle_just_pressed = true;
        self.middle_click_x = self.x;
        self.middle_click_y = self.y;
    }

    pub fn on_middle_release(&mut self) {
        self.middle_pressed = false;
        self.middle_just_released = true;
    }

    pub fn reset(&mut self) {
        self.left_just_pressed = false;
        self.left_just_released = false;
        self.left_just_double_clicked = false;
        self.right_just_pressed = false;
        self.right_just_released = false;
        self.middle_just_pressed = false;
        self.middle_just_released = false;
    }
}
