use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let text = ui.add(Text::new("Hello, world!", 100.0, 100.0, 32.0));

    let rect = ui.add(Rect::new(0.0, 0.0, 200.0, 100.0));

    let button = ui.add(Button::new("Click me!", 500.0, 100.0, 200.0, 50.0));

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
