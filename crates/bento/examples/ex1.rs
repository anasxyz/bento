#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 0.0, 100.0, 100.0));

    let conn = ui.on_any(move |e: &KeyPress, ui| {
        let key = e.key;
        ui.events.spawn(async move {
            // async work here
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            // return a closure that runs on main thread
            move |ui: &mut Ui| {
                println!("key={:?}", key);
            }
        });
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
