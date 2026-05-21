#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut rect = Rect::new();
    let mut rect2 = Rect::new();

    ui.add(&mut rect);
    ui.add(&mut rect2);

    rect.set_x(100.0);

    ui.print_slots();

    ui.remove(&rect);

    ui.print_slots();

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
