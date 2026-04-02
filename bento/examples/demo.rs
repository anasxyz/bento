use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_flex_wrap(FlexWrap::Wrap);
    ui.set_root(root);

    for i in 0..5000 {
        let label = ui.add(Label::new(&format!("item {}", i)));
        ui.get_mut(label)
            .unwrap()
            .set_size(12.0)
            .set_margin([2.0, 2.0, 2.0, 2.0]);
        ui.append(root, label);
    }

    let mut app = App::new();
    app.open_window(WindowConfig::default(), ui);
    app.run();
}
