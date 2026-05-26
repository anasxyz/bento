#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut row = Group::new();
    row.layout = Layout::Row { gap: 8.0 };
    let row = ui.add(row);

    let input = ui.add(LineInput::new());
    ui.append(row, input);
    

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
