use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 100.0, 200.0, 100.0));

    ui.with(rect, |r| {
        r.set_color([0.0, 0.5, 1.0, 1.0]);
        r.set_radii([10.0; 4]);
    });

    ui.timer(1.0, move |ui: &mut Ui| {
        ui.remove(rect);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
