use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let text = ui.add(Text::new("Click me", 0.0, 0.0, 30.0));

    ui.with(text, |t| {
        t.set_color([0.3, 0.5, 1.0, 1.0]);
        t.set_size(16.0);
        t.set_transition_x(2.0, Easing::EaseInOut);
        t.set_x(200.0);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}

