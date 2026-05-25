#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let input = ui.add(Editor::new());

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
