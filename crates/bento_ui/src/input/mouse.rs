use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub just_entered: bool,
    pub just_left: bool,
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
            just_entered: false,
            just_left: false,
        }
    }

    pub fn clear(&mut self) {
        self.dx = 0.0;
        self.dy = 0.0;

        self.scroll_x = 0.0;
        self.scroll_y = 0.0;

        self.left.just_pressed = false;
        self.left.just_released = false;

        self.right.just_pressed = false;
        self.right.just_released = false;

        self.middle.just_pressed = false;
        self.middle.just_released = false;

        self.just_entered = false;
        self.just_left = false;
    }
}

impl fmt::Display for Mouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Mouse:")?;
        writeln!(
            f,
            "  pos=({:.0},{:.0}) delta=({:.0},{:.0}) scroll=({:.0},{:.0}) inside={}",
            self.x, self.y, self.dx, self.dy, self.scroll_x, self.scroll_y, self.inside_window
        )?;
        writeln!(
            f,
            "  L: pressed={} released={} just_pressed={} just_released={}",
            self.left.pressed, self.left.released, self.left.just_pressed, self.left.just_released
        )?;
        writeln!(
            f,
            "  R: pressed={} released={} just_pressed={} just_released={}",
            self.right.pressed,
            self.right.released,
            self.right.just_pressed,
            self.right.just_released
        )?;
        writeln!(
            f,
            "  M: pressed={} released={} just_pressed={} just_released={}",
            self.middle.pressed,
            self.middle.released,
            self.middle.just_pressed,
            self.middle.just_released
        )
    }
}
