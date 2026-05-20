#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 0.0, 100.0, 100.0));
    ui.get_mut(rect).unwrap().set_color([1.0, 0.0, 0.0, 1.0]);

    ui.asyncs.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        move |ui: &mut Ui| {
            ui.remove(rect);
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
