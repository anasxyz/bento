use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let group = ui.add(Group::new());
    ui.get_mut(group).unwrap()
        .set_flex_direction(FlexDirection::Column)
        .set_gap([8.0, 0.0])
        .set_padding([16.0; 4]);

    // fixed px
    let r1 = ui.add_to(group, Rect::new());
    ui.get_mut(r1).unwrap()
        .set_width(Size::Px(200.0))
        .set_height(Size::Px(80.0))
        .set_color([1.0, 0.0, 0.0, 1.0]);

    // 50% width
    let r2 = ui.add_to(group, Rect::new());
    ui.get_mut(r2).unwrap()
        .set_width(Size::Percent(0.5))
        .set_height(Size::Px(80.0))
        .set_color([0.0, 1.0, 0.0, 1.0]);

    // auto — fills via flex_grow
    let r3 = ui.add_to(group, Rect::new());
    ui.get_mut(r3).unwrap()
        .set_flex_grow(1.0)
        .set_height(Size::Px(80.0))
        .set_color([0.0, 0.0, 1.0, 1.0]);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
