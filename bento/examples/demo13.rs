use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_flex_direction(FlexDirection::Col)
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center)
        .set_row_gap(16.0);
    ui.set_root(root);

    let box1 = ui.add_to(root, Container::new());
    ui.get_mut(box1)
        .unwrap()
        .set_width(Size::Fixed(200.0))
        .set_height(Size::Fixed(60.0))
        .set_color(Color::rgb(80, 120, 200));

    let box2 = ui.add_to(root, Container::new());
    ui.get_mut(box2)
        .unwrap()
        .set_width(Size::Fixed(200.0))
        .set_height(Size::Fixed(60.0))
        .set_color(Color::rgb(200, 80, 80));

    let box3 = ui.add_to(root, Container::new());
    ui.get_mut(box3)
        .unwrap()
        .set_width(Size::Fixed(200.0))
        .set_height(Size::Fixed(60.0))
        .set_color(Color::rgb(80, 200, 80));

    // toggle display on box1 (removes from layout)
    let btn1 = ui.add_to(root, Button::new("Toggle display (blue)"));
    ui.on::<Button, Click>(btn1, move |ui, this, e| {
        if let Some(b) = ui.get_mut(box1) {
            b.set_display(!b.is_displayed());
        }
    });

    // toggle visibility on box2 (keeps space)
    let btn2 = ui.add_to(root, Button::new("Toggle visibility (red)"));
    ui.on::<Button, Click>(btn2, move |ui, this, e| {
        if let Some(b) = ui.get_mut(box2) {
            b.set_visibility(!b.is_visible());
        }
    });

    let mut app = App::new();
    app.open_window(WindowConfig::default(), ui);
    app.run();
}

