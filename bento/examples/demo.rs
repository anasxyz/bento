use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Rect::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(30, 30, 30))
        .set_flex_direction(FlexDirection::Row)
        .set_padding([16.0, 16.0, 16.0, 16.0])
        .set_row_gap(8.0);
    ui.set_root(root);

    for i in 0..1000 {
        let label = ui.add(Label::new(&format!("Item {}", i)));
        ui.get_mut(label)
            .unwrap()
            .set_size(14.0)
            .set_color(Color::WHITE);
        ui.append(root, label);
    }

    let mut app = App::new();
    app.open_window(WindowConfig::default(), ui);
    app.run();
}

