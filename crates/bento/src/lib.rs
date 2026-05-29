#![allow(dead_code)]
#![allow(unused)]

pub use bento_ui::*;
pub use bento_winit::{App, Window, WindowConfig};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();

    let mut app = App::new();
    let mut ui = Ui::new();

    let input = ui.add(LineInput::new());

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
