#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 0.0, 100.0, 100.0));

    let conn = ui.listen_any(move |e: &KeyPress, ui| {
        println!("key={:?}", e.key);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
