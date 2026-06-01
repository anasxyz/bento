#![allow(dead_code)]
#![allow(unused)]

pub use bento_ui::*;
pub use bento_winit::{App, Window, WindowConfig};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct State {
    count: i32,
    label: WidgetHandle<Text>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();

    let mut app = App::new();
    let mut ui = Ui::new();

    let root = ui.root();
    ui.set(root, |g: &mut Group| {
        g.layout = Layout::Column {
            gap: 0.0,
            padding: [0.0; 4],
            main_axis: MainAxis::Start,
            cross_axis: CrossAxis::Stretch,
            wrap: false,
        };
        g.width = Size::Fill;
        g.height = Size::Fill;
    });

    // top bar
    let top_bar = ui.add(root, Group::new());
    ui.set(top_bar, |g: &mut Group| {
        g.layout = Layout::Row {
            gap: 8.0,
            padding: [8.0; 4],
            main_axis: MainAxis::SpaceBetween,
            cross_axis: CrossAxis::Center,
            wrap: false,
        };
        g.width = Size::Fill;
    });

    let btn_a = ui.add(top_bar, Button::new("File"));
    let btn_b = ui.add(top_bar, Button::new("Edit"));
    let btn_c = ui.add(top_bar, Button::new("View"));
    let btn_settings = ui.add(top_bar, Button::new("Settings"));

    // content area
    let content = ui.add(root, Group::new());
    ui.set(content, |g: &mut Group| {
        g.layout = Layout::Column {
            gap: 16.0,
            padding: [24.0; 4],
            main_axis: MainAxis::Start,
            cross_axis: CrossAxis::Center,
            wrap: false,
        };
        g.width = Size::Fill;
        g.height = Size::Fill;
    });

    let label = ui.add(content, Text::new("Hello"));
    let btn_inc = ui.add(content, Button::new("Increment"));
    let btn_dec = ui.add(content, Button::new("Decrement"));

    ui.set_state(State { count: 0, label });

    ui.listen(btn_inc, move |_: &Click, ui: &mut Ui| {
        ui.with_state(|s: &mut State, ui: &mut Ui| {
            s.count += 1;
            ui.set(s.label, |t: &mut Text| {
                t.set_content(&format!("Count: {}", s.count))
            });
        });
    });

    ui.listen(btn_dec, move |_: &Click, ui: &mut Ui| {
        ui.with_state(|s: &mut State, ui: &mut Ui| {
            s.count -= 1;
            ui.set(s.label, |t: &mut Text| {
                t.set_content(&format!("Count: {}", s.count))
            });
        });
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
