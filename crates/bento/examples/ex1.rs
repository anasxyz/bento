#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    ui.events.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        move |ui: &mut Ui| {
            println!("callback");
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
