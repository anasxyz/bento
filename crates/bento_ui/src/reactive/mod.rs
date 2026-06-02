pub(crate) mod signal;
pub(crate) mod runtime;

use signal::Signal;

pub fn state<T: 'static>(value: T) -> Signal<T> {
    runtime::create_signal(value)
}
