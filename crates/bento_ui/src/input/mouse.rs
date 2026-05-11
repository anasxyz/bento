pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub struct MouseButtonState {
    pub pressed: bool,
    pub released: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}

pub struct Mouse {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,

    pub scroll_x: f32,
    pub scroll_y: f32,

    pub left: MouseButtonState,
    pub right: MouseButtonState,
    pub middle: MouseButtonState,

    pub inside_window: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            dx: 0.0,
            dy: 0.0,

            scroll_x: 0.0,
            scroll_y: 0.0,

            left: MouseButtonState {
                pressed: false,
                released: false,
                just_pressed: false,
                just_released: false,
            },
            right: MouseButtonState {
                pressed: false,
                released: false,
                just_pressed: false,
                just_released: false,
            },
            middle: MouseButtonState {
                pressed: false,
                released: false,
                just_pressed: false,
                just_released: false,
            },

            inside_window: false,
        }
    }
}
