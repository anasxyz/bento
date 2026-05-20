#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let text = ui.add(Text::new("Hello, world!", 100.0, 100.0, 20.0));

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
