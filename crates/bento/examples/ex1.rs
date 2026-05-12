#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 0.0, 100.0, 100.0));

    // mouse move
    ui.listen(rect, move |e: &Click, ui| {
        println!("clicked rect");
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
