use std::marker::PhantomData;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct SignalId(pub(crate) usize);

pub struct Signal<T> {
    pub(crate) id: SignalId,
    pub(crate) _phantom: PhantomData<T>,
}

impl<T> Copy for Signal<T> {}
impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        *self
    }
}
