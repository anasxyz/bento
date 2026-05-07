use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    ui.add(Button::new("Click me", 100.0, 100.0, 200.0, 50.0));

    app.open_window(
        WindowConfig {
            title: "demo".to_string(),
            width: 800,
            height: 600,
            clear_color: [1.0, 1.0, 1.0, 1.0],
        },
        ui,
    );
    app.run();
}
