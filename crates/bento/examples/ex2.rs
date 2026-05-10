use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 100.0, 200.0, 100.0));

    ui.with(rect, |r| {
        r.set_color([0.0, 0.5, 1.0, 1.0]);
        r.set_radii([10.0; 4]);
        r.animate_x(500.0, 10.0, Easing::EaseInOut, LoopMode::Once);
    });

    ui.timer(2.5, move |ui| {
        ui.with(rect, |r| { 
            r.stop_x_animation(); 
        });
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}

