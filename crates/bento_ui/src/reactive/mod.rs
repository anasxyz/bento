pub(crate) mod runtime;
mod signal;
mod derived;
mod effect;

pub use signal::Signal;
pub use derived::Derived;
pub use effect::Effect;

pub fn state<T: Clone + 'static>(value: T) -> Signal<T> {
    Signal::new(value)
}

pub fn derived<T: Clone + 'static>(f: impl Fn() -> T + 'static) -> Derived<T> {
    Derived::new(f)
}

pub fn effect(f: impl Fn() + 'static) -> Effect {
    Effect::new(f)
}
