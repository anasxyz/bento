#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let btn = ui.add(Button::new());
    ui.get_mut(btn).unwrap().set_color([1.0, 0.0, 0.0, 1.0]);

    ui.print_slots();

    ui.asyncs.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            ui.get_mut(btn).unwrap().set_color([0.0, 1.0, 0.0, 1.0]);
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
