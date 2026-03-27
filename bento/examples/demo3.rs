use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Rect::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(30, 30, 30))
        .set_flex_direction(FlexDirection::Col)
        .set_padding([16.0, 16.0, 16.0, 16.0]);
    ui.set_root(root);

    let scroll = ui.add(ScrollContainer::new());
    ui.get_mut(scroll)
        .unwrap()
        .set_flex_direction(FlexDirection::Col)
        .set_width(Size::Percent(100.0))
        .set_height(Size::Fixed(450.0))
        .set_color(Color::rgb(20, 20, 20));
    ui.append(root, scroll);

    for i in 0..100 {
        let label = ui.add(Label::new(&format!("Item {}", i)));
        ui.get_mut(label)
            .unwrap()
            .set_color(Color::WHITE)
            .set_size(16.0);
        ui.append(scroll, label);
    }

    let mut app = App::new();
    app.open_window(WindowConfig::default(), ui);
    app.run();
}
