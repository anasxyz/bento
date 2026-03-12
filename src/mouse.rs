use crate::ui::{Handle, Ui};

const DRAG_THRESHOLD: f32 = 4.0;

#[derive(Debug)]
pub struct MouseState {
    // mouse position
    pub x: f32,
    pub y: f32,

    // mouse button state
    pub left_pressed: bool,
    pub left_just_pressed: bool,
    pub left_just_released: bool,
    pub right_pressed: bool,
    pub right_just_pressed: bool,
    pub middle_pressed: bool,
    pub middle_just_pressed: bool,

    // drag
    pub is_dragging: bool,
    pub drag_start_x: f32,
    pub drag_start_y: f32,

    // left click count
    pub left_click_count: u32,
    pub left_click_x: f32,
    pub left_click_y: f32,

    // right click count
    pub right_click_count: u32,
    pub right_click_x: f32,
    pub right_click_y: f32,

    // middle click count
    pub middle_click_count: u32,
    pub middle_click_x: f32,
    pub middle_click_y: f32,

    // click timing
    pub right_click_timer: std::time::Instant,
    pub last_right_click_time: f64,
    pub left_click_timer: std::time::Instant,
    pub last_left_click_time: f64,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            left_pressed: false,
            left_just_pressed: false,
            left_just_released: false,
            right_pressed: false,
            right_just_pressed: false,
            middle_pressed: false,
            middle_just_pressed: false,
            is_dragging: false,
            drag_start_x: 0.0,
            drag_start_y: 0.0,
            left_click_count: 0,
            left_click_x: 0.0,
            left_click_y: 0.0,
            right_click_count: 0,
            right_click_x: 0.0,
            right_click_y: 0.0,
            middle_click_count: 0,
            middle_click_x: 0.0,
            middle_click_y: 0.0,
            right_click_timer: std::time::Instant::now(),
            last_right_click_time: 0.0,
            left_click_timer: std::time::Instant::now(),
            last_left_click_time: 0.0,
        }
    }
}

impl MouseState {
    pub fn reset(&mut self) {
        self.left_just_pressed = false;
        self.left_just_released = false;
        self.right_just_pressed = false;
        self.middle_just_pressed = false;
    }

    pub fn update_drag(&mut self) {
        if !self.left_pressed && !self.left_just_pressed {
            self.is_dragging = false;
        }
        if self.left_just_pressed {
            self.drag_start_x = self.x;
            self.drag_start_y = self.y;
            self.is_dragging = false;
        }
        if self.left_pressed {
            let dx = self.x - self.drag_start_x;
            let dy = self.y - self.drag_start_y;
            if (dx * dx + dy * dy).sqrt() > DRAG_THRESHOLD {
                self.is_dragging = true;
            }
        }
        if self.left_just_released {
            self.is_dragging = false;
        }
    }
}

pub fn event_tree(ui: &Ui, handle: Handle, mouse: &mut MouseState) {
    let el = match ui.get(handle) {
        Some(e) => e,
        None => return,
    };

    let x = el.style.x;
    let y = el.style.y;
    let w = el.style.w;
    let h = el.style.h;

    let _hovered = mouse.x >= x && mouse.x <= x + w && mouse.y >= y && mouse.y <= y + h;

    let children: Vec<Handle> = ui.children(handle).to_vec();
    for child in children {
        event_tree(ui, child, mouse);
    }
}
