use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Rect::new());
    ui.get_mut(root)
        .unwrap()
        .set_flex_direction(FlexDirection::Col)
        .set_row_gap(20.0)
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0));
    ui.set_root(root);

    let toolbar = ui.add_to(root, Rect::new());
    ui.get_mut(toolbar)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Fixed(20.0))
        .set_color(Color::rgb(25, 25, 30));

    let toolbar_items = [
        "File",
        "Edit",
        "View",
        "Help",
        "About",
        "Quit",
    ];

    for text in toolbar_items {
        let button = ui.add_to(toolbar, Button::new(text));
        ui.get_mut(button)
            .unwrap()
            .set_height(Size::Percent(100.0))
            .set_color(Color::rgb(25, 25, 30))
            .set_radius(0.0);
        ui.on::<Button, Click>(button, |ui, this, e| {
            println!("clicked {}", this.get_text());
        });
    }

    let mut app = App::new();
    app.open_window(WindowConfig::default(), ui);
    app.run();
}
