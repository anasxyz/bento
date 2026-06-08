use std::{
    any::Any,
    cell::{Cell, RefCell},
    marker::PhantomData,
    rc::Rc,
};

use slab::Slab;

use crate::reactive::{
    owner,
    signal::{Signal, SignalId},
};

/// Basically just an index into the subscribers slab on the runtime
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct SubscriberId(pub usize);

#[derive(Debug)]
struct SignalEntry {
    value: Box<dyn Any>,
    subscribers: Vec<SubscriberId>,
}

struct Runtime {
    signals: Slab<SignalEntry>,
    subscribers: Slab<Rc<dyn Fn()>>,
    observer_stack: Vec<SubscriberId>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            signals: Slab::new(),
            subscribers: Slab::new(),
            observer_stack: Vec::new(),
        }
    }
}

thread_local! {
    static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::new());
    static FLUSH_QUEUE: RefCell<Vec<Rc<dyn Fn()>>> = RefCell::new(Vec::new());
    static IS_FLUSHING: Cell<bool> = Cell::new(false);
}

/// Creates and stores a Signal with the given value in the reactive runtime
/// Returns the Signal handler
pub(crate) fn create_signal<T: 'static>(value: T) -> Signal<T> {
    let new_sig_entry = SignalEntry {
        value: Box::new(value),
        subscribers: Vec::new(),
    };

    let new_sig_id = RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        rt.signals.insert(new_sig_entry)
    });

    // eprintln!("[create_signal] created signal {:?}", new_sig_id);

    let new_sig = Signal {
        id: SignalId(new_sig_id),
        _phantom: PhantomData,
    };

    owner::register_cleanup(move || {
        destroy_signal(SignalId(new_sig_id));
    });

    new_sig
}

pub(crate) fn destroy_signal(id: SignalId) {
    // eprintln!("[destroy_signal] destroying signal {:?}", id);
    RUNTIME.with(|rt| {
        rt.borrow_mut().signals.remove(id.0);
    });
}

/// Retrives a given signal from the reactive runtime
/// Returns the value associated with the given Signal
pub(crate) fn get_signal<T: Clone + 'static>(signal: Signal<T>) -> T {
    RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        let s_id = signal.id.0;

        // if someone is watching, subscribe them to this signal
        if let Some(&observer) = rt.observer_stack.last() {
            // same observer can only appear once per signal
            if !rt.signals[s_id].subscribers.contains(&observer) {
                rt.signals[s_id].subscribers.push(observer);
            }
        }

        rt.signals[s_id].value.downcast_ref::<T>().unwrap().clone()
    })
}

/// Sets new value to a given signal
/// Both values must be of the same type
pub(crate) fn set_signal<T: 'static>(signal: Signal<T>, value: T) {
    let fns: Vec<Rc<dyn Fn()>> = RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        let sig_id = signal.id.0;
        rt.signals[sig_id].value = Box::new(value);

        // iterate signal's list of SubscriberIds, which are just indices into the runtime
        // subscribers slab
        // for each id, look up the Rc<dyn Fn()>> in that index and clone it
        // return a Vec of the collected functions
        rt.signals[sig_id]
            .subscribers
            .iter()
            .filter_map(|sub_id| rt.subscribers.get(sub_id.0).cloned())
            .collect()
    });

    FLUSH_QUEUE.with(|q| q.borrow_mut().extend(fns));

    if IS_FLUSHING.with(|f| f.get()) {
        // already flushing, just queue
        return;
    }

    IS_FLUSHING.with(|f| f.set(true));
    loop {
        let batch: Vec<Rc<dyn Fn()>> = FLUSH_QUEUE.with(|q| q.borrow_mut().drain(..).collect());
        if batch.is_empty() {
            break;
        }
        for f in batch {
            f();
        }
    }
    IS_FLUSHING.with(|f| f.set(false));
}

pub(crate) fn create_subscriber(f: Rc<dyn Fn()>) -> SubscriberId {
    RUNTIME.with(|rt| {
        let id = rt.borrow_mut().subscribers.insert(f);
        SubscriberId(id)
    })
}

pub(crate) fn push_observer(id: SubscriberId) {
    RUNTIME.with(|rt| rt.borrow_mut().observer_stack.push(id));
}

pub(crate) fn pop_observer() {
    RUNTIME.with(|rt| rt.borrow_mut().observer_stack.pop());
}

/// Removes the subscriber id from every signal's subscriber list
/// so on the next rerun only the signals actually read get resubscribed
pub(crate) fn clear_subscriptions(id: SubscriberId) {
    RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        for (_, signal) in rt.signals.iter_mut() {
            signal.subscribers.retain(|&s| s != id);
        }
    });
}

/// Updates a subscriber in the runtime to a new function given its id
pub(crate) fn update_subscriber(id: SubscriberId, f: Rc<dyn Fn()>) {
    RUNTIME.with(|rt| {
        if let Some(entry) = rt.borrow_mut().subscribers.get_mut(id.0) {
            *entry = f;
        }
    });
}

/// Removes a subscriber from the runtime given its id
pub(crate) fn remove_subscriber(id: SubscriberId) {
    let removed = RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        if rt.subscribers.contains(id.0) {
            Some(rt.subscribers.remove(id.0))
        } else {
            None
        }
    });

    drop(removed);
}

pub(crate) fn subscriber_count(signal_id: usize) -> usize {
    RUNTIME.with(|rt| rt.borrow().signals[signal_id].subscribers.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_signal() {
        let sig = create_signal(67);
        let sig_val = get_signal(sig);

        assert_eq!(sig_val, 67);
    }

    #[test]
    fn test_set_signal() {
        let sig = create_signal(69);
        let sig_val = get_signal(sig);
        assert_eq!(sig_val, 69);

        set_signal(sig, 67);
        let new_sig_val = get_signal(sig);
        assert_eq!(new_sig_val, 67);
    }

    #[test]
    fn test_push_observer() {
        let sub_id = create_subscriber(Rc::new(|| {}));

        push_observer(sub_id);

        RUNTIME.with(|rt| {
            assert_eq!(rt.borrow().observer_stack.len(), 1);
            assert_eq!(rt.borrow().observer_stack[0], sub_id);
        });
    }

    #[test]
    fn test_pop_observer() {
        let sub_id = create_subscriber(Rc::new(|| {}));

        push_observer(sub_id);
        pop_observer();

        RUNTIME.with(|rt| {
            assert_eq!(rt.borrow().observer_stack.len(), 0);
        });
    }

    #[test]
    fn test_subscribe() {
        let sig = create_signal(67);
        let sub_id = create_subscriber(Rc::new(|| {}));

        push_observer(sub_id);
        get_signal(sig);
        pop_observer();

        RUNTIME.with(|rt| {
            let rt = rt.borrow();
            assert!(rt.signals[sig.id.0].subscribers.contains(&sub_id));
        });
    }

    #[test]
    fn test_notify_fn_called_on_set() {
        use std::cell::Cell;
        let sig = create_signal(0i32);
        let called = Rc::new(Cell::new(0));
        let called_clone = called.clone();

        let sub_id = create_subscriber(Rc::new(move || {
            called_clone.set(called_clone.get() + 1);
        }));

        push_observer(sub_id);
        get_signal(sig);
        pop_observer();

        set_signal(sig, 1);
        assert_eq!(called.get(), 1);
        set_signal(sig, 2);
        assert_eq!(called.get(), 2);
    }
}
