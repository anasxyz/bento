pub(crate) mod effect;
pub(crate) mod runtime;
pub(crate) mod signal;
pub(crate) mod owner;

use effect::Effect;
use signal::Signal;

pub fn state<T: 'static>(value: T) -> Signal<T> {
    signal::state(value)
}

pub fn effect(f: impl Fn() + 'static) {
    effect::effect(f)
}
