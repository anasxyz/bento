use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Column::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_bg_color(Some(Color::rgb(20, 20, 20)));

    let editor = ui.add(TextArea::new());
    ui.get_mut(editor)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_bg_color(Some(Color::rgb(10, 10, 10)))
        .set_text_color(Color::WHITE)
        .set_font_size(16.0)
        .set_font_family("ZedMono Nerd Font")
        .set_padding([12.0, 16.0, 12.0, 16.0])
        .set_placeholder("Start typing...");

    ui.connect(editor, |_ui, _event| {});

    ui.append(root, editor);
    ui.set_root(root);

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
