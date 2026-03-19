use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Row::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0));

    // sidebar
    let sidebar = ui.add(Column::new());
    ui.get_mut(sidebar)
        .unwrap()
        .set_width(Size::Fixed(200.0))
        .set_height(Size::Percent(100.0))
        .set_bg_color(Some(Color::rgb(24, 24, 24)))
        .set_padding([16.0, 0.0, 16.0, 0.0])
        .set_row_gap(4.0);

    for (i, label) in [
        "General",
        "Appearance",
        "Keybindings",
        "Extensions",
        "Privacy",
        "About",
    ]
    .iter()
    .enumerate()
    {
        let item = ui.add(Rect::new());
        ui.get_mut(item)
            .unwrap()
            .set_width(Size::Percent(100.0))
            .set_height(Size::Fixed(36.0))
            .set_bg_color(if i == 0 {
                Color::rgb(55, 55, 55)
            } else {
                Color::rgb(24, 24, 24)
            })
            .set_border_radius(Some(6.0));

        let text = ui.add(Label::new(label));
        ui.get_mut(text)
            .unwrap()
            .set_font_size(13.0)
            .set_text_color(if i == 0 {
                Color::WHITE
            } else {
                Color::rgb(180, 180, 180)
            })
            .set_margin([10.0, 0.0, 0.0, 12.0]);

        ui.append(item, text);
        ui.append(sidebar, item);
    }

    // scrollable main content
    let scroll = ui.add(ScrollContainer::new());
    ui.get_mut(scroll)
        .unwrap()
        .set_flex_grow(1.0)
        .set_height(Size::Percent(100.0))
        .set_bg_color(Some(Color::rgb(30, 30, 30)));

    let content = ui.add(Column::new());
    ui.get_mut(content)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_padding([24.0, 24.0, 24.0, 24.0])
        .set_row_gap(16.0);

    let title = ui.add(Label::new("General"));
    ui.get_mut(title)
        .unwrap()
        .set_font_size(20.0)
        .set_font_weight(600)
        .set_text_color(Color::WHITE);
    ui.append(content, title);

    for (name, desc) in [
        ("Auto Save", "Save files automatically after a delay"),
        ("Format on Save", "Run formatter when saving a file"),
        ("Word Wrap", "Wrap long lines to fit the editor width"),
        ("Show Line Numbers", "Display line numbers in the gutter"),
        ("Minimap", "Show a minimap overview of the file"),
        ("Telemetry", "Send usage data to improve the product"),
        ("Hardware Acceleration", "Use GPU rendering where available"),
        ("Smooth Scrolling", "Animate scrolling for a smoother feel"),
        ("Auto Save", "Save files automatically after a delay"),
        ("Format on Save", "Run formatter when saving a file"),
        ("Word Wrap", "Wrap long lines to fit the editor width"),
        ("Show Line Numbers", "Display line numbers in the gutter"),
        ("Minimap", "Show a minimap overview of the file"),
        ("Telemetry", "Send usage data to improve the product"),
        ("Hardware Acceleration", "Use GPU rendering where available"),
        ("Smooth Scrolling", "Animate scrolling for a smoother feel"),
    ] {
        let row = ui.add(Rect::new());
        ui.get_mut(row)
            .unwrap()
            .set_width(Size::Percent(100.0))
            .set_height(Size::Fixed(64.0))
            .set_bg_color(Color::rgb(38, 38, 38))
            .set_border_radius(Some(8.0));

        let label = ui.add(Label::new(name));
        ui.get_mut(label)
            .unwrap()
            .set_font_size(13.0)
            .set_font_weight(500)
            .set_text_color(Color::WHITE)
            .set_margin([12.0, 0.0, 0.0, 16.0]);

        let desc = ui.add(Label::new(desc));
        ui.get_mut(desc)
            .unwrap()
            .set_font_size(12.0)
            .set_text_color(Color::rgb(140, 140, 140))
            .set_margin([4.0, 0.0, 0.0, 16.0]);

        ui.append(row, label);
        ui.append(row, desc);
        ui.append(content, row);
    }

    ui.append(scroll, content);
    ui.append(root, sidebar);
    ui.append(root, scroll);
    ui.set_root(root);

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
