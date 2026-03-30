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

    let status = ui.add_to(root, Label::new("hover or click the buttons"));
    ui.get_mut(status)
        .unwrap()
        .set_size(13.0)
        .set_color(Color::rgb(140, 140, 140))
        .set_width(Size::Fixed(320.0));

    // button 1 — normal, internal hover behaviour runs
    let btn1 = ui.add_to(root, Button::new("Normal button"));
    ui.get_mut(btn1)
        .unwrap()
        .set_width(Size::Fixed(320.0))
        .set_color(Color::rgb(60, 100, 60));

    ui.on::<Button, Hover>(btn1, move |ui, this, e| {
        if let Some(lbl) = ui.get_mut(status) {
            lbl.set_text("btn1 hovered — internal hover still runs (color changes)");
        }
    });

    // button 2 — stop_default, internal hover colour change is prevented
    let btn2 = ui.add_to(root, Button::new("stop_default on hover"));
    ui.get_mut(btn2)
        .unwrap()
        .set_width(Size::Fixed(320.0))
        .set_color(Color::rgb(100, 60, 60));

    ui.on::<Button, Hover>(btn2, move |ui, this, e| {
        e.stop_default(); // internal hover colour change won't run
        if let Some(lbl) = ui.get_mut(status) {
            lbl.set_text("btn2 hovered — stop_default called, no colour change");
        }
    });

    // outer container to test stop_propagation
    let outer = ui.add_to(root, Rect::new());
    ui.get_mut(outer)
        .unwrap()
        .set_width(Size::Fixed(320.0))
        .set_height(Size::Fixed(50.0))
        .set_color(Color::rgb(40, 40, 80))
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center);

    let btn3 = ui.add_to(outer, Button::new("stop_propagation on click"));
    ui.get_mut(btn3)
        .unwrap()
        .set_width(Size::Fixed(280.0))
        .set_color(Color::rgb(60, 60, 160));

    // btn3 click stops propagation — outer never sees it
    ui.on::<Button, Click>(btn3, move |ui, this, e| {
        println!("btn3 click fired, stopping propagation");
        e.stop_propagation();
        if let Some(lbl) = ui.get_mut(status) {
            lbl.set_text("btn3 clicked — propagation stopped, outer won't fire");
        }
    });

    // outer click — only fires if propagation not stopped
    ui.on::<Rect, Click>(outer, move |ui, this, e| {
        println!(
            "outer click fired, propagation_stopped: {}",
            e.is_propagation_stopped()
        );
        if let Some(lbl) = ui.get_mut(status) {
            lbl.set_text("outer clicked — this shouldn't appear if btn3 was clicked");
        }
    });

    let mut app = App::new();
    app.open_window(
        WindowConfig {
            title: "event demo".to_string(),
            width: 500,
            height: 400,
            clear_color: Color::rgb(18, 18, 18),
        },
        ui,
    );
    app.run();
}
