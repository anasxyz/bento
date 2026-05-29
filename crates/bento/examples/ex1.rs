#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();
    ui.debug = true;

    let text = ui.add(Text::new("Hello world"));
    ui.set(text, |t| {
        t.set_x(100.0);
        t.set_y(100.0);
    });

    let text2 = ui.add(Text::new("Second hello world"));
    ui.set(text2, |t| {
        t.set_x(100.0);
        t.set_y(300.0);
    });

    ui.asyncs.spawn(async move {
        tokio::time::sleep(web_time::Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            ui.set(text, |t| {
                t.set_x(200.0);
            });
            ui.set(text2, |t| {
                t.set_x(200.0);
            });
        }
    });

    ui.asyncs.spawn(async move {
        tokio::time::sleep(web_time::Duration::from_secs(4)).await;
        move |ui: &mut Ui| {
            ui.set(text, |t| {
                t.set_x(400.0);
            });
            ui.set(text2, |t| {
                t.set_x(400.0);
            });
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
