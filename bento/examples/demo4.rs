use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Rect::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(30, 30, 30))
        .set_flex_direction(FlexDirection::Row);
    ui.set_root(root);

    let label = ui.add(Label::new("Name: "));
    ui.get_mut(label).unwrap()
        .set_size(16.0);
    ui.append(root, label);

    let input = ui.add(TextInput::new());
    ui.get_mut(input).unwrap()
        .set_width(Size::Fixed(300.0))
        .set_placeholder("Enter your name")
        .set_font_size(16.0)
        .set_border_width(2.0)
        .set_border_radius(4.0);
        // .set_font_family("ZedMono Nerd Font Mono");
    ui.append(root, input);

    let mut app = App::new();
    app.open_window(WindowConfig::default(), ui);
    app.run();
}
