use bento::*;

fn main() {
    let mut ui = Ui::new();
    let root = ui.add(Container::new());
    ui.set_root(root);

    let size = 20.0;

    let rect = ui.add(Rect::new());
    ui[rect].layout.width = Size::Fixed(size);
    ui[rect].layout.height = Size::Fixed(size);
    ui[rect].bg_color = Color::RED;
    ui[rect].layout.position = Position::Absolute;
    ui.append(root, rect);

    AppWindow::new(WindowConfig::default()).run(ui, move |ui| {
        let mx = ui.mouse.x;
        let my = ui.mouse.y;
        ui[rect].layout.inset[0] = Size::Fixed(my - size / 2.0);
        ui[rect].layout.inset[3] = Size::Fixed(mx - size / 2.0);
    });
}
