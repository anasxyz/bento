#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let btn = ui.add(Button::new());

    ui.print_slots();

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
