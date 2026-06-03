pub(crate) mod effect;
pub(crate) mod runtime;
pub(crate) mod signal;
pub(crate) mod owner;
pub(crate) mod derived;

use signal::Signal;
use derived::Derived;

pub fn state<T: 'static>(value: T) -> Signal<T> {
    signal::state(value)
}

pub fn effect(f: impl Fn() + 'static) {
    effect::effect(f)
}

pub fn derived<T: Clone + 'static>(f: impl Fn() -> T + 'static) -> Derived<T> {
    derived::derived(f)
}
