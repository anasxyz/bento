use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 0.0, 200.0, 100.0));

    ui.listen(rect, move |e: &KeyPress, ui| {
        if let Some(r) = ui.get_mut(rect) {
            match e.key {
                Key::Left => r.set_x(r.x() - 10.0),
                Key::Right => r.set_x(r.x() + 10.0),
                _ => {}
            }
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
