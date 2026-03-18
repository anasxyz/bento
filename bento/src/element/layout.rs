use crate::element::values::*;

#[derive(Debug, Clone)]
pub struct Layout {
    // computed by layout system
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub prev_x: f32,
    pub prev_y: f32,
    pub prev_w: f32,
    pub prev_h: f32,

    // size
    pub width: Size,
    pub height: Size,
    pub min_w: Size,
    pub min_h: Size,
    pub max_w: Size,
    pub max_h: Size,
    pub aspect_ratio: Option<f32>,

    // spacing
    pub padding: [f32; 4], // [top, right, bottom, left]
    pub margin: [f32; 4],
    pub row_gap: f32,
    pub col_gap: f32,

    // flex
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Size,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub justify_content: JustifyContent,

    // overflow + position
    pub overflow_x: Overflow,
    pub overflow_y: Overflow,
    pub position: Position,
    pub inset: [Size; 4],

    // visual
    pub opacity: f32,
    pub z_index: i32,
    pub visible: bool,

    // transform
    // applied during rendering only, never touches taffy
    // can be used by scroll containers, animations, drag, etc but have to be careful between layout
    // altering animations and non layout altering animations
    pub transform: Option<(f32, f32)>,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            prev_x: 0.0,
            prev_y: 0.0,
            prev_w: 0.0,
            prev_h: 0.0,

            width: Size::Auto,
            height: Size::Auto,
            min_w: Size::Fixed(0.0),
            min_h: Size::Fixed(0.0),
            max_w: Size::Auto,
            max_h: Size::Auto,
            aspect_ratio: None,

            padding: [0.0; 4],
            margin: [0.0; 4],
            row_gap: 0.0,
            col_gap: 0.0,

            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Size::Auto,
            align_items: AlignItems::Start,
            align_self: AlignSelf::Auto,
            justify_content: JustifyContent::Start,

            overflow_x: Overflow::Hidden,
            overflow_y: Overflow::Hidden,
            position: Position::Relative,
            inset: [Size::Auto, Size::Auto, Size::Auto, Size::Auto],

            opacity: 1.0,
            z_index: 0,
            visible: true,

            transform: None,
        }
    }
}
