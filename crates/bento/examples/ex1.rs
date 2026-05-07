use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    ui.add_button(Button::new("Hello", 100.0, 100.0, 100.0, 100.0));

    app.open_window(WindowConfig::default(), ui);
    app.run();
}

