use taffy::prelude::*;
use taffy::{Overflow, Point};

#[derive(Clone, Debug)]
pub struct LayoutProps {
    pub display: Display,
    pub width: Dimension,
    pub height: Dimension,
    pub min_width: Dimension,
    pub min_height: Dimension,
    pub max_width: Dimension,
    pub max_height: Dimension,
    pub aspect_ratio: Option<f32>,
    pub position: Position,
    pub inset: Rect<LengthPercentageAuto>,
    pub margin: Rect<LengthPercentageAuto>,
    pub padding: Rect<LengthPercentage>,
    pub border: Rect<LengthPercentage>,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,
    pub justify_content: Option<JustifyContent>,
    pub justify_self: Option<AlignSelf>,
    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignSelf>,
    pub align_content: Option<AlignContent>,
    pub gap: Size<LengthPercentage>,
    pub grid_auto_rows: Vec<TrackSizingFunction>,
    pub grid_auto_columns: Vec<TrackSizingFunction>,
    pub grid_auto_flow: GridAutoFlow,
    pub grid_row: Line<GridPlacement>,
    pub grid_column: Line<GridPlacement>,
    pub overflow: Point<Overflow>,
    pub scrollbar_width: f32,
}

impl Default for LayoutProps {
    fn default() -> Self {
        Self {
            display: Display::Flex,
            width: Dimension::AUTO,
            height: Dimension::AUTO,
            min_width: Dimension::AUTO,
            min_height: Dimension::AUTO,
            max_width: Dimension::AUTO,
            max_height: Dimension::AUTO,
            aspect_ratio: None,
            position: Position::Relative,
            inset: Rect::auto(),
            margin: Rect::zero(),
            padding: Rect::zero(),
            border: Rect::zero(),
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Dimension::AUTO,
            justify_content: None,
            justify_self: None,
            align_items: None,
            align_self: None,
            align_content: None,
            gap: Size::zero(),
            grid_auto_rows: Vec::new(),
            grid_auto_columns: Vec::new(),
            grid_auto_flow: GridAutoFlow::Row,
            grid_row: Line::auto(),
            grid_column: Line::auto(),
            overflow: Point::default(),
            scrollbar_width: 0.0,
        }
    }
}

impl LayoutProps {
    pub fn to_taffy_style(&self) -> Style {
        Style {
            display: self.display,
            size: Size {
                width: self.width,
                height: self.height,
            },
            min_size: Size {
                width: self.min_width,
                height: self.min_height,
            },
            max_size: Size {
                width: self.max_width,
                height: self.max_height,
            },
            aspect_ratio: self.aspect_ratio,
            position: self.position,
            inset: self.inset,
            margin: self.margin,
            padding: self.padding,
            border: self.border,
            flex_direction: self.flex_direction,
            flex_wrap: self.flex_wrap,
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            flex_basis: self.flex_basis,
            justify_content: self.justify_content,
            justify_self: self.justify_self,
            align_items: self.align_items,
            align_self: self.align_self,
            align_content: self.align_content,
            gap: self.gap,
            grid_auto_rows: self.grid_auto_rows.clone(),
            grid_auto_columns: self.grid_auto_columns.clone(),
            grid_auto_flow: self.grid_auto_flow,
            grid_row: self.grid_row.clone(),
            grid_column: self.grid_column.clone(),
            overflow: self.overflow,
            scrollbar_width: self.scrollbar_width,
            ..Default::default()
        }
    }
}

pub fn px(value: f32) -> Dimension { Dimension::from_length(value) }
pub fn pct(value: f32) -> Dimension { Dimension::from_percent(value / 100.0) }
pub fn fill() -> Dimension { Dimension::from_percent(1.0) }
pub fn auto() -> Dimension { Dimension::AUTO }
pub fn row() -> FlexDirection { FlexDirection::Row }
pub fn col() -> FlexDirection { FlexDirection::Column }
