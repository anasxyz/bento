use crate::layout::types::{
    AlignContent as BAlignContent, AlignItems as BAlignItems, AlignSelf as BAlignSelf,
    Display as BDisplay, FlexDirection as BFlexDirection, FlexWrap as BFlexWrap,
    JustifyContent as BJustifyContent, JustifyItems as BJustifyItems, JustifySelf as BJustifySelf,
    Layout, Position as BPosition, Size as BSize,
};
use taffy::{Point, prelude::*};

fn to_taffy_dimension(s: &BSize) -> Dimension {
    match s {
        BSize::Auto => Dimension::auto(),
        BSize::Px(v) => Dimension::length(*v),
        BSize::Percent(v) => Dimension::percent(*v / 100.0),
    }
}

pub fn to_taffy_style(l: &Layout) -> Style {
    Style {
        display: match l.display {
            BDisplay::Flex => Display::Flex,
            BDisplay::Block => Display::Block,
            BDisplay::None => Display::None,
        },
        position: match l.position {
            BPosition::Relative => Position::Relative,
            BPosition::Absolute => Position::Absolute,
        },
        flex_direction: match l.flex_direction {
            BFlexDirection::Row => FlexDirection::Row,
            BFlexDirection::Column => FlexDirection::Column,
            BFlexDirection::RowReverse => FlexDirection::RowReverse,
            BFlexDirection::ColumnReverse => FlexDirection::ColumnReverse,
        },
        flex_wrap: match l.flex_wrap {
            BFlexWrap::NoWrap => FlexWrap::NoWrap,
            BFlexWrap::Wrap => FlexWrap::Wrap,
            BFlexWrap::WrapReverse => FlexWrap::WrapReverse,
        },
        justify_content: l.justify_content.map(|j| match j {
            BJustifyContent::Start => JustifyContent::Start,
            BJustifyContent::End => JustifyContent::End,
            BJustifyContent::Center => JustifyContent::Center,
            BJustifyContent::SpaceBetween => JustifyContent::SpaceBetween,
            BJustifyContent::SpaceAround => JustifyContent::SpaceAround,
            BJustifyContent::SpaceEvenly => JustifyContent::SpaceEvenly,
        }),
        justify_items: l.justify_items.map(|j| match j {
            BJustifyItems::Start => JustifyItems::Start,
            BJustifyItems::End => JustifyItems::End,
            BJustifyItems::Center => JustifyItems::Center,
            BJustifyItems::Stretch => JustifyItems::Stretch,
            BJustifyItems::Baseline => JustifyItems::Baseline,
        }),
        justify_self: l.justify_self.map(|j| match j {
            BJustifySelf::Start => JustifySelf::Start,
            BJustifySelf::End => JustifySelf::End,
            BJustifySelf::Center => JustifySelf::Center,
            BJustifySelf::Stretch => JustifySelf::Stretch,
            BJustifySelf::Baseline => JustifySelf::Baseline,
        }),
        align_content: l.align_content.map(|a| match a {
            BAlignContent::Start => AlignContent::Start,
            BAlignContent::End => AlignContent::End,
            BAlignContent::Center => AlignContent::Center,
            BAlignContent::Stretch => AlignContent::Stretch,
            BAlignContent::SpaceBetween => AlignContent::SpaceBetween,
            BAlignContent::SpaceAround => AlignContent::SpaceAround,
            BAlignContent::SpaceEvenly => AlignContent::SpaceEvenly,
        }),
        align_items: l.align_items.map(|a| match a {
            BAlignItems::Start => AlignItems::Start,
            BAlignItems::End => AlignItems::End,
            BAlignItems::Center => AlignItems::Center,
            BAlignItems::Stretch => AlignItems::Stretch,
            BAlignItems::Baseline => AlignItems::Baseline,
        }),
        align_self: l.align_self.map(|a| match a {
            BAlignSelf::Start => AlignSelf::Start,
            BAlignSelf::End => AlignSelf::End,
            BAlignSelf::Center => AlignSelf::Center,
            BAlignSelf::Stretch => AlignSelf::Stretch,
            BAlignSelf::Baseline => AlignSelf::Baseline,
        }),
        flex_grow: l.flex_grow,
        flex_shrink: l.flex_shrink,
        flex_basis: to_taffy_dimension(&l.flex_basis),
        size: Size {
            width: to_taffy_dimension(&l.width),
            height: to_taffy_dimension(&l.height),
        },
        min_size: Size {
            width: to_taffy_dimension(&l.min_width),
            height: to_taffy_dimension(&l.min_height),
        },
        max_size: Size {
            width: to_taffy_dimension(&l.max_width),
            height: to_taffy_dimension(&l.max_height),
        },
        padding: Rect {
            top: LengthPercentage::length(l.padding[0]),
            right: LengthPercentage::length(l.padding[1]),
            bottom: LengthPercentage::length(l.padding[2]),
            left: LengthPercentage::length(l.padding[3]),
        },
        margin: Rect {
            top: LengthPercentageAuto::length(l.margin[0]),
            right: LengthPercentageAuto::length(l.margin[1]),
            bottom: LengthPercentageAuto::length(l.margin[2]),
            left: LengthPercentageAuto::length(l.margin[3]),
        },
        inset: Rect {
            top: LengthPercentageAuto::length(l.inset[0]),
            right: LengthPercentageAuto::length(l.inset[1]),
            bottom: LengthPercentageAuto::length(l.inset[2]),
            left: LengthPercentageAuto::length(l.inset[3]),
        },
        gap: Size {
            width: LengthPercentage::length(l.gap[1]),
            height: LengthPercentage::length(l.gap[0]),
        },
        overflow: Point {
            x: taffy::Overflow::Visible,
            y: taffy::Overflow::Visible,
        },
        ..Style::default()
    }
}

pub fn write_taffy_output(
    taffy_layout: &taffy::Layout,
    parent_x: f32,
    parent_y: f32,
    l: &mut Layout,
) {
    l.x = parent_x + taffy_layout.location.x;
    l.y = parent_y + taffy_layout.location.y;
    l.w = taffy_layout.size.width;
    l.h = taffy_layout.size.height;
}
