use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Rect::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(13, 13, 15))
        .set_flex_direction(FlexDirection::Row)
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center)
        .set_col_gap(40.0);
    ui.set_root(root);

    // left box — overflow hidden
    // the purple child is 300x300 but the box is only 150x150
    // the child should be clipped to the box bounds
    let box_hidden = ui.add(Rect::new());
    ui.get_mut(box_hidden)
        .unwrap()
        .set_width(Size::Fixed(150.0))
        .set_height(Size::Fixed(150.0))
        .set_color(Color::rgb(40, 40, 40))
        .set_overflow(Overflow::Hidden);
    ui.append(root, box_hidden);

    let lbl = ui.add(Label::new("This text is very wide and should be clipped"));
    ui.get_mut(lbl)
        .unwrap()
        .set_size(20.0)
        .set_color(Color::WHITE)
        .set_wrap(false)
        .set_width(Size::Fixed(400.0));
    ui.append(box_hidden, lbl); // directly inside the hidden box

    // right box — overflow visible
    // the red child is 300x300 but the box is only 150x150
    // the child should spill outside the box bounds
    let box_visible = ui.add(Rect::new());
    ui.get_mut(box_visible)
        .unwrap()
        .set_width(Size::Fixed(150.0))
        .set_height(Size::Fixed(150.0))
        .set_color(Color::rgb(40, 40, 40))
        .set_overflow(Overflow::Visible);
    ui.append(root, box_visible);

    let child_visible = ui.add(Rect::new());
    ui.get_mut(child_visible)
        .unwrap()
        .set_width(Size::Fixed(300.0))
        .set_height(Size::Fixed(300.0))
        .set_color(Color::rgb(220, 80, 80));
    ui.append(box_visible, child_visible);

    let mut app = App::new();
    app.open_window(
        WindowConfig {
            title: "Overflow test".to_string(),
            width: 600,
            height: 500,
            clear_color: Color::rgb(13, 13, 15),
        },
        ui,
    );
    app.run();
}
