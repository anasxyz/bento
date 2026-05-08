use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let button_handle = ui.add(Button::new("Click me", 100.0, 100.0));

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
