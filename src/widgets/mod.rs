use std::any::Any;
use std::marker::PhantomData;
use crate::{Drawer, MouseState};

mod button;
mod slider;
mod text_input;
mod manager;

pub use button::ButtonWidget;
pub use slider::SliderWidget;
pub use text_input::TextInputWidget;
pub use manager::WidgetManager;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
}

impl Default for Rect {
    fn default() -> Self { Self { x: 0.0, y: 0.0, w: 100.0, h: 100.0 } }
}

pub trait Widget: Any {
    fn id(&self) -> usize;
    fn bounds(&self) -> Rect;
    fn set_bounds(&mut self, bounds: Rect);
    fn update(&mut self, mouse: &MouseState);
    /// Measure and compute bounds. Called once per frame before render, no draw calls.
    fn layout(&mut self, fonts: &mut crate::Fonts) {}
    fn render(&mut self, drawer: &mut Drawer);
    /// Returns true if this widget's visual state changed since the last clear_dirty.
    fn is_dirty(&self) -> bool;
    fn clear_dirty(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
pub struct WidgetHandle<T: Widget> {
    pub(crate) id: usize,
    _marker: PhantomData<T>,
}

impl<T: Widget> Copy for WidgetHandle<T> {}
impl<T: Widget> Clone for WidgetHandle<T> {
    fn clone(&self) -> Self { *self }
}

impl<T: Widget> WidgetHandle<T> {
    pub(crate) fn new(id: usize) -> Self {
        Self { id, _marker: PhantomData }
    }
}
