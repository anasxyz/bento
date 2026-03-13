// size
#[derive(Debug, Clone)]
pub enum Size {
    Fixed(f32),
    Percent(f32),
    Auto,
}

impl Default for Size {
    fn default() -> Self {
        Size::Auto
    }
}

impl Size {
    pub fn resolve(&self, parent: f32) -> f32 {
        match self {
            Size::Fixed(v) => *v,
            Size::Percent(p) => parent * p / 100.0,
            Size::Auto => 0.0,
        }
    }
}

// position
#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    Relative,
    Absolute,
}

impl Default for Position {
    fn default() -> Self {
        Position::Relative
    }
}

// overflow
#[derive(Debug, Clone, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Hidden
    }
}

// align items
#[derive(Debug, Clone, PartialEq)]
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl Default for AlignItems {
    fn default() -> Self {
        AlignItems::Stretch
    }
}

// align self
#[derive(Debug, Clone, PartialEq)]
pub enum AlignSelf {
    Auto,
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl Default for AlignSelf {
    fn default() -> Self {
        AlignSelf::Auto
    }
}

// justify content
#[derive(Debug, Clone, PartialEq)]
pub enum JustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> Self {
        JustifyContent::Start
    }
}

// flex wrap
#[derive(Debug, Clone, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> Self {
        FlexWrap::NoWrap
    }
}

// flex direction
#[derive(Debug, Clone, PartialEq)]
pub enum FlexDirection {
    Row,
    Col,
    RowReverse,
    ColReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        FlexDirection::Row
    }
}
