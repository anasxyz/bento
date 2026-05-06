#![allow(unused)]
use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui.get_mut(root)
        .unwrap()
        .set_flex_direction(FlexDirection::Col)
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0));
    ui.set_root(root);

    let toolbar = ui.add_to(root, Container::new());
    ui.get_mut(toolbar)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Fixed(20.0))
        .set_color(Color::rgb(25, 25, 30));

    let labeltest = ui.add_to(root, Label::new("Label"));
    ui.get_mut(labeltest)
        .unwrap()
        .set_size(16.0)
        .set_selectable(true)
        .set_color(Color::WHITE);

    let file_btn = ui.add_to(toolbar, Button::new("File"));
    ui.get_mut(file_btn)
        .unwrap()
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(25, 25, 30))
        .set_font_size(12.0)
        .add_underline(0, 1, Color::WHITE, 1.0)
        .set_radius(0.0);

    let edit_btn = ui.add_to(toolbar, Button::new("Edit"));
    ui.get_mut(edit_btn)
        .unwrap()
        .set_height(Size::Percent(100.0))
        .set_font_size(12.0)
        .set_color(Color::rgb(25, 25, 30))
        .set_radius(0.0);

    let mid = ui.add_to(toolbar, Container::new());
    ui.get_mut(mid)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(25, 25, 30));

    let view_btn = ui.add_to(toolbar, Button::new("View"));
    ui.get_mut(view_btn)
        .unwrap()
        .set_height(Size::Percent(100.0))
        .set_font_size(12.0)
        .set_color(Color::rgb(25, 25, 30))
        .set_radius(0.0);

    let help_btn = ui.add_to(toolbar, Button::new("Help"));
    ui.get_mut(help_btn)
        .unwrap()
        .set_height(Size::Percent(100.0))
        .set_font_size(12.0)
        .add_underline(3, 4, Color::WHITE, 1.0)
        .set_color(Color::rgb(25, 25, 30))
        .set_radius(0.0);

    // dropdown — absolute, layer 1, hidden by default
    let dropdown = ui.add_to(root, Container::new());
    ui.get_mut(dropdown)
        .unwrap()
        .set_position(Position::Absolute)
        .set_inset([Size::Fixed(20.0), Size::Auto, Size::Auto, Size::Fixed(0.0)])
        .set_width(Size::Fixed(160.0))
        .set_flex_direction(FlexDirection::Col)
        .set_color(Color::rgb(35, 35, 42))
        .set_border_color(Color::rgb(60, 60, 72))
        .set_border_widths([1.0, 1.0, 1.0, 1.0])
        .set_radius(4.0)
        .set_layer(1)
        .set_display(false);

    for item in ["New", "Open", "Save", "Save As"] {
        let btn = ui.add_to(dropdown, Button::new(item));
        ui.get_mut(btn)
            .unwrap()
            .set_width(Size::Percent(100.0))
            .set_height(Size::Fixed(28.0))
            .set_color(Color::rgb(35, 35, 42))
            .set_font_size(13.0)
            .set_radius(0.0)
            .set_layer(1);
        ui.on::<Button, Click>(btn, move |ui, this, e| {
            println!("{}", this.get_text());
            if let Some(d) = ui.get_mut(dropdown) {
                d.set_display(false);
            }
        });
    }

    let sep = ui.add_to(dropdown, Container::new());
    ui.get_mut(sep)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Fixed(1.0))
        .set_color(Color::rgb(60, 60, 72))
        .set_layer(1);

    let exit_btn = ui.add_to(dropdown, Button::new("Exit"));
    ui.get_mut(exit_btn)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Fixed(28.0))
        .set_color(Color::rgb(35, 35, 42))
        .set_font_size(13.0)
        .set_radius(0.0)
        .set_layer(1);
    ui.on::<Button, Click>(exit_btn, |ui, this, e| {
        std::process::exit(0);
    });

    ui.on::<Button, Click>(file_btn, move |ui, this, e| {
        if let Some(d) = ui.get_mut(dropdown) {
            let shown = d.is_displayed();
            d.set_display(!shown);
        }
    });

    let mut app = App::new();
    app.open_window(WindowConfig::default(), ui);
    app.run();
}
