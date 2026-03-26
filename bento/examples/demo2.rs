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

    let label = ui.add(Label::new("hello"));
    ui.get_mut(label)
        .unwrap()
        .set_size(14.0);
    ui.append(root, label);

    let label2 = ui.add(Label::new("hello2"));
    ui.get_mut(label2)
        .unwrap()
        .set_size(14.0);
    ui.append(root, label2);

    let mut app = App::new();
    app.open_window(WindowConfig::default(), ui);
    app.run();
}
