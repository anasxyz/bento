#![allow(dead_code)]
#![allow(unused)]

pub use bento_ui::*;
pub use bento_winit::{App, Window, WindowConfig};
pub use bento_macros::component;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();

}
