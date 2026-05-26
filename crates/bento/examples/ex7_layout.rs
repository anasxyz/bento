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

    // left sidebar
    let mut sidebar = Rect::new(200.0, 0.0);
    sidebar.width = Size::Fixed(200.0);
    sidebar.height = Size::Fill;
    sidebar.set_color([0.12, 0.12, 0.15, 1.0]);
    let sidebar = ui.add(sidebar);

    // main col: editor + status bar
    let mut col = Group::new();
    col.layout = Layout::Column { gap: 0.0 };
    col.width = Size::Fill;
    col.height = Size::Fill;
    let col = ui.add(col);

    let editor = ui.add(Editor::new());
    ui.set(editor, |e| {
        e.lines = vec!["hello".to_string(), "world💁👌🎍😍".to_string()];
        e.width = Size::Fill;
        e.height = Size::Fill;
    });

    let mut status = Rect::new(0.0, 24.0);
    status.width = Size::Fill;
    status.height = Size::Fixed(24.0);
    status.set_color([0.15, 0.15, 0.18, 1.0]);
    let status = ui.add(status);

    ui.append(col, editor);
    ui.append(col, status);

    ui.append(row, sidebar);
    ui.append(row, col);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
