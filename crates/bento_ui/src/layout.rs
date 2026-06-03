#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Size {
    Auto,           // size to content
    Fixed(f32),     // fixed pixels
    Fill,           // fill available space
    Percent(f32),   // percentage of available space
    FillMinus(f32), // fill minus fixed amount
}

impl Size {
    pub fn resolve(&self, available: f32, content: f32) -> f32 {
        match self {
            Size::Auto => content,
            Size::Fixed(v) => *v,
            Size::Fill => available,
            Size::Percent(p) => available * p / 100.0,
            Size::FillMinus(v) => (available - v).max(0.0),
        }
    }

    pub fn is_fill(&self) -> bool {
        matches!(self, Size::Fill | Size::FillMinus(_))
    }

    pub fn is_auto(&self) -> bool {
        matches!(self, Size::Auto)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MainAxis {
    Start,
    Center,
    End,
    SpaceBetween,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CrossAxis {
    Start,
    Center,
    End,
    Stretch,
}
