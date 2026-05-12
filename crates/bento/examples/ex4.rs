use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let status = ui.add(Text::new("Press the button...", 20.0, 20.0, 18.0));
    let btn = ui.add(Button::new("Fetch", 20.0, 60.0, 120.0, 40.0));

    ui.listen(btn, move |_e: &Click, ui| {
        if let Some(t) = ui.get_mut(status) {
            t.set_text("Loading...");
        }

        ui.asyncs.timer(2.0, move |ui| {
            if let Some(t) = ui.get_mut(status) {
                t.set_text("Done! Data loaded.");
            }
        });
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
