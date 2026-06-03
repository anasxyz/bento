use std::any::Any;
use std::cell::RefCell;

pub struct Owner {
    cleanups: Vec<Box<dyn FnOnce()>>,
    owned: Vec<Box<dyn Any>>,
}

impl Owner {
    pub fn new() -> Self {
        OWNER_STACK.with(|s| s.borrow_mut().push(OwnerScope::new()));
        Self {
            cleanups: Vec::new(),
            owned: Vec::new(),
        }
    }

    pub fn collect(mut self) -> Self {
        OWNER_STACK.with(|s| {
            if let Some(scope) = s.borrow_mut().pop() {
                self.cleanups = scope.cleanups;
                self.owned = scope.owned;
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
        self.owned.clear();
    }
}

struct OwnerScope {
    cleanups: Vec<Box<dyn FnOnce()>>,
    owned: Vec<Box<dyn Any>>,
}

impl OwnerScope {
    fn new() -> Self {
        Self {
            cleanups: Vec::new(),
            owned: Vec::new(),
        }
    }
}

thread_local! {
    static OWNER_STACK: RefCell<Vec<OwnerScope>> = RefCell::new(Vec::new());
}

pub(crate) fn register_cleanup(f: impl FnOnce() + 'static) {
    OWNER_STACK.with(|s| {
        if let Some(top) = s.borrow_mut().last_mut() {
            top.cleanups.push(Box::new(f));
        }
    });
}

/// Store a value in the current owner scope to keep it alive.
/// Returns true if there was an owner, false if not.
pub(crate) fn store(value: impl Any + 'static) -> bool {
    OWNER_STACK.with(|s| {
        let mut s = s.borrow_mut();
        if let Some(top) = s.last_mut() {
            top.owned.push(Box::new(value));
            true
        } else {
            false
        }
    })
}
