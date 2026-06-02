#![allow(dead_code)]
#![allow(unused)]

pub use bento_ui::*;
pub use bento_winit::{App, Window, WindowConfig};

fn app() -> impl View {
    let count = state(0);

    let inc = move || count.set(count.get() + 1);

    timer(2.0, move || count.set(count.get() + 1));

    text(move || format!("count: {}", count.get())).on(move |e: &Click| inc())
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();

    App::run(app());
}
