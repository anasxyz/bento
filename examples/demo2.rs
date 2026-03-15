use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = Column::new(&mut ui);
    ui.get_mut(root).unwrap().layout.width = Size::Percent(100.0);
    ui.get_mut(root).unwrap().layout.height = Size::Percent(100.0);
    ui.get_mut(root).unwrap().layout.padding = [40.0, 40.0, 40.0, 40.0];
    ui.get_mut(root).unwrap().layout.row_gap = 16.0;
    ui.get_mut(root).unwrap().bg_color = Some(Color::hex("181825"));

    let status = Label::new(&mut ui, "Press a button...");
    ui.get_mut(status).unwrap().font_size = 18.0;
    ui.get_mut(status).unwrap().text_color = Color::hex("cdd6f4");

    let btn_a = Button::new(&mut ui, "Button A");
    ui.connect(btn_a, Button::CLICKED, move |ui| {
        ui.get_mut(status).unwrap().text = "Button A clicked!".to_string();
    });

    let btn_b = Button::new(&mut ui, "Button B");
    ui.get_mut(btn_b).unwrap().color = Color::hex("a6e3a1");
    ui.get_mut(btn_b).unwrap().layout.width = Size::Fixed(160.0);
    ui.connect(btn_b, Button::CLICKED, move |ui| {
        ui.get_mut(status).unwrap().text = "Button B clicked!".to_string();
    });

    let btn_c = Button::new(&mut ui, "Hover me");
    ui.get_mut(btn_c).unwrap().color = Color::hex("f38ba8");
    ui.connect(btn_c, Button::HOVERED, move |ui| {
        ui.get_mut(btn_a).unwrap().color = Color::hex("fab387");
    });
    ui.connect(btn_c, Button::HOVERED, move |ui| {
        println!("hovered");
    });
    ui.connect(btn_c, Button::HOVER_END, move |ui| {
        ui.get_mut(btn_a).unwrap().color = Color::rgb(70, 70, 200);
    });
    ui.connect(btn_c, Button::CLICKED, move |ui| {
        ui.get_mut(status).unwrap().text = "Button C clicked!".to_string();
    });

    let counter_label = Label::new(&mut ui, "Count: 0");
    ui.get_mut(counter_label).unwrap().font_size = 14.0;
    ui.get_mut(counter_label).unwrap().text_color = Color::hex("a6adc8");

    let btn_count = Button::new(&mut ui, "Increment");
    ui.get_mut(btn_count).unwrap().color = Color::hex("cba6f7");

    let count = std::rc::Rc::new(std::cell::Cell::new(0u32));
    ui.connect(btn_count, Button::CLICKED, move |ui| {
        count.set(count.get() + 1);
        ui.get_mut(counter_label).unwrap().text = format!("Count: {}", count.get());
    });

    ui.append(root, status);
    ui.append(root, btn_a);
    ui.append(root, btn_b);
    ui.append(root, btn_c);
    ui.append(root, counter_label);
    ui.append(root, btn_count);
    ui.set_root(root);

    AppWindow::new(WindowConfig {
        title: "bento demo".to_string(),
        width: 600,
        height: 400,
        clear_color: Color::hex("181825"),
    })
    .run(ui, |_ui| {});
}
