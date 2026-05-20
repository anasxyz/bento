#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = Rect::new(&mut ui, 100.0, 100.0, 200.0, 100.0);
    rect.set_color(&mut ui, [1.0, 0.0, 0.0, 1.0]);

    let text = Text::new(&mut ui, "Hello", 100.0, 220.0, 32.0);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
