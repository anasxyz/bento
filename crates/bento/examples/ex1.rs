#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new());

    ui.get_mut(rect).unwrap().set_x(100.0);

    ui.remove(rect);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
