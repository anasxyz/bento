use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = Column::new(&mut ui);
    ui[root].layout.width = Size::Percent(100.0);
    ui[root].layout.height = Size::Percent(100.0);
    ui[root].layout.padding = [40.0, 40.0, 40.0, 40.0];
    ui[root].layout.row_gap = 16.0;
    ui[root].bg_color = Some(Color::hex("181825"));

    let hint = Label::new(
        &mut ui,
        "Click the box to focus it, then type. Press Escape to clear.",
    );
    ui[hint].text_color = Color::hex("a6adc8");
    ui[hint].font_size = 14.0;

    let output = Label::new(&mut ui, "");
    ui[output].text_color = Color::hex("cdd6f4");
    ui[output].font_size = 24.0;

    // focusable box
    let box1 = Rect::new(&mut ui);
    ui[box1].bg_color = Color::hex("313244");
    ui[box1].border_radius = Some(8.0);
    ui[box1].layout_mut().width = Size::Fixed(300.0);
    ui[box1].layout_mut().height = Size::Fixed(48.0);

    ui.connect(box1, Rect::HOVERED, move |ui| {
        ui[box1].bg_color = Color::hex("45475a");
        ui[box1].border_color = Some(Color::hex("89b4fa"));
        ui[box1].border_thickness = 2.0;
    });
    ui.connect(box1, Rect::HOVER_END, move |ui| {
        if !ui[box1].focused {
            println!("hover ended and not focused");
            ui[box1].bg_color = Color::hex("313244");
            ui[box1].border_color = None;
            ui[box1].border_thickness = 0.0;
        }
    });
    // change color on focus
    ui.connect(box1, Rect::FOCUS_GAINED, move |ui| {
        println!("focus gained");
        ui[box1].bg_color = Color::hex("45475a");
        ui[box1].border_color = Some(Color::hex("89b4fa"));
        ui[box1].border_thickness = 2.0;
    });
    ui.connect(box1, Rect::FOCUS_LOST, move |ui| {
        ui[box1].bg_color = Color::hex("313244");
        ui[box1].border_color = None;
        ui[box1].border_thickness = 0.0;
    });

    // type into the output label when box is focused
    ui.connect_key(box1, move |ui, key, _mods, text| match key {
        Key::Backspace => {
            ui[output].text.pop();
        }
        _ => {
            if let Some(ch) = text {
                ui[output].text.push(ch);
            }
        }
    });

    // global — Escape always clears regardless of focus
    ui.connect_key_global(move |ui, key, _mods, _text| {
        if key == Key::Escape {
            ui[output].text.clear();
        }
    });

    ui.append(root, hint);
    ui.append(root, box1);
    ui.append(root, output);
    ui.set_root(root);

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
