use super::signal::Signal;
use super::{owner, runtime};
use std::rc::Rc;

pub struct Derived<T: Clone + 'static> {
    signal: Signal<T>,
    id: runtime::SubscriberId,
}

impl<T: Clone + 'static> Derived<T> {
    pub fn new(f: impl Fn() -> T + 'static) -> Self {
        let id = runtime::create_subscriber(Rc::new(|| {}));

        runtime::push_observer(id);
        let initial = f();
        runtime::pop_observer();

        let signal = Signal::new(initial);
        let signal_copy = signal;

        let notify = Rc::new(move || {
            runtime::clear_subscriptions(id);
            runtime::push_observer(id);
            let new_val = f();
            runtime::pop_observer();
            signal_copy.set(new_val); 
        });

        runtime::update_subscriber(id, notify);

        owner::register_cleanup(move || {
            runtime::clear_subscriptions(id);
            runtime::remove_subscriber(id);
        });

        Self { signal, id }
    }

    pub fn get(&self) -> T {
        self.signal.get()
    }
}

impl<T: Clone + 'static> Drop for Derived<T> {
    fn drop(&mut self) {
        runtime::clear_subscriptions(self.id);
        runtime::remove_subscriber(self.id);
    }
}

pub fn derived<T: Clone + 'static>(f: impl Fn() -> T + 'static) -> Derived<T> {
    Derived::new(f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reactive::{owner::Owner, state};

    #[test]
    fn derived_initial_value() {
        let owner = Owner::new();
        let count = state(2);
        let doubled = derived(move || count.get() * 2);
        let _owner = owner.collect();
        assert_eq!(doubled.get(), 4);
    }

    #[test]
    fn derived_updates_when_signal_changes() {
        let owner = Owner::new();
        let count = state(2);
        let doubled = derived(move || count.get() * 2);
        let _owner = owner.collect();
        count.set(5);
        assert_eq!(doubled.get(), 10);
    }

    #[test]
    fn derived_chain() {
        let owner = Owner::new();
        let count = state(1);
        let doubled = derived(move || count.get() * 2);
        let quadrupled = derived(move || doubled.get() * 2);
        let _owner = owner.collect();
        count.set(3);
        assert_eq!(quadrupled.get(), 12);
    }

    #[test]
    fn derived_cleaned_up_when_owner_drops() {
        let count = state(0);
        {
            let owner = Owner::new();
            let _d = derived(move || count.get() * 2);
            let _owner = owner.collect();
            count.set(1);
            assert!(runtime::subscriber_count(count.id.0) > 0);
        } // owner drops
        assert_eq!(runtime::subscriber_count(count.id.0), 0);
    }

    #[test]
    fn nested_derived_cleanup() {
        let count = state(0);
        let outer_owner = Owner::new();
        let outer_d = derived(move || count.get() * 2);
        {
            let inner_owner = Owner::new();
            let inner_d = derived(move || count.get() * 3);
            let _inner_owner = inner_owner.collect();
            count.set(1);
            assert_eq!(outer_d.get(), 2);
            assert_eq!(inner_d.get(), 3);
        } // inner drops
        count.set(2);
        assert_eq!(outer_d.get(), 4); // outer still works
        let _outer_owner = outer_owner.collect();
    }
}
