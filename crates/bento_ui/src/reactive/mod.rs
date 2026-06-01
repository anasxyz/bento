mod runtime;
mod signal;

pub use signal::Signal;

pub fn state<T: Clone + 'static>(value: T) -> Signal<T> {
    Signal::new(value)
}
