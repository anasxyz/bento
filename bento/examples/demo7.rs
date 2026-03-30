use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui.get_mut(root).unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(18, 18, 18))
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center)
        .set_flex_direction(FlexDirection::Col)
        .set_row_gap(16.0);
    ui.set_root(root);

    let status = ui.add_to(root, Label::new("nothing checked"));
    ui.get_mut(status).unwrap()
        .set_size(13.0)
        .set_color(Color::rgb(140, 140, 140));

    for label_text in ["option a", "option b", "option c"] {
        let row = ui.add_to(root, Container::new());
        ui.get_mut(row).unwrap()
            .set_flex_direction(FlexDirection::Row)
            .set_align_items(AlignItems::Center)
            .set_col_gap(8.0);

        let cb = ui.add_to(row, Checkbox::new());
        ui.get_mut(cb).unwrap()
            .set_checked_color(Color::rgb(99, 102, 241));

        let lbl = ui.add_to(row, Label::new(label_text));
        ui.get_mut(lbl).unwrap()
            .set_size(14.0)
            .set_color(Color::rgb(200, 200, 200));

        ui.on::<Checkbox, Change>(cb, move |ui, this, e| {
            if let Some(s) = ui.get_mut(status) {
                s.set_text(&format!(
                    "{} is {}",
                    label_text,
                    if this.checked { "checked" } else { "unchecked" }
                ));
            }
        });
    }

    let mut app = App::new();
    app.open_window(WindowConfig {
        title: "Checkbox demo".to_string(),
        width: 300,
        height: 250,
        clear_color: Color::rgb(18, 18, 18),
    }, ui);
    app.run();
}
