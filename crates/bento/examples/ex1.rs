#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let text = ui.add(Text::new("Hello world"));
    ui.get_mut(text).unwrap().set_x(100.0);
    ui.get_mut(text).unwrap().set_y(100.0);

    ui.asyncs.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            ui.get_mut(text).unwrap().set_x(200.0);
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
