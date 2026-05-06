use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(13, 13, 15))
        .set_flex_direction(FlexDirection::Col)
        .set_padding([20.0, 20.0, 20.0, 20.0])
        .set_row_gap(24.0);
    ui.set_root(root);

    // ── 1. bare labels in col ─────────────────────────────────────────────────
    let s1 = ui.add(Label::new("1. BARE LABELS IN COL"));
    ui.get_mut(s1)
        .unwrap()
        .set_size(11.0)
        .set_weight(600)
        .set_color(Color::rgb(100, 100, 120));
    ui.append(root, s1);

    let row1 = ui.add(Container::new());
    ui.get_mut(row1)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_flex_direction(FlexDirection::Col)
        .set_color(Color::rgb(25, 25, 30))
        .set_padding([8.0, 8.0, 8.0, 8.0])
        .set_row_gap(4.0);
    ui.append(root, row1);

    for text in [
        "Short label",
        "A slightly longer label text here",
        "This is a much longer label that should wrap when the container is narrow enough to constrain it",
    ] {
        let lbl = ui.add(Label::new(text));
        ui.get_mut(lbl)
            .unwrap()
            .set_size(14.0)
            .set_color(Color::WHITE);
        ui.append(row1, lbl);
    }

    // ── 2. labels in row with wrap ───────────────────────────────────────────
    let s2 = ui.add(Label::new("2. LABELS IN ROW (flex wrap)"));
    ui.get_mut(s2)
        .unwrap()
        .set_size(11.0)
        .set_weight(600)
        .set_color(Color::rgb(100, 100, 120));
    ui.append(root, s2);

    let row2 = ui.add(Container::new());
    ui.get_mut(row2)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_flex_direction(FlexDirection::Row)
        .set_flex_wrap(FlexWrap::Wrap)
        .set_color(Color::rgb(25, 25, 30))
        .set_padding([8.0, 8.0, 8.0, 8.0])
        .set_col_gap(12.0)
        .set_row_gap(4.0);
    ui.append(root, row2);

    for text in [
        "First",
        "Second",
        "Third label",
        "Fourth",
        "Fifth label here",
    ] {
        let lbl = ui.add(Label::new(text));
        ui.get_mut(lbl)
            .unwrap()
            .set_size(14.0)
            .set_color(Color::WHITE);
        ui.append(row2, lbl);
    }

    // ── 3. label next to checkbox ────────────────────────────────────────────
    let s3 = ui.add(Label::new("3. LABEL NEXT TO CHECKBOX"));
    ui.get_mut(s3)
        .unwrap()
        .set_size(11.0)
        .set_weight(600)
        .set_color(Color::rgb(100, 100, 120));
    ui.append(root, s3);

    let row3 = ui.add(Container::new());
    ui.get_mut(row3)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_flex_direction(FlexDirection::Row)
        .set_align_items(AlignItems::Center)
        .set_color(Color::rgb(25, 25, 30))
        .set_padding([8.0, 8.0, 8.0, 8.0])
        .set_col_gap(8.0);
    ui.append(root, row3);

    let cb = ui.add(Checkbox::new());
    ui.append(row3, cb);

    let lbl = ui.add(Label::new(
        "Label next to a checkbox — should sit on one line",
    ));
    ui.get_mut(lbl)
        .unwrap()
        .set_size(14.0)
        .set_color(Color::WHITE);
    ui.append(row3, lbl);

    // ── 4. centered col ──────────────────────────────────────────────────────
    let s4 = ui.add(Label::new("4. CENTERED COL"));
    ui.get_mut(s4)
        .unwrap()
        .set_size(11.0)
        .set_weight(600)
        .set_color(Color::rgb(100, 100, 120));
    ui.append(root, s4);

    let row4 = ui.add(Container::new());
    ui.get_mut(row4)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_flex_direction(FlexDirection::Col)
        .set_align_items(AlignItems::Center)
        .set_color(Color::rgb(25, 25, 30))
        .set_padding([8.0, 8.0, 8.0, 8.0])
        .set_row_gap(4.0);
    ui.append(root, row4);

    for text in [
        "Short",
        "Medium length label",
        "A longer label to test centering behavior",
    ] {
        let lbl = ui.add(Label::new(text));
        ui.get_mut(lbl)
            .unwrap()
            .set_size(14.0)
            .set_color(Color::WHITE);
        ui.append(row4, lbl);
    }

    // ── 5. fixed width container ─────────────────────────────────────────────
    let s5 = ui.add(Label::new("5. FIXED WIDTH CONTAINER (200px)"));
    ui.get_mut(s5)
        .unwrap()
        .set_size(11.0)
        .set_weight(600)
        .set_color(Color::rgb(100, 100, 120));
    ui.append(root, s5);

    let row5 = ui.add(Container::new());
    ui.get_mut(row5)
        .unwrap()
        .set_flex_direction(FlexDirection::Col)
        .set_width(Size::Fixed(200.0))
        .set_color(Color::rgb(25, 25, 30))
        .set_padding([8.0, 8.0, 8.0, 8.0])
        .set_row_gap(4.0);
    ui.append(root, row5);

    let lbl = ui.add(Label::new(
        "This label should wrap inside the 200px container naturally",
    ));
    ui.get_mut(lbl)
        .unwrap()
        .set_size(14.0)
        .set_color(Color::WHITE);
    ui.append(row5, lbl);

    let mut app = App::new();
    app.open_window(
        WindowConfig {
            title: "demo".to_string(),
            width: 700,
            height: 700,
            clear_color: Color::rgb(13, 13, 15),
        },
        ui,
    );
    app.run();
}
