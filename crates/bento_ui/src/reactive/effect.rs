use super::{owner, runtime};
use std::rc::Rc;

pub(crate) struct Effect {
    id: runtime::SubscriberId,
}

impl Drop for Effect {
    fn drop(&mut self) {
        runtime::clear_subscriptions(self.id);
        runtime::remove_subscriber(self.id);
    }
}

pub fn effect(f: impl Fn() + 'static) {
    let id = runtime::create_subscriber(Rc::new(|| {}));

    let notify = Rc::new(move || {
        runtime::clear_subscriptions(id);
        runtime::push_observer(id);
        f();
        runtime::pop_observer();
    });

    runtime::update_subscriber(id, notify.clone());
    notify();

    owner::store(Effect { id });
}

mod tests {
    use super::effect;
    use crate::reactive::{owner::Owner, signal::Signal, state};
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    #[test]
    fn effect_runs_immediately() {
        let ran = Rc::new(Cell::new(false));
        let ran_clone = ran.clone();
        let owner = Owner::new();
        effect(move || {
            ran_clone.set(true);
        });
        let _owner = owner.collect();
        assert!(ran.get());
    }

    #[test]
    fn effect_reruns_on_change() {
        let count = state(0i32);
        let ran = Rc::new(Cell::new(0));
        let ran_clone = ran.clone();
        let owner = Owner::new();
        effect(move || {
            let _ = count.get();
            ran_clone.set(ran_clone.get() + 1);
        });
        let _owner = owner.collect();
        assert_eq!(ran.get(), 1);
        count.set(1);
        assert_eq!(ran.get(), 2);
        count.set(2);
        assert_eq!(ran.get(), 3);
    }

    #[test]
    fn effect_cleaned_up_when_owner_drops() {
        let count = state(0i32);
        let ran = Rc::new(Cell::new(0));
        let ran_clone = ran.clone();
        {
            let owner = Owner::new();
            effect(move || {
                let _ = count.get();
                ran_clone.set(ran_clone.get() + 1);
            });
            let _owner = owner.collect();
            assert_eq!(ran.get(), 1);
            count.set(1);
            assert_eq!(ran.get(), 2);
        } // owner drops here
        count.set(2);
        assert_eq!(ran.get(), 2); // must NOT run again
    }

    #[test]
    fn nested_owners_clean_up_independently() {
        let count = state(0i32);
        let outer_ran = Rc::new(Cell::new(0));
        let inner_ran = Rc::new(Cell::new(0));
        let outer_clone = outer_ran.clone();
        let inner_clone = inner_ran.clone();

        let outer_owner = Owner::new();

        effect(move || {
            let _ = count.get();
            outer_clone.set(outer_clone.get() + 1);
        });

        {
            let inner_owner = Owner::new();
            effect(move || {
                let _ = count.get();
                inner_clone.set(inner_clone.get() + 1);
            });
            let _inner_owner = inner_owner.collect();

            assert_eq!(outer_ran.get(), 1);
            assert_eq!(inner_ran.get(), 1);

            count.set(1);
            assert_eq!(outer_ran.get(), 2);
            assert_eq!(inner_ran.get(), 2);
        } // inner owner drops

        count.set(2);
        assert_eq!(outer_ran.get(), 3); // outer still runs
        assert_eq!(inner_ran.get(), 2); // inner stopped

        let _outer_owner = outer_owner.collect();
        drop(_outer_owner);

        count.set(3);
        assert_eq!(outer_ran.get(), 3); // outer stopped too
        assert_eq!(inner_ran.get(), 2);
    }
}
