#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let input = ui.add(Input::new(100.0, 100.0, 300.0, 40.0).placeholder("Type here..."));

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
