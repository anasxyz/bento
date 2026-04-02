use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(20, 20, 20));
    ui.set_root(root);

    // layer 0 — big red box
    let back = ui.add_to(root, Container::new());
    ui.get_mut(back)
        .unwrap()
        .set_width(Size::Fixed(300.0))
        .set_height(Size::Fixed(300.0))
        .set_color(Color::rgb(180, 40, 40))
        .set_layer(0);

    // layer 1 — smaller blue box, same position via absolute inset
    // should draw ON TOP of the red box
    let front = ui.add_to(root, Container::new());
    ui.get_mut(front)
        .unwrap()
        .set_width(Size::Fixed(150.0))
        .set_height(Size::Fixed(150.0))
        .set_color(Color::rgba(40, 40, 180, 255))
        .set_position(Position::Absolute)
        .set_inset([
            Size::Fixed(100.0),
            Size::Auto,
            Size::Auto,
            Size::Fixed(100.0),
        ])
        .set_layer(2);

    let lbl = ui.add_to(root, Label::new("layer 1"));
    ui.get_mut(lbl)
        .unwrap()
        .set_size(16.0)
        .set_selectable(true)
        .set_position(Position::Absolute)
        .set_inset([
            Size::Fixed(100.0),
            Size::Auto,
            Size::Auto,
            Size::Fixed(80.0),
        ])
        .set_layer(1)
        .set_color(Color::WHITE);

    let lbl = ui.add_to(root, Label::new("layerrrrrrrrrrrrrrrrrrr"));
    ui.get_mut(lbl)
        .unwrap()
        .set_size(16.0)
        .set_selectable(true)
        .set_position(Position::Absolute)
        .set_inset([
            Size::Fixed(120.0),
            Size::Auto,
            Size::Auto,
            Size::Fixed(120.0),
        ])
        .set_layer(10)
        .set_color(Color::WHITE);

    let mut app = App::new();
    app.open_window(WindowConfig::default(), ui);
    app.run();
}
