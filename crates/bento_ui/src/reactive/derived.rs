use super::runtime::{self, SubscriberId};
use super::signal::Signal;
use crate::reactive::owner;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct Derived<T> {
    signal: Signal<T>,
    id: SubscriberId,
    _phantom: PhantomData<T>,
}

impl<T: Clone + 'static> Clone for Derived<T> {
    fn clone(&self) -> Self {
        Self {
            signal: self.signal,
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl<T: Clone + 'static> Derived<T> {
    pub fn new(f: impl Fn() -> T + 'static) -> Self {
        let id = runtime::next_subscriber_id();

        runtime::push_observer(id);
        let initial = f();
        runtime::pop_observer();

        let signal = Signal::new(initial);
        let signal_copy = signal;

        let rank = runtime::max_dependency_rank(id) + 1;

        let notify = Rc::new(move || {
            runtime::clear_subscriptions(id);
            runtime::push_observer(id);
            let new_val = f();
            runtime::pop_observer();
            signal_copy.set(new_val);
        });

        runtime::register_subscriber(id, rank, notify);

        owner::register_cleanup(move || runtime::unregister_subscriber(id));

        Self {
            signal,
            id,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self) -> T {
        self.signal.get()
    }
}

impl<T> Drop for Derived<T> {
    fn drop(&mut self) {
        runtime::unregister_subscriber(self.id);
    }
}

#[cfg(test)]
mod tests {
    use crate::reactive::{derived, effect, state};
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn derived_initial_value() {
        let count = state(2i32);
        let doubled = derived(move || count.get() * 2);
        assert_eq!(doubled.get(), 4);
    }

    #[test]
    fn derived_updates_when_signal_changes() {
        let count = state(2i32);
        let doubled = derived(move || count.get() * 2);
        count.set(5);
        assert_eq!(doubled.get(), 10);
    }

    #[test]
    fn derived_chain() {
        let count = state(1i32);
        let doubled = derived(move || count.get() * 2);
        let quadrupled = derived(move || doubled.get() * 2);
        count.set(3);
        assert_eq!(quadrupled.get(), 12);
    }

    #[test]
    fn effect_runs_on_change() {
        let count = state(0i32);
        let ran = Rc::new(Cell::new(0));
        let ran_clone = ran.clone();

        let _eff = effect(move || {
            let _ = count.get();
            ran_clone.set(ran_clone.get() + 1);
        });

        assert_eq!(ran.get(), 1);
        count.set(1);
        assert_eq!(ran.get(), 2);
        count.set(2);
        assert_eq!(ran.get(), 3);
    }
}
