use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut rect = Rect::new(100.0, 100.0, 200.0, 50.0);
    rect.color = [0.2, 0.5, 1.0, 1.0];
    ui.add(rect);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
