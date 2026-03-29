use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Rect::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(18, 18, 18))
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center)
        .set_flex_direction(FlexDirection::Col)
        .set_row_gap(12.0);
    ui.set_root(root);

    let btn1 = ui.add(Button::new("Click me"));
    ui.get_mut(btn1)
        .unwrap()
        .set_width(Size::Fixed(160.0))
        .set_color(Color::rgb(99, 102, 241))
        .set_hover_color(Color::rgb(118, 120, 255))
        .set_pressed_color(Color::rgb(79, 82, 200))
        .set_radius(8.0)
        .set_font_size(14.0)
        .set_font_weight(600);
    ui.append(root, btn1);

    let btn2 = ui.add(Button::new("Disabled"));
    ui.get_mut(btn2)
        .unwrap()
        .set_width(Size::Fixed(160.0))
        .set_color(Color::rgb(99, 102, 241))
        .set_hover_color(Color::rgb(118, 120, 255))
        .set_pressed_color(Color::rgb(79, 82, 200))
        .set_disabled_color(Color::rgb(50, 52, 100))
        .set_radius(8.0)
        .set_font_size(14.0)
        .set_font_weight(600)
        .set_disabled(true);
    ui.append(root, btn2);

    let mut app = App::new();
    app.open_window(
        WindowConfig {
            title: "Button demo".to_string(),
            width: 400,
            height: 300,
            clear_color: Color::rgb(18, 18, 18),
        },
        ui,
    );
    app.run();
}
