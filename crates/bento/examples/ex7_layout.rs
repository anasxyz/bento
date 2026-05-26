#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();
    ui.debug(true);

    let mut row = Group::new();
    row.layout = Layout::Row { gap: 8.0 };
    row.height = Size::Fill;
    let row = ui.add(row);

    let mut col = Group::new();
    col.layout = Layout::Column { gap: 8.0 };
    col.width = Size::Auto;
    let col = ui.add(col);

    let editor = ui.add(Editor::new());

    let mut bottom = Rect::new(0.0, 40.0);
    bottom.width = Size::Fixed(400.0);
    bottom.height = Size::Fill;
    bottom.set_color([0.2, 0.4, 0.2, 1.0]);
    let bottom = ui.add(bottom);

    ui.append(col, editor);
    ui.append(col, bottom);

    let mut sidebar = Rect::new(120.0, 0.0);
    sidebar.width = Size::Fixed(120.0);
    sidebar.height = Size::Fill;
    sidebar.set_color([0.3, 0.2, 0.4, 1.0]);
    let sidebar = ui.add(sidebar);

    ui.append(row, col);
    ui.append(row, sidebar);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
