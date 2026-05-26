#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut row = Group::new();
    row.layout = Layout::Row { gap: 0.0 };
    row.width = Size::Fill;
    row.height = Size::Fill;
    let row = ui.add(row);

    let editor = ui.add(Editor::new());
    ui.set(editor, |e| {
        e.set_font_size(20.0);
        e.color = [0.88, 0.88, 0.88, 1.0];
        e.font_family = "Iosevka Nerd Font Mono".to_string();
        e.width = Size::Fill;
        e.height = Size::Fill;
        e.use_spaces = true;
        e.wrap = false;
    });

    ui.append(row, editor);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
