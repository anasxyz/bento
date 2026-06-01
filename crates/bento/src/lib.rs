#![allow(dead_code)]
#![allow(unused)]

pub use bento_ui::*;
pub use bento_winit::{App, Window, WindowConfig};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct State {
    label: WidgetHandle<Text>,
    count: i32,
}

impl State {
    fn increment(&mut self, ui: &mut Ui) {
        self.count += 1;
        ui.set(self.label, |t: &mut Text| t.set_content(&format!("{}", self.count)));
    }

    fn decrement(&mut self, ui: &mut Ui) {
        self.count -= 1;
        ui.set(self.label, |t: &mut Text| t.set_content(&format!("{}", self.count)));
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();

    let mut app = App::new();
    let mut ui = Ui::new();

    let mut group = Group::new();
    group.layout = Layout::Row { gap: 16.0 };
    group.x = 100.0;
    group.y = 100.0;
    let group = ui.add(group);

    let btn_inc = ui.add(Button::new("+"));
    let label = ui.add(Text::new("0"));
    let btn_dec = ui.add(Button::new("-"));

    ui.attach(group, btn_inc);
    ui.attach(group, label);
    ui.attach(group, btn_dec);

    ui.set_state(State { label, count: 0 });

    ui.listen(btn_inc, move |_: &Click, ui: &mut Ui| {
        ui.with_state(|s: &mut State, ui: &mut Ui| s.increment(ui));
    });

    ui.listen(btn_dec, move |_: &Click, ui: &mut Ui| {
        ui.with_state(|s: &mut State, ui: &mut Ui| s.decrement(ui));
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
