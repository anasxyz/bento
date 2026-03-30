use super::values::*;

#[derive(Debug, Clone)]
pub struct Layout {
    // size
    pub width: Size,
    pub height: Size,
    pub min_w: Size,
    pub max_w: Size,
    pub min_h: Size,
    pub max_h: Size,

    // flex
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Size,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub justify_content: JustifyContent,

    // spacing
    pub padding: [f32; 4], // top, right, bottom, left
    pub margin: [f32; 4],
    pub row_gap: f32,
    pub col_gap: f32,

    // positioning
    pub position: Position,
    pub inset: [Size; 4], // top, right, bottom, left
    pub overflow: Overflow,
    pub aspect_ratio: Option<f32>,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            width: Size::Auto,
            height: Size::Auto,
            min_w: Size::Auto,
            max_w: Size::Auto,
            min_h: Size::Auto,
            max_h: Size::Auto,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Size::Auto,
            align_items: AlignItems::Stretch,
            align_self: AlignSelf::Auto,
            justify_content: JustifyContent::Start,
            padding: [0.0; 4],
            margin: [0.0; 4],
            row_gap: 0.0,
            col_gap: 0.0,
            position: Position::Relative,
            inset: [Size::Auto, Size::Auto, Size::Auto, Size::Auto],
            overflow: Overflow::Visible,
            aspect_ratio: None,
        }
    }
}
