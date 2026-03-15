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
    ui.get_mut(btn).unwrap().border_radius = 0.0;
    ui.get_mut(btn).unwrap().layout.padding = [5.0; 4];
    ui.connect(btn, Button::CLICKED, move |ui| {
        ui.get_mut(label).unwrap().text = "Clicked!".to_string();
    });

    ui.append(root, label);
    ui.append(root, btn);
    ui.set_root(root);

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
