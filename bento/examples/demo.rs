use bento::*;

fn main() {
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new());
    ui.get_mut(rect)
        .unwrap()
        .set_width(Size::Fixed(300.0))
        .set_height(Size::Fixed(200.0))
        .set_color(Color::rgb(100, 150, 255))
        .set_radius(8.0);
    ui.set_root(rect);

    let mut app = App::new();
    app.open_window(WindowConfig::default(), ui);
    app.run();
}
