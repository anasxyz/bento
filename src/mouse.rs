
// all mouse state in one place
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

