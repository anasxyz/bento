use bento::*;
use std::cell::Cell;
use std::rc::Rc;

const THEME_CHANGED: u32 = 100;

fn main() {
    let mut ui = Ui::new();

    let root = Column::new(&mut ui);
    ui.get_mut(root).unwrap().layout.width = Size::Percent(100.0);
    ui.get_mut(root).unwrap().layout.height = Size::Percent(100.0);
    ui.get_mut(root).unwrap().layout.padding = [40.0, 40.0, 40.0, 40.0];
    ui.get_mut(root).unwrap().layout.row_gap = 16.0;
    ui.get_mut(root).unwrap().bg_color = Some(Color::hex("181825"));

    let status = Label::new(&mut ui, "Click a button inside the panel.");
    ui.get_mut(status).unwrap().text_color = Color::hex("cdd6f4");

    // --- bubbling demo ---
    let panel = Column::new(&mut ui);
    ui.get_mut(panel).unwrap().layout.row_gap = 8.0;
    ui.get_mut(panel).unwrap().layout.padding = [16.0, 16.0, 16.0, 16.0];
    ui.get_mut(panel).unwrap().bg_color = Some(Color::hex("1e1e2e"));
    ui.get_mut(panel).unwrap().border_radius = Some(8.0);

    let btn_a = Button::new(&mut ui, "Button A");
    let btn_b = Button::new(&mut ui, "Button B");
    ui.get_mut(btn_b).unwrap().color = Color::hex("a6e3a1");

    ui.append(panel, btn_a);
    ui.append(panel, btn_b);

    // connected on PANEL — bubbling makes this work
    ui.connect(panel, Button::CLICKED, move |ui| {
        ui.get_mut(status).unwrap().text =
            "A button inside the panel was clicked! (bubbled)".to_string();
    });

    // --- broadcast demo ---
    let is_light = Rc::new(Cell::new(false));

    let label_a = Label::new(&mut ui, "I react to theme changes");
    let label_b = Label::new(&mut ui, "Me too!");
    ui.get_mut(label_a).unwrap().text_color = Color::hex("cdd6f4");
    ui.get_mut(label_b).unwrap().text_color = Color::hex("cdd6f4");

    let is_light_a = is_light.clone();
    ui.connect(label_a, THEME_CHANGED, move |ui| {
        ui.get_mut(label_a).unwrap().text_color = if is_light_a.get() {
            Color::hex("cdd6f4")
        } else {
            Color::hex("4c4f69")
        };
    });

    let is_light_b = is_light.clone();
    ui.connect(label_b, THEME_CHANGED, move |ui| {
        ui.get_mut(label_b).unwrap().text_color = if is_light_b.get() {
            Color::hex("cdd6f4")
        } else {
            Color::hex("4c4f69")
        };
    });

    let is_light_root = is_light.clone();
    ui.connect(root, THEME_CHANGED, move |ui| {
        ui.get_mut(root).unwrap().bg_color = Some(if is_light_root.get() {
            Color::hex("181825")
        } else {
            Color::hex("eff1f5")
        });
    });

    let btn_theme = Button::new(&mut ui, "Toggle Theme");
    ui.get_mut(btn_theme).unwrap().color = Color::hex("cba6f7");
    ui.connect(btn_theme, Button::CLICKED, move |ui| {
        is_light.set(!is_light.get());
        ui.broadcast(THEME_CHANGED);
    });

    ui.append(root, status);
    ui.append(root, panel);
    ui.append(root, label_a);
    ui.append(root, label_b);
    ui.append(root, btn_theme);
    ui.set_root(root);

    AppWindow::new(WindowConfig {
        title: "signal demo".to_string(),
        width: 600,
        height: 400,
        clear_color: Color::hex("181825"),
    })
    .run(ui, |_ui| {});
}
