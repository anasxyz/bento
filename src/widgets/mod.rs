use std::any::Any;

use crate::Ui;

mod button;
mod manager;

pub use button::ButtonWidget;
pub use manager::WidgetManager;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub trait Widget: Any {
    fn id(&self) -> usize;
    fn bounds(&self) -> Rect;
    fn set_bounds(&mut self, bounds: Rect);
    fn render(&self, ui: &mut Ui);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
