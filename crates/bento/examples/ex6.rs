#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut row = Group::new();
    row.layout = Layout::Row { gap: 8.0 };
    let row = ui.add(row);

    let mut col = Group::new();
    col.layout = Layout::Column { gap: 8.0 };
    let col = ui.add(col);

    let editor = ui.add(Editor::new());

    let btn = ui.add(Button::new("below editor"));

    ui.append(col, editor);
    ui.append(col, btn);

    let btn2 = ui.add(Button::new("next to editor"));

    ui.append(row, col);
    ui.append(row, btn2);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
