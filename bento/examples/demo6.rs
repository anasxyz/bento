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

    let input = ui.add_to(root, TextInput::new());
    ui.get_mut(input)
        .unwrap()
        .set_width(Size::Fixed(280.0))
        .set_placeholder("Type something...");

    let label = ui.add_to(root, Label::new(""));
    ui.get_mut(label)
        .unwrap()
        .set_width(Size::Fixed(280.0))
        .set_size(13.0)
        .set_color(Color::rgb(140, 140, 140));

    let btn = ui.add_to(root, Button::new("Submit"));
    ui.get_mut(btn)
        .unwrap()
        .set_width(Size::Fixed(280.0))
        .set_color(Color::rgb(99, 102, 241));

    ui.on_change(input, move |ui, this, e| {
        if let Some(lbl) = ui.get_mut(label) {
            lbl.set_text(&format!("{} chars", e.value.len()));
        }
    });

    ui.on_click(btn, move |ui, this, e| {
        if let Some(inp) = ui.get_mut(input) {
            inp.set_value("");
        }
        if let Some(lbl) = ui.get_mut(label) {
            lbl.set_text("Cleared!");
        }
    });

    let mut app = App::new();
    app.open_window(
        WindowConfig {
            title: "demo".to_string(),
            width: 400,
            height: 300,
            clear_color: Color::rgb(18, 18, 18),
        },
        ui,
    );
    app.run();
}
