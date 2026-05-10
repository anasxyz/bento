use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let group = ui.add(Group::new());
    ui.get_mut(group).unwrap()
        .set_flex_direction(FlexDirection::Row)
        .set_gap([0.0, 8.0])
        .set_padding([8.0; 4]);

    let left = ui.add_to(group, Rect::new());
    ui.get_mut(left).unwrap()
        .set_flex_grow(1.0)
        .set_color([0.8, 0.2, 0.2, 1.0]);

    let right = ui.add_to(group, Rect::new());
    ui.get_mut(right).unwrap()
        .set_flex_grow(1.0)
        .set_color([0.2, 0.2, 0.8, 1.0]);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
