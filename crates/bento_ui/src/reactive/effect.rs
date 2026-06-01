use super::runtime::{self, SubscriberId};
use crate::reactive::owner;
use std::rc::Rc;

pub struct Effect {
    id: SubscriberId,
}

impl Effect {
    pub fn new(f: impl Fn() + 'static) -> Self {
        let id = runtime::next_subscriber_id();

        runtime::push_observer(id);
        f();
        runtime::pop_observer();

        runtime::register_subscriber(
            id,
            usize::MAX,
            Rc::new(move || {
                runtime::clear_subscriptions(id);
                runtime::push_observer(id);
                f();
                runtime::pop_observer();
            }),
        );

        owner::register_cleanup(move || runtime::unregister_subscriber(id));

        Self { id }
    }
}

impl Drop for Effect {
    fn drop(&mut self) {
        runtime::unregister_subscriber(self.id);
    }
}

#[cfg(test)]
mod tests {
    use crate::reactive::{effect, state};
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn effect_runs_immediately() {
        let ran = Rc::new(Cell::new(false));
        let ran_clone = ran.clone();
        let _eff = effect(move || {
            ran_clone.set(true);
        });
        assert!(ran.get());
    }

    #[test]
    fn effect_reruns_on_change() {
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

    #[test]
    fn effect_drops_cleanly() {
        let count = state(0i32);
        let ran = Rc::new(Cell::new(0));
        let ran_clone = ran.clone();
        {
            let _eff = effect(move || {
                let _ = count.get();
                ran_clone.set(ran_clone.get() + 1);
            });
            count.set(1);
            assert_eq!(ran.get(), 2);
        } // _eff dropped here
        count.set(2);
        assert_eq!(ran.get(), 2); // should not have run again
    }
}
