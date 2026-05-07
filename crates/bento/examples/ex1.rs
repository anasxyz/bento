use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut button = Button::new("Click me", 100.0, 80.0);
    button.color = [0.2, 0.5, 1.0, 1.0];
    button.font_size = 28.0;
    button.radius = 10.0;
    button.font_weight = 700;
    button.border_color = [0.0, 0.0, 0.0, 1.0];
    button.border_width = 3.0;
    ui.add(button);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
