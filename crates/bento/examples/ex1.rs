#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let button = ui.add(Button::new("Click meeeeeeee!", 100.0, 100.0, 100.0, 32.0));

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
