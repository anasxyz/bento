use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = Column::new(&mut ui);
    ui[root].layout.width = Size::Percent(100.0);
    ui[root].layout.height = Size::Percent(100.0);
    ui[root].layout.padding = [40.0, 40.0, 40.0, 40.0];
    ui[root].layout.row_gap = 16.0;
    ui[root].bg_color = Some(Color::hex("181825"));

    let input = TextInput::new(&mut ui, "Type something...");
    ui[input].layout.width = Size::Fixed(300.0);

    let output = Label::new(&mut ui, "");
    ui[output].text_color = Color::hex("cdd6f4");
    ui[output].font_size = 18.0;

    let btn = Button::new(&mut ui, "Submit");
    ui[btn].color = Color::hex("89b4fa");

    ui.connect(input, TextInput::TEXT_CHANGED, move |ui| {
        ui[output].text = ui[input].text.clone();
    });

    ui.connect(input, TextInput::SUBMITTED, move |ui| {
        ui[output].text = format!("Submitted: {}", ui[input].text);
        ui[input].text.clear();
        ui[input].cursor_pos.set(0);
    });

    ui.connect(btn, Button::CLICKED, move |ui| {
        ui[output].text = format!("Submitted: {}", ui[input].text);
        ui[input].text.clear();
        ui[input].cursor_pos.set(0);
    });

    ui.connect_key_global(move |ui, key, _mods, _text| {
        if key == Key::Escape {
            ui[input].text.clear();
            ui[input].cursor_pos.set(0);
        }
    });

    ui.append(root, input);
    ui.append(root, output);
    ui.append(root, btn);
    ui.set_root(root);

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
