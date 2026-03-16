use crate::element::container::Container;
use crate::element::label::Label;
use crate::element::layout::Layout;
use crate::element::rect::Rect;
use crate::fonts::Fonts;
use crate::keyboard::{Key, Modifiers};
use crate::mouse::MouseButton;

pub enum AnyElement {
    Rect(Rect),
    Label(Label),
    Container(Container),
}

impl AnyElement {
    pub fn layout(&self) -> &Layout {
        match self {
            AnyElement::Rect(e) => &e.layout,
            AnyElement::Label(e) => &e.layout,
            AnyElement::Container(e) => &e.layout,
        }
    }

    pub fn layout_mut(&mut self) -> &mut Layout {
        match self {
            AnyElement::Rect(e) => &mut e.layout,
            AnyElement::Label(e) => &mut e.layout,
            AnyElement::Container(e) => &mut e.layout,
        }
    }

    pub fn measure(&self, fonts: &mut Fonts, max_width: Option<f32>) -> Option<(f32, f32)> {
        match self {
            AnyElement::Label(e) => e.measure(fonts, max_width),
            _ => None,
        }
    }

    pub fn has_measure(&self) -> bool {
        matches!(
            self,
            AnyElement::Label(_)
        )
    }

    pub fn on_mouse_press(&mut self, x: f32, y: f32, button: MouseButton) -> Option<u32> {
        match self {
            _ => None,
        }
    }

    pub fn on_mouse_release(&mut self, x: f32, y: f32, button: MouseButton) -> Option<u32> {
        match self {
            _ => None,
        }
    }

    pub fn on_mouse_click(&mut self, x: f32, y: f32, button: MouseButton) -> Option<u32> {
        match self {
            _ => None,
        }
    }

    pub fn on_mouse_double_click(&mut self, x: f32, y: f32, button: MouseButton) -> Option<u32> {
        match self {
            _ => None,
        }
    }

    pub fn on_mouse_enter(&mut self) -> Option<u32> {
        match self {
            _ => None,
        }
    }

    pub fn on_mouse_leave(&mut self) -> Option<u32> {
        match self {
            _ => None,
        }
    }

    pub fn on_key_press(
        &mut self,
        key: Key,
        modifiers: Modifiers,
        text: Option<char>,
    ) -> Option<u32> {
        match self {
            _ => None,
        }
    }

    pub fn on_key_release(&mut self, key: Key, modifiers: Modifiers) -> Option<u32> {
        None
    }

    pub fn on_focus_gained(&mut self) -> Option<u32> {
        match self {
            AnyElement::Rect(e) => e.on_focus_gained(),
            AnyElement::Container(e) => e.on_focus_gained(),
            _ => None,
        }
    }

    pub fn on_focus_lost(&mut self) -> Option<u32> {
        match self {
            AnyElement::Rect(e) => e.on_focus_lost(),
            AnyElement::Container(e) => e.on_focus_lost(),
            _ => None,
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
