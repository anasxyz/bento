use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut button = Button::new("Click meeeeeeeee", 100.0, 100.0);
    button.color = [0.2, 0.5, 1.0, 1.0];
    button.font_size = 12.0;
    button.max_width = Some(100.0);
    ui.add(button);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
