use std::{any::Any, cell::RefCell, marker::PhantomData, rc::Rc};

use slab::Slab;

use crate::reactive::signal::{Signal, SignalId};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct SubscriberId(pub usize);

struct SignalEntry {
    value: Box<dyn Any>,
    subscribers: Vec<SubscriberId>,
}

struct Runtime {
    signals: Slab<SignalEntry>,
    subscribers: Slab<Rc<dyn Fn()>>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            signals: Slab::new(),
            subscribers: Slab::new(),
        }
    }
}

thread_local! {
    static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::new());
}

/// Creates and stores a Signal with the given value in the reactive runtime
/// Returns the Signal handler
pub(crate) fn create_signal<T: 'static>(value: T) -> Signal<T> {
    let new_sig = SignalEntry {
        value: Box::new(value),
        subscribers: Vec::new(),
    };

    let new_sig_id = RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        rt.signals.insert(new_sig)
    });

    Signal {
        id: SignalId(new_sig_id),
        _phantom: PhantomData,
    }
}

/// Retrives a given signal from the reactive runtime
/// Returns the value associated with the given Signal
pub(crate) fn get_signal<T: Clone + 'static>(signal: Signal<T>) -> T {
    RUNTIME.with(|rt| {
        let rt = rt.borrow_mut();
        let s_id = signal.id.0;

        rt.signals[s_id].value.downcast_ref::<T>().unwrap().clone()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_signal() {
        let sig = create_signal(67);

        RUNTIME.with(|rt| {
            assert_eq!(rt.borrow().signals.len(), 1);
        });
    }

    #[test]
    fn test_get_signal() {
        let sig = create_signal(67);
        let sig_val = get_signal(sig);

        assert_eq!(sig_val, 67);
    }
}
