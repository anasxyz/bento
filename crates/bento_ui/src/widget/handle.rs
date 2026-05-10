use std::marker::PhantomData;

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

impl<T> Copy for WidgetHandle<T> {}
impl<T> Clone for WidgetHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}
