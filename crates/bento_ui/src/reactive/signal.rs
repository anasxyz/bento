use super::runtime;
use std::marker::PhantomData;

pub struct Signal<T> {
    id: usize,
    _phantom: PhantomData<T>,
}

// Manual impls so we don't require T: Copy/Clone on the handle itself
impl<T> Copy for Signal<T> {}
impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Clone + 'static> Signal<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            id: runtime::create_signal_id(value),
            _phantom: PhantomData,
        }
    }

    pub fn get(&self) -> T {
        runtime::get_signal_value(self.id)
    }

    pub fn set(&self, value: T) {
        runtime::set_signal_value(self.id, value);
    }

    pub fn update(&self, f: impl FnOnce(T) -> T) {
        let current = self.get();
        self.set(f(current));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reactive::state;

    #[test]
    fn signal_get_set() {
        let count = state(0i32);
        assert_eq!(count.get(), 0);
        count.set(5);
        assert_eq!(count.get(), 5);
    }

    #[test]
    fn signal_update() {
        let count = state(0i32);
        count.update(|n| n + 1);
        assert_eq!(count.get(), 1);
    }

    #[test]
    fn signal_is_copy() {
        let count = state(0i32);
        // this is a copy so no clone needed
        let also_count = count; 
        count.set(10);
        // same underlying id
        assert_eq!(also_count.get(), 10); 
    }
}
