use std::cell::RefCell;
use std::collections::HashMap;

/// Stores the actual value of every signal.
/// Keyed by a plain usize ID.
struct SignalStore {
    values: Vec<Box<dyn std::any::Any>>,
}

impl SignalStore {
    fn new() -> Self {
        Self { values: Vec::new() }
    }

    fn insert(&mut self, value: Box<dyn std::any::Any>) -> usize {
        let id = self.values.len();
        self.values.push(value);
        id
    }

    fn get<T: 'static>(&self, id: usize) -> &T {
        self.values[id]
            .downcast_ref::<T>()
            .expect("signal type mismatch")
    }

    fn set<T: 'static>(&mut self, id: usize, value: T) {
        self.values[id] = Box::new(value);
    }
}

thread_local! {
    static RUNTIME: RefCell<SignalStore> = RefCell::new(SignalStore::new());
}

pub(crate) fn create_signal_id<T: 'static>(value: T) -> usize {
    RUNTIME.with(|rt| rt.borrow_mut().insert(Box::new(value)))
}

pub(crate) fn get_signal_value<T: Clone + 'static>(id: usize) -> T {
    RUNTIME.with(|rt| rt.borrow().get::<T>(id).clone())
}

pub(crate) fn set_signal_value<T: 'static>(id: usize, value: T) {
    RUNTIME.with(|rt| rt.borrow_mut().set(id, value));
}
