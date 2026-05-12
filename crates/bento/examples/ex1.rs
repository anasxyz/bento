#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 0.0, 100.0, 100.0));

    // mouse move
    ui.listen_any(move |e: &Click, ui| {
        println!("click button {:?} at {:.0},{:.0}", e.button, e.x, e.y);
        ui.get_mut(rect).unwrap().set_x(e.x);
        ui.get_mut(rect).unwrap().set_y(e.y);
    });

    ui.listen_any(|e: &MouseEnter, ui| {
        println!("mouse enter");
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
