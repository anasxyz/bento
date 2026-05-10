#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Display {
    Flex,
    Block,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    Relative,
    Absolute,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlignSelf {
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlignContent {
    Start,
    End,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JustifyItems {
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JustifySelf {
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Size {
    Auto,
    Px(f32),
    Percent(f32),
}

impl Size {
    pub fn to_px(&self) -> Option<f32> {
        match self {
            Size::Px(v) => Some(*v),
            _ => None,
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size::Auto
    }
}

#[derive(Clone)]
pub struct Layout {
    pub display: Display,
    pub position: Position,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: Option<JustifyContent>,
    pub justify_items: Option<JustifyItems>,
    pub justify_self: Option<JustifySelf>,
    pub align_content: Option<AlignContent>,
    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignSelf>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Size,
    pub width: Size,
    pub height: Size,
    pub min_width: Size,
    pub min_height: Size,
    pub max_width: Size,
    pub max_height: Size,
    pub padding: [f32; 4], // top right bottom left
    pub margin: [f32; 4],
    pub inset: [f32; 4], // top right bottom left
    pub gap: [f32; 2],   // row col
    pub overflow_x: Overflow,
    pub overflow_y: Overflow,

    // computed outputs
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Layout {
    pub fn inputs_equal(&self, other: &Layout) -> bool {
        self.display == other.display
            && self.position == other.position
            && self.flex_direction == other.flex_direction
            && self.flex_wrap == other.flex_wrap
            && self.justify_content == other.justify_content
            && self.justify_items == other.justify_items
            && self.justify_self == other.justify_self
            && self.align_content == other.align_content
            && self.align_items == other.align_items
            && self.align_self == other.align_self
            && self.flex_grow == other.flex_grow
            && self.flex_shrink == other.flex_shrink
            && self.flex_basis == other.flex_basis
            && self.width == other.width
            && self.height == other.height
            && self.min_width == other.min_width
            && self.min_height == other.min_height
            && self.max_width == other.max_width
            && self.max_height == other.max_height
            && self.padding == other.padding
            && self.margin == other.margin
            && self.inset == other.inset
            && self.gap == other.gap
            && self.overflow_x == other.overflow_x
            && self.overflow_y == other.overflow_y
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            display: Display::Flex,
            position: Position::Relative,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: None,
            justify_items: None,
            justify_self: None,
            align_content: None,
            align_items: Some(AlignItems::Start),
            align_self: None,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Size::Auto,
            width: Size::Auto,
            height: Size::Auto,
            min_width: Size::Auto,
            min_height: Size::Auto,
            max_width: Size::Auto,
            max_height: Size::Auto,
            padding: [0.0; 4],
            margin: [0.0; 4],
            inset: [0.0; 4],
            gap: [0.0; 2],
            overflow_x: Overflow::Visible,
            overflow_y: Overflow::Visible,
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
        }
    }
}
