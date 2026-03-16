use crate::Color;
use crate::element::container::Container;
use crate::element::label::Label;
use crate::element::layout::Layout;
use crate::element::rect::Rect;
use crate::element::values::*;
use crate::fonts::Fonts;
use crate::keyboard::{Key, Modifiers};
use crate::mouse::MouseButton;

pub enum AnyElement {
    Rect(Rect),
    Label(Label),
    Container(Container),
}

impl AnyElement {
    // --- dirty ---
    pub(crate) fn is_dirty(&self) -> bool {
        match self {
            AnyElement::Rect(e) => e.dirty,
            AnyElement::Label(e) => e.dirty,
            AnyElement::Container(e) => e.dirty,
        }
    }

    pub(crate) fn clear_dirty(&mut self) {
        match self {
            AnyElement::Rect(e) => e.dirty = false,
            AnyElement::Label(e) => e.dirty = false,
            AnyElement::Container(e) => e.dirty = false,
        }
    }

    pub(crate) fn mark_dirty(&mut self) {
        match self {
            AnyElement::Rect(e) => e.dirty = true,
            AnyElement::Label(e) => e.dirty = true,
            AnyElement::Container(e) => e.dirty = true,
        }
    }

    // --- layout ---
    pub fn layout(&self) -> &Layout {
        match self {
            AnyElement::Rect(e) => &e.layout,
            AnyElement::Label(e) => &e.layout,
            AnyElement::Container(e) => &e.layout,
        }
    }

    // internal
    // does not mark dirty, used by layout system
    pub(crate) fn layout_mut_internal(&mut self) -> &mut Layout {
        match self {
            AnyElement::Rect(e) => &mut e.layout,
            AnyElement::Label(e) => &mut e.layout,
            AnyElement::Container(e) => &mut e.layout,
        }
    }

    // public
    // marks dirty, used by setters below
    fn layout_mut(&mut self) -> &mut Layout {
        self.mark_dirty();
        match self {
            AnyElement::Rect(e) => &mut e.layout,
            AnyElement::Label(e) => &mut e.layout,
            AnyElement::Container(e) => &mut e.layout,
        }
    }

    // --- layout getters ---
    pub fn width(&self) -> &Size {
        &self.layout().width
    }
    pub fn height(&self) -> &Size {
        &self.layout().height
    }
    pub fn min_w(&self) -> &Size {
        &self.layout().min_w
    }
    pub fn max_w(&self) -> &Size {
        &self.layout().max_w
    }
    pub fn min_h(&self) -> &Size {
        &self.layout().min_h
    }
    pub fn max_h(&self) -> &Size {
        &self.layout().max_h
    }
    pub fn padding(&self) -> [f32; 4] {
        self.layout().padding
    }
    pub fn margin(&self) -> [f32; 4] {
        self.layout().margin
    }
    pub fn row_gap(&self) -> f32 {
        self.layout().row_gap
    }
    pub fn col_gap(&self) -> f32 {
        self.layout().col_gap
    }
    pub fn flex_direction(&self) -> &FlexDirection {
        &self.layout().flex_direction
    }
    pub fn flex_wrap(&self) -> &FlexWrap {
        &self.layout().flex_wrap
    }
    pub fn flex_grow(&self) -> f32 {
        self.layout().flex_grow
    }
    pub fn flex_shrink(&self) -> f32 {
        self.layout().flex_shrink
    }
    pub fn flex_basis(&self) -> &Size {
        &self.layout().flex_basis
    }
    pub fn align_items(&self) -> &AlignItems {
        &self.layout().align_items
    }
    pub fn align_self_val(&self) -> &AlignSelf {
        &self.layout().align_self
    }
    pub fn justify_content(&self) -> &JustifyContent {
        &self.layout().justify_content
    }
    pub fn position(&self) -> &Position {
        &self.layout().position
    }
    pub fn inset(&self) -> &[Size; 4] {
        &self.layout().inset
    }
    pub fn overflow_x(&self) -> &Overflow {
        &self.layout().overflow_x
    }
    pub fn overflow_y(&self) -> &Overflow {
        &self.layout().overflow_y
    }
    pub fn z_index(&self) -> i32 {
        self.layout().z_index
    }
    pub fn opacity(&self) -> f32 {
        self.layout().opacity
    }
    pub fn visible(&self) -> bool {
        self.layout().visible
    }

    // --- layout setters ---
    pub fn set_width(&mut self, v: Size) -> &mut Self {
        self.layout_mut().width = v;
        self
    }
    pub fn set_height(&mut self, v: Size) -> &mut Self {
        self.layout_mut().height = v;
        self
    }
    pub fn set_min_w(&mut self, v: Size) -> &mut Self {
        self.layout_mut().min_w = v;
        self
    }
    pub fn set_max_w(&mut self, v: Size) -> &mut Self {
        self.layout_mut().max_w = v;
        self
    }
    pub fn set_min_h(&mut self, v: Size) -> &mut Self {
        self.layout_mut().min_h = v;
        self
    }
    pub fn set_max_h(&mut self, v: Size) -> &mut Self {
        self.layout_mut().max_h = v;
        self
    }
    pub fn set_padding(&mut self, v: [f32; 4]) -> &mut Self {
        self.layout_mut().padding = v;
        self
    }
    pub fn set_margin(&mut self, v: [f32; 4]) -> &mut Self {
        self.layout_mut().margin = v;
        self
    }
    pub fn set_row_gap(&mut self, v: f32) -> &mut Self {
        self.layout_mut().row_gap = v;
        self
    }
    pub fn set_col_gap(&mut self, v: f32) -> &mut Self {
        self.layout_mut().col_gap = v;
        self
    }
    pub fn set_flex_direction(&mut self, v: FlexDirection) -> &mut Self {
        self.layout_mut().flex_direction = v;
        self
    }
    pub fn set_flex_wrap(&mut self, v: FlexWrap) -> &mut Self {
        self.layout_mut().flex_wrap = v;
        self
    }
    pub fn set_flex_grow(&mut self, v: f32) -> &mut Self {
        self.layout_mut().flex_grow = v;
        self
    }
    pub fn set_flex_shrink(&mut self, v: f32) -> &mut Self {
        self.layout_mut().flex_shrink = v;
        self
    }
    pub fn set_flex_basis(&mut self, v: Size) -> &mut Self {
        self.layout_mut().flex_basis = v;
        self
    }
    pub fn set_align_items(&mut self, v: AlignItems) -> &mut Self {
        self.layout_mut().align_items = v;
        self
    }
    pub fn set_align_self(&mut self, v: AlignSelf) -> &mut Self {
        self.layout_mut().align_self = v;
        self
    }
    pub fn set_justify_content(&mut self, v: JustifyContent) -> &mut Self {
        self.layout_mut().justify_content = v;
        self
    }
    pub fn set_position(&mut self, v: Position) -> &mut Self {
        self.layout_mut().position = v;
        self
    }
    pub fn set_inset(&mut self, v: [Size; 4]) -> &mut Self {
        self.layout_mut().inset = v;
        self
    }
    pub fn set_overflow_x(&mut self, v: Overflow) -> &mut Self {
        self.layout_mut().overflow_x = v;
        self
    }
    pub fn set_overflow_y(&mut self, v: Overflow) -> &mut Self {
        self.layout_mut().overflow_y = v;
        self
    }
    pub fn set_z_index(&mut self, v: i32) -> &mut Self {
        self.layout_mut().z_index = v;
        self
    }
    pub fn set_opacity(&mut self, v: f32) -> &mut Self {
        self.layout_mut().opacity = v;
        self
    }
    pub fn set_visible(&mut self, v: bool) -> &mut Self {
        self.layout_mut().visible = v;
        self
    }

    // element specific setters delegated to inner types
    pub fn set_bg_color(&mut self, color: impl Into<Option<Color>>) -> &mut Self {
        let color = color.into();
        match self {
            AnyElement::Rect(e) => {
                e.set_bg_color(color.unwrap_or(Color::BLACK));
            }
            AnyElement::Container(e) => {
                e.set_bg_color(color);
            }
            _ => {}
        }
        self
    }
    pub fn set_border_radius(&mut self, radius: Option<f32>) -> &mut Self {
        match self {
            AnyElement::Rect(e) => {
                e.set_border_radius(radius);
            }
            AnyElement::Container(e) => {
                e.set_border_radius(radius);
            }
            _ => {}
        }
        self
    }
    pub fn set_border_color(&mut self, color: Option<Color>) -> &mut Self {
        match self {
            AnyElement::Rect(e) => {
                e.set_border_color(color);
            }
            AnyElement::Container(e) => {
                e.set_border_color(color);
            }
            _ => {}
        }
        self
    }
    pub fn set_border_thickness(&mut self, thickness: f32) -> &mut Self {
        match self {
            AnyElement::Rect(e) => {
                e.set_border_thickness(thickness);
            }
            AnyElement::Container(e) => {
                e.set_border_thickness(thickness);
            }
            _ => {}
        }
        self
    }
    pub fn set_text(&mut self, text: &str) -> &mut Self {
        if let AnyElement::Label(e) = self {
            e.set_text(text);
        }
        self
    }
    pub fn set_font_size(&mut self, size: f32) -> &mut Self {
        if let AnyElement::Label(e) = self {
            e.set_font_size(size);
        }
        self
    }
    pub fn set_font_weight(&mut self, weight: u16) -> &mut Self {
        if let AnyElement::Label(e) = self {
            e.set_font_weight(weight);
        }
        self
    }
    pub fn set_font_italic(&mut self, italic: bool) -> &mut Self {
        if let AnyElement::Label(e) = self {
            e.set_font_italic(italic);
        }
        self
    }
    pub fn set_text_color(&mut self, color: Color) -> &mut Self {
        if let AnyElement::Label(e) = self {
            e.set_text_color(color);
        }
        self
    }
    pub fn set_font_family(&mut self, family: &str) -> &mut Self {
        if let AnyElement::Label(e) = self {
            e.set_font_family(family);
        }
        self
    }

    // --- measure ---
    pub(crate) fn measure(&self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)> {
        match self {
            AnyElement::Label(e) => e.measure(fonts, max_width),
            _ => None,
        }
    }

    pub(crate) fn has_measure(&self) -> bool {
        matches!(self, AnyElement::Label(_))
    }

    // --- internal event handlers ---
    pub(crate) fn on_mouse_press(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    pub(crate) fn on_mouse_release(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    pub(crate) fn on_mouse_click(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    pub(crate) fn on_mouse_double_click(&mut self, _x: f32, _y: f32, _button: MouseButton) {}
    pub(crate) fn on_mouse_enter(&mut self) {}
    pub(crate) fn on_mouse_leave(&mut self) {}
    pub(crate) fn on_key_press(&mut self, _key: Key, _modifiers: Modifiers, _text: Option<char>) {}
    pub(crate) fn on_key_release(&mut self, _key: Key, _modifiers: Modifiers) {}
    pub(crate) fn on_focus_gained(&mut self) {
        match self {
            AnyElement::Rect(e) => e.on_focus_gained(),
            _ => {}
        }
    }
    pub(crate) fn on_focus_lost(&mut self) {
        match self {
            AnyElement::Rect(e) => e.on_focus_lost(),
            _ => {}
        }
    }
}

impl From<Rect> for AnyElement {
    fn from(e: Rect) -> Self {
        AnyElement::Rect(e)
    }
}
impl From<Label> for AnyElement {
    fn from(e: Label) -> Self {
        AnyElement::Label(e)
    }
}
impl From<Container> for AnyElement {
    fn from(e: Container) -> Self {
        AnyElement::Container(e)
    }
}
