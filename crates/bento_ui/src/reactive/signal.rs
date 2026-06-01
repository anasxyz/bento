use super::runtime;
use std::marker::PhantomData;
use crate::reactive::owner;

pub struct Signal<T> {
    pub(crate) id: usize,
    _phantom: PhantomData<T>,
}

impl<T> Copy for Signal<T> {}
impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Clone + 'static> Signal<T> {
    pub(crate) fn new(value: T) -> Self {
        let id = runtime::create_signal_id(value);
        owner::register_cleanup(move || runtime::drop_signal(id));
        Self {
            id,
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
    use crate::reactive::{runtime, state};
    use std::cell::Cell;
    use std::rc::Rc;

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
        let also_count = count;
        count.set(10);
        assert_eq!(also_count.get(), 10);
    }

    #[test]
    fn signal_notifies_subscriber() {
        let count = state(0i32);
        let notified = Rc::new(Cell::new(false));
        let notified_clone = notified.clone();

        let sub_id = runtime::next_subscriber_id();
        runtime::register_subscriber(
            sub_id,
            0,
            Rc::new(move || {
                notified_clone.set(true);
            }),
        );

        runtime::push_observer(sub_id);
        let _ = count.get();
        runtime::pop_observer();

        count.set(1);
        assert!(notified.get());
    }
}
