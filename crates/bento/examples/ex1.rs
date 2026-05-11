#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    ui.add(Rect::new(0.0, 0.0, 100.0, 100.0));

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
