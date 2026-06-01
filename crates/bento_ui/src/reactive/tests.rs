#[cfg(test)]
mod tests {
    use crate::reactive::{derived, effect, owner::Owner, state};
    use crate::rect;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn nested_owners_cleanup() {
        let outer_ran = Rc::new(Cell::new(0));
        let inner_ran = Rc::new(Cell::new(0));

        let count = state(0i32);

        let outer_ran_clone = outer_ran.clone();
        let inner_ran_clone = inner_ran.clone();

        let outer_owner = Owner::new();
        let _outer_eff = effect(move || {
            let _ = count.get();
            outer_ran_clone.set(outer_ran_clone.get() + 1);
        });

        let inner_owner = Owner::new();
        let _inner_eff = effect(move || {
            let _ = count.get();
            inner_ran_clone.set(inner_ran_clone.get() + 1);
        });
        let inner_owner = inner_owner.collect();
        let outer_owner = outer_owner.collect();

        assert_eq!(outer_ran.get(), 1);
        assert_eq!(inner_ran.get(), 1);

        count.set(1);
        assert_eq!(outer_ran.get(), 2);
        assert_eq!(inner_ran.get(), 2);

        // drop inner only
        drop(inner_owner);
        count.set(2);
        assert_eq!(outer_ran.get(), 3); // outer still runs
        assert_eq!(inner_ran.get(), 2); // inner stopped

        // drop outer
        drop(outer_owner);
        count.set(3);
        assert_eq!(outer_ran.get(), 3); // outer stopped too
    }

    #[test]
    fn shared_state_outside_owner() {
        let count = state(0i32); // no owner, lives forever

        let ran = Rc::new(Cell::new(0));
        let ran_clone = ran.clone();

        let owner = Owner::new();
        let _eff = effect(move || {
            let _ = count.get();
            ran_clone.set(ran_clone.get() + 1);
        });
        let owner = owner.collect();

        count.set(1);
        assert_eq!(ran.get(), 2);

        drop(owner);
        count.set(2);
        assert_eq!(ran.get(), 2); // effect gone but signal still alive

        // signal still works after owner dropped
        assert_eq!(count.get(), 2);
    }

    #[test]
    fn stale_subscriptions_dont_accumulate() {
        let switch = state(true);
        let a = state(1i32);
        let b = state(2i32);
        let ran = Rc::new(Cell::new(0));
        let ran_clone = ran.clone();

        let owner = Owner::new();
        let _eff = effect(move || {
            if switch.get() {
                let _ = a.get();
            } else {
                let _ = b.get();
            }
            ran_clone.set(ran_clone.get() + 1);
        });
        let owner = owner.collect();

        assert_eq!(ran.get(), 1);

        // switch to b
        switch.set(false);
        assert_eq!(ran.get(), 2);

        // a should no longer trigger the effect
        a.set(99);
        assert_eq!(ran.get(), 2);

        // b should trigger
        b.set(99);
        assert_eq!(ran.get(), 3);

        drop(owner);
    }

    #[test]
    fn diamond_dependency() {
        // count -> doubled
        // count -> tripled
        // doubled + tripled -> sum
        let count = state(1i32);
        let doubled = derived(move || count.get() * 2);
        let tripled = derived(move || count.get() * 3);
        let sum = derived(move || doubled.get() + tripled.get());

        assert_eq!(sum.get(), 5);
        count.set(2);
        assert_eq!(sum.get(), 10);
        count.set(3);
        assert_eq!(sum.get(), 15);
    }

    #[test]
    fn dropped_signal_no_dangling_subscribers() {
        let ran = Rc::new(Cell::new(0));
        let ran_clone = ran.clone();

        let outer = state(0i32);

        let owner = Owner::new();
        let inner = state(0i32);
        let _eff = effect(move || {
            let _ = inner.get();
            ran_clone.set(ran_clone.get() + 1);
        });
        let owner = owner.collect();

        assert_eq!(ran.get(), 1);
        inner.set(1);
        assert_eq!(ran.get(), 2);

        drop(owner); // inner signal and effect dropped

        // outer signal should still work fine
        outer.set(1);
        assert_eq!(outer.get(), 1);
    }

    #[test]
    fn nested_owners_register_correctly() {
        let ran_outer = Rc::new(Cell::new(0));
        let ran_inner = Rc::new(Cell::new(0));
        let count = state(0i32);

        let ran_outer_clone = ran_outer.clone();
        let ran_inner_clone = ran_inner.clone();

        // open outer
        let outer = Owner::new();
        let _eff_outer = effect(move || {
            let _ = count.get();
            ran_outer_clone.set(ran_outer_clone.get() + 1);
        });

        // open inner while outer is open
        let inner = Owner::new();
        let _eff_inner = effect(move || {
            let _ = count.get();
            ran_inner_clone.set(ran_inner_clone.get() + 1);
        });
        let inner = inner.collect();
        let outer = outer.collect();

        count.set(1);
        assert_eq!(ran_outer.get(), 2);
        assert_eq!(ran_inner.get(), 2);

        drop(inner);
        count.set(2);
        assert_eq!(ran_outer.get(), 3);
        assert_eq!(ran_inner.get(), 2); // inner stopped, outer still runs

        drop(outer);
        count.set(3);
        assert_eq!(ran_outer.get(), 3); // outer stopped too
    }

    #[test]
    fn component_macro_cleans_up() {
        use crate::OwnedView;

        let count = state(0i32);
        let ran = Rc::new(Cell::new(0));
        let ran_clone = ran.clone();

        // simulate what #[component] produces
        let owner = Owner::new();
        let _eff = effect(move || {
            let _ = count.get();
            ran_clone.set(ran_clone.get() + 1);
        });
        let view = rect();
        let owned: OwnedView = OwnedView::new(owner.collect(), view);

        assert_eq!(ran.get(), 1);
        count.set(1);
        assert_eq!(ran.get(), 2);

        drop(owned); // owner drops, effect should be cleaned up
        count.set(2);
        assert_eq!(ran.get(), 2); // effect is gone
    }
}
