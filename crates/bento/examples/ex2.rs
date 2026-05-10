use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 100.0, 200.0, 100.0));

    ui.with(rect, |r| {
        r.set_color([0.0, 0.5, 1.0, 1.0]);
        r.set_radii([10.0; 4]);
    });

    ui.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            ui.with(rect, |r| {
                r.animate_color([1.0, 0.0, 0.0, 1.0], 1.0, Easing::EaseInOut, LoopMode::Once);
            });
        }
    });

    ui.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            ui.with(rect, |r| {
                r.animate_x(100.0, 1.0, Easing::EaseInOut, LoopMode::Once);
            });
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
