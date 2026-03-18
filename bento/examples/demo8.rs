use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Row::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_align_items(AlignItems::Start)
        .set_justify_content(JustifyContent::Start)
        .set_bg_color(Some(Color::rgb(10, 10, 10)));

    let rect = ui.add(Container::new());
    ui.get_mut(rect)
        .unwrap()
        .set_width(Size::Fixed(150.0))
        .set_height(Size::Percent(100.0))
        .set_border([0.0, 1.0, 0.0, 0.0])
        .set_border_color(Some(Color::rgb(50, 50, 50)))
        .set_bg_color(Some(Color::rgb(10, 10, 10)));

    let label = ui.add(Label::new("Salma"));

    ui.append(root, rect);
    ui.append(root, label);
    ui.set_root(root);

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
