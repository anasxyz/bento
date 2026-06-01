use std::any::Any;
use std::cell::{Cell, RefCell};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct SubscriberId(pub usize);

struct SignalEntry {
    value: Box<dyn Any>,
    subscribers: Vec<SubscriberId>,
}

struct SubscriberEntry {
    id: SubscriberId,
    rank: usize,
}

struct Runtime {
    signals: Vec<SignalEntry>,
    subscribers: Vec<SubscriberEntry>,
    next_subscriber_id: usize,
    observer_stack: Vec<SubscriberId>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            signals: Vec::new(),
            subscribers: Vec::new(),
            next_subscriber_id: 0,
            observer_stack: Vec::new(),
        }
    }

    fn create_signal(&mut self, value: Box<dyn Any>) -> usize {
        let id = self.signals.len();
        self.signals.push(SignalEntry {
            value,
            subscribers: Vec::new(),
        });
        id
    }

    fn get<T: Clone + 'static>(&mut self, id: usize) -> T {
        if let Some(&observer) = self.observer_stack.last() {
            let entry = &mut self.signals[id];
            if !entry.subscribers.contains(&observer) {
                entry.subscribers.push(observer);
            }
        }
        self.signals[id]
            .value
            .downcast_ref::<T>()
            .expect("signal type mismatch")
            .clone()
    }

    fn set_value<T: 'static>(&mut self, id: usize, value: T) {
        self.signals[id].value = Box::new(value);
    }

    fn get_ordered_subscribers(&self, signal_id: usize) -> Vec<SubscriberId> {
        let subs = &self.signals[signal_id].subscribers;
        let mut queue: BinaryHeap<Reverse<(usize, SubscriberId)>> = BinaryHeap::new();
        for &sub_id in subs {
            if let Some(entry) = self.subscribers.iter().find(|e| e.id == sub_id) {
                queue.push(Reverse((entry.rank, sub_id)));
            }
        }
        let mut ordered = Vec::new();
        while let Some(Reverse((_, sub_id))) = queue.pop() {
            ordered.push(sub_id);
        }
        ordered
    }

    fn next_subscriber_id(&mut self) -> SubscriberId {
        let id = SubscriberId(self.next_subscriber_id);
        self.next_subscriber_id += 1;
        id
    }

    fn rank_of(&self, id: SubscriberId) -> usize {
        self.subscribers
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.rank)
            .unwrap_or(0)
    }

    fn register_subscriber(&mut self, id: SubscriberId, rank: usize) {
        if let Some(entry) = self.subscribers.iter_mut().find(|e| e.id == id) {
            entry.rank = rank;
        } else {
            self.subscribers.push(SubscriberEntry { id, rank });
        }
    }

    fn unregister_subscriber(&mut self, id: SubscriberId) {
        self.subscribers.retain(|e| e.id != id);
        for signal in &mut self.signals {
            signal.subscribers.retain(|&s| s != id);
        }
    }

    fn push_observer(&mut self, id: SubscriberId) {
        self.observer_stack.push(id);
    }

    fn pop_observer(&mut self) {
        self.observer_stack.pop();
    }

    fn max_dependency_rank(&self, id: SubscriberId) -> usize {
        let mut max = 0;
        for signal in &self.signals {
            if signal.subscribers.contains(&id) {
                for other in &self.subscribers {
                    if signal.subscribers.contains(&other.id) && other.id != id {
                        max = max.max(other.rank);
                    }
                }
            }
        }
        max
    }
}

thread_local! {
    static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::new());
    static NOTIFY_FNS: RefCell<Vec<(SubscriberId, Rc<dyn Fn()>)>> = RefCell::new(Vec::new());
    static FLUSH_QUEUE: RefCell<BinaryHeap<Reverse<(usize, SubscriberId)>>> = RefCell::new(BinaryHeap::new());
    static IS_FLUSHING: Cell<bool> = Cell::new(false);
}

pub(crate) fn create_signal_id<T: 'static>(value: T) -> usize {
    RUNTIME.with(|rt| rt.borrow_mut().create_signal(Box::new(value)))
}

pub(crate) fn get_signal_value<T: Clone + 'static>(id: usize) -> T {
    RUNTIME.with(|rt| rt.borrow_mut().get::<T>(id))
}

pub(crate) fn set_signal_value<T: 'static>(id: usize, value: T) {
    let ordered = RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        rt.set_value(id, value);
        rt.get_ordered_subscribers(id)
    });

    FLUSH_QUEUE.with(|q| {
        let mut q = q.borrow_mut();
        for sub_id in ordered {
            let rank = RUNTIME.with(|rt| rt.borrow().rank_of(sub_id));
            q.push(Reverse((rank, sub_id)));
        }
    });

    let already_flushing = IS_FLUSHING.with(|f| f.get());
    if already_flushing {
        return;
    }

    IS_FLUSHING.with(|f| f.set(true));

    loop {
        let next = FLUSH_QUEUE.with(|q| q.borrow_mut().pop());
        match next {
            None => break,
            Some(Reverse((_, sub_id))) => {
                let f = NOTIFY_FNS.with(|fns| {
                    fns.borrow()
                        .iter()
                        .find(|(sid, _)| *sid == sub_id)
                        .map(|(_, f)| f.clone())
                });
                if let Some(f) = f {
                    f();
                }
            }
        }
    }

    IS_FLUSHING.with(|f| f.set(false));
}

pub(crate) fn next_subscriber_id() -> SubscriberId {
    RUNTIME.with(|rt| rt.borrow_mut().next_subscriber_id())
}

pub(crate) fn register_subscriber(id: SubscriberId, rank: usize, notify: Rc<dyn Fn()>) {
    RUNTIME.with(|rt| rt.borrow_mut().register_subscriber(id, rank));
    NOTIFY_FNS.with(|fns| {
        let mut fns = fns.borrow_mut();
        if let Some(entry) = fns.iter_mut().find(|(sid, _)| *sid == id) {
            entry.1 = notify;
        } else {
            fns.push((id, notify));
        }
    });
}

pub(crate) fn rank_of(id: SubscriberId) -> usize {
    RUNTIME.with(|rt| rt.borrow().rank_of(id))
}

pub(crate) fn unregister_subscriber(id: SubscriberId) {
    RUNTIME.with(|rt| rt.borrow_mut().unregister_subscriber(id));
    let removed = NOTIFY_FNS.with(|fns| {
        let mut fns = fns.borrow_mut();
        let pos = fns.iter().position(|(sid, _)| *sid == id);
        pos.map(|i| fns.swap_remove(i))
    });
    drop(removed); // dropped here, outside any borrow
}

pub(crate) fn push_observer(id: SubscriberId) {
    RUNTIME.with(|rt| rt.borrow_mut().push_observer(id));
}

pub(crate) fn pop_observer() {
    RUNTIME.with(|rt| rt.borrow_mut().pop_observer());
}

pub(crate) fn max_dependency_rank(id: SubscriberId) -> usize {
    RUNTIME.with(|rt| rt.borrow().max_dependency_rank(id))
}
