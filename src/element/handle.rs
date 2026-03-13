use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct Handle<T> {
    pub id: u32,
    pub generation: u32,
    _marker: PhantomData<*const T>,
}

impl<T> Handle<T> {
    pub fn new(id: u32, generation: u32) -> Self {
        Self {
            id,
            generation,
            _marker: PhantomData,
        }
    }
}

impl<T> Copy for Handle<T> {}
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}
