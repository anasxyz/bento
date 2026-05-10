use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let root = ui.add(Group::new());
    ui.with(root, |group| {
        group.set_color(Some([0.1, 0.1, 0.1, 1.0]));
        group.set_width(Size::Px(200.0));
    });

    let button = ui.add_to(root, Button::new("click me"));
    ui.with(button, |button| {
        button.set_color([0.0, 0.0, 0.0, 1.0]);
        button.set_position(Position::Absolute);
        button.set_x(100.0);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
