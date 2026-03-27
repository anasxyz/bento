use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Rect::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_flex_wrap(FlexWrap::Wrap);
    ui.set_root(root);

    let rect = ui.add(Rect::new());
    ui.get_mut(rect)
        .unwrap()
        .set_width(Size::Fixed(400.0))
        .set_height(Size::Fixed(400.0))
        .set_color(Color::rgb(30, 30, 30))
        .set_padding([16.0, 16.0, 16.0, 16.0]);
    ui.append(root, rect);

    let mut app = App::new();
    app.open_window(WindowConfig::default(), ui);
    app.run();
}
