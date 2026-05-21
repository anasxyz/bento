use std::marker::PhantomData;

use crate::widget::Widget;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct WidgetHandle<T> {
    pub id: u32,
    pub generation: u32,
    _marker: PhantomData<fn() -> T>,
}

impl<T> WidgetHandle<T> {
    pub fn new(id: u32, generation: u32) -> Self {
        Self {
            id,
            generation,
            _marker: PhantomData,
        }
    }

    pub fn untyped(&self) -> WidgetHandle<()> {
        WidgetHandle::new(self.id, self.generation)
    }
}

impl<T> Default for WidgetHandle<T> {
    fn default() -> Self {
        Self {
            id: 0,
            generation: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Copy for WidgetHandle<T> {}
impl<T> Clone for WidgetHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}
