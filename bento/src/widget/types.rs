#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Row,
    Col,
    RowReverse,
    ColReverse,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Row
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Wrap {
    None,
    Wrap,
    WrapReverse,
}

impl Default for Wrap {
    fn default() -> Self {
        Wrap::None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Distribute {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for Distribute {
    fn default() -> Self {
        Distribute::Start
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Align {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl Default for Align {
    fn default() -> Self {
        Align::Stretch
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelfAlign {
    Auto,
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl Default for SelfAlign {
    fn default() -> Self {
        SelfAlign::Auto
    }
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Visible
    }
}
