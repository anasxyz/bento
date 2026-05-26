use std::marker::PhantomData;

use crate::widget::Widget;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct WidgetHandle<T> {
    pub id: usize,
    pub generation: usize,
    _marker: PhantomData<fn() -> T>,
}

impl<T> WidgetHandle<T> {
    pub fn new(id: usize, generation: usize) -> Self {
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

impl<W> WidgetHandle<W> {
    pub fn from_id(id: usize) -> Self {
        Self {
            id,
            generation: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl WidgetHandle<()> {
    pub fn typed<W>(self) -> WidgetHandle<W> {
        WidgetHandle::from_id(self.id)
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
