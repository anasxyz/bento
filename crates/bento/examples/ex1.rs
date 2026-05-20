#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add_rect(100.0, 100.0, 200.0, 100.0);
    ui.rect(rect).color = [1.0, 0.0, 0.0, 1.0];

    ui.asyncs.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            ui.rect(rect).color = [0.0, 1.0, 0.0, 1.0];
            ui.request_redraw();
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
