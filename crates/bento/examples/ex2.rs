use bento::*;

struct AppEvent {
    message: &'static str,
}

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let button = ui.add(Rect::new(50.0, 50.0, 100.0, 40.0));

    ui.listen(button, |_e: &Click, ui| {
        ui.send_global(AppEvent {
            message: "hello from button",
        });
    });

    ui.listen_any(|e: &AppEvent, _ui| {
        println!("global event received: {}", e.message);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
