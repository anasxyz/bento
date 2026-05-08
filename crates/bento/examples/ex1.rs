use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut group = Group::new();
    group.offset_x = 10.0;
    group.add(Button::new("Click me", 0.0, 0.0));
    group.add(Button::new("Another", 0.0, 60.0));
    ui.add(group);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
