#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(100.0, 100.0, 200.0, 100.0));
    ui.listen(rect, move |e: &Click, ui| {
        ui.get_mut(rect).unwrap().set_color([1.0, 0.0, 0.0, 1.0]);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
