use crate::reactive::runtime;
use std::marker::PhantomData;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct SignalId(pub(crate) usize);

pub struct Signal<T> {
    pub(crate) id: SignalId,
    pub(crate) _phantom: PhantomData<T>,
}

// no clone impl for runtime::create_signal
impl<T: 'static> Signal<T> {
    pub fn new(value: T) -> Self {
        runtime::create_signal(value)
    }
}

// clone impl for runtime::get_signal
impl<T: Clone + 'static> Signal<T> {
    pub fn get(&self) -> T {
        runtime::get_signal(*self)
    }

    pub fn set(&self, value: T) {
        runtime::set_signal(*self, value);
    }
}

pub fn state<T: 'static>(value: T) -> Signal<T> {
    Signal::new(value)
}

impl<T> Copy for Signal<T> {}
impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        *self
    }
}
