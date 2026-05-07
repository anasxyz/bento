use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut rect = Rect::new(100.0, 100.0, 200.0, 50.0);
    rect.color = [0.2, 0.5, 1.0, 1.0];
    rect.radii = [10.0; 4];
    ui.add(rect);

    let mut text = Text::new("Hello", 0.0, 0.0, 25.0);
    text.color = [0.0, 0.0, 0.0, 1.0];
    ui.add(text);

    app.open_window(WindowConfig {
        title: "demo".to_string(),
        width: 800,
        height: 600,
        clear_color: [1.0, 1.0, 1.0, 1.0],
    }, ui);
    app.run();
}
