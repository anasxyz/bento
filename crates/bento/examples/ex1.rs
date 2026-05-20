#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new());

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
