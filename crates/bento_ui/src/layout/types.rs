#[derive(Clone)]
pub enum Layout {
    None,
    Row {
        gap: f32,
        padding: [f32; 4],
        main_axis: MainAxis,
        cross_axis: CrossAxis,
        wrap: bool,
    },
    Column {
        gap: f32,
        padding: [f32; 4],
        main_axis: MainAxis,
        cross_axis: CrossAxis,
        wrap: bool,
    },
}

impl Default for Layout {
    fn default() -> Self {
        Layout::None
    }
}

#[derive(Clone, Default)]
pub enum MainAxis {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

#[derive(Clone, Default)]
pub enum CrossAxis {
    #[default]
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Debug)]
pub enum Size {
    Auto,
    Fixed(f32),
    Fill,
    Percent(f32),
    FillMinus(f32),
}

impl Size {
    pub fn resolve(&self, available: f32) -> f32 {
        match self {
            // placeholder for Auto, layout should not call resolve on it
            Size::Auto => 0.0,
            Size::Fixed(v) => *v,
            Size::Fill => available,
            Size::Percent(p) => available * p / 100.0,
            Size::FillMinus(v) => (available - v).max(0.0),
        }
    }

    pub fn is_auto(&self) -> bool {
        matches!(self, Size::Auto)
    }
}
