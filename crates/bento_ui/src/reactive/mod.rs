pub(crate) mod derived;
pub(crate) mod effect;
pub(crate) mod owner;
pub(crate) mod runtime;
pub(crate) mod signal;
pub(crate) mod value;

use derived::Derived;
use signal::Signal;

pub fn state<T: 'static>(value: T) -> Signal<T> {
    signal::state(value)
}

pub fn effect(f: impl Fn() + 'static) {
    effect::effect(f)
}

pub fn derived<T: Clone + 'static>(f: impl Fn() -> T + 'static) -> Derived<T> {
    derived::derived(f)
}

#[macro_export]
macro_rules! inspect {
    ($signal:expr) => {
        $crate::reactive::effect({
            let s = $signal;
            move || {
                let msg = format!("[inspect] {} = {:?}", stringify!($signal), s.get());
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&msg.into());
                #[cfg(not(target_arch = "wasm32"))]
                eprintln!("{}", msg);
            }
        });
    };
}
