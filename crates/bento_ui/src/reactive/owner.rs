use std::cell::RefCell;

pub struct Owner {
    pub cleanups: Vec<Box<dyn FnOnce()>>,
}

impl Owner {
    pub fn new() -> Self {
        OWNER_STACK.with(|s| s.borrow_mut().push(Vec::new()));
        Self {
            cleanups: Vec::new(),
        }
    }

    pub fn collect(mut self) -> Self {
        OWNER_STACK.with(|s| {
            if let Some(cleanups) = s.borrow_mut().pop() {
                self.cleanups = cleanups;
            }
        });
        self
    }
}

impl Drop for Owner {
    fn drop(&mut self) {
        for cleanup in self.cleanups.drain(..) {
            cleanup();
        }
    }
}

thread_local! {
    static OWNER_STACK: RefCell<Vec<Vec<Box<dyn FnOnce()>>>> = RefCell::new(Vec::new());
}

pub(crate) fn register_cleanup(f: impl FnOnce() + 'static) {
    OWNER_STACK.with(|s| {
        if let Some(top) = s.borrow_mut().last_mut() {
            top.push(Box::new(f));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reactive::{effect, state};
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn cleanup_on_drop() {
        let ran = Rc::new(Cell::new(0));
        let ran_clone = ran.clone();

        let count = state(0i32);

        let owner = Owner::new();
        let _eff = effect(move || {
            let _ = count.get();
            ran_clone.set(ran_clone.get() + 1);
        });
        let owner = owner.collect();

        assert_eq!(ran.get(), 1); // ran once on init
        count.set(1);
        assert_eq!(ran.get(), 2); // ran on change

        drop(owner); // cleanup
        count.set(2);
        assert_eq!(ran.get(), 2); // should NOT have run again
    }
}
