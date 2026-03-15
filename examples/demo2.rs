use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = Column::new(&mut ui);
    ui.get_mut(root).unwrap().layout.width = Size::Percent(100.0);
    ui.get_mut(root).unwrap().layout.height = Size::Percent(100.0);
    ui.get_mut(root).unwrap().layout.padding = [40.0, 40.0, 40.0, 40.0];
    ui.get_mut(root).unwrap().layout.row_gap = 12.0;
    ui.get_mut(root).unwrap().bg_color = Some(Color::hex("181825"));

    let label = Label::new(&mut ui, "Press the button");
    ui.get_mut(label).unwrap().text_color = Color::hex("cdd6f4");

    let btn = Button::new(&mut ui, "Click me");
    ui.get_mut(btn).unwrap().text_color = Color::hex("1e1e2e");
    ui.get_mut(btn).unwrap().font_family = "monospace".to_string();
    ui.get_mut(btn).unwrap().border_color = Some(Color::hex("cba6f7"));
    ui.get_mut(btn).unwrap().border_thickness = 2.0;
    ui.get_mut(btn).unwrap().disabled = false; // grays out + blocks all interaction
    ui.get_mut(btn).unwrap().layout.padding = [12.0, 24.0, 12.0, 24.0];
    ui.get_mut(btn).unwrap().layout.margin = [8.0, 8.0, 8.0, 100.0];
    ui.connect(btn, Button::CLICKED, move |ui| {
        ui.get_mut(label).unwrap().text = "Clicked!".to_string();
    });

    ui.append(root, label);
    ui.append(root, btn);
    ui.set_root(root);

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
