use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Rect::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgba(30, 30, 30, 0))
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center)
        .set_flex_direction(FlexDirection::Col)
        .set_row_gap(8.0);
    ui.set_root(root);

    let row1 = ui.add(Rect::new());
    ui.get_mut(row1)
        .unwrap()
        .set_flex_direction(FlexDirection::Row)
        .set_align_items(AlignItems::Center);
    ui.append(root, row1);

    let row2 = ui.add(Rect::new());
    ui.get_mut(row2)
        .unwrap()
        .set_flex_direction(FlexDirection::Row)
        .set_align_items(AlignItems::Center);
    ui.append(root, row2);

    let label = ui.add(Label::new("Name:"));
    ui.get_mut(label)
        .unwrap()
        .set_size(16.0)
        .set_width(Size::Fixed(100.0));
    ui.append(row1, label);

    let input = ui.add(TextInput::new());
    ui.get_mut(input)
        .unwrap()
        .set_width(Size::Fixed(300.0))
        .set_placeholder("Enter your name")
        .set_font_size(16.0)
        .set_border_width(2.0)
        .set_border_radius(0.0);
    ui.append(row1, input);

    let label2 = ui.add(Label::new("Password:"));
    ui.get_mut(label2)
        .unwrap()
        .set_size(16.0)
        .set_width(Size::Fixed(100.0));
    ui.append(row2, label2);

    let input2 = ui.add(TextInput::new());
    ui.get_mut(input2)
        .unwrap()
        .set_width(Size::Fixed(300.0))
        .set_placeholder("Enter your password")
        .set_font_size(16.0)
        .set_border_width(2.0)
        .set_border_radius(0.0);
    ui.append(row2, input2);

    let mut app = App::new();
    app.open_window(
        WindowConfig {
            clear_color: Color::rgb(10, 10, 10),
            ..WindowConfig::default()
        },
        ui,
    );
    app.run();
}
