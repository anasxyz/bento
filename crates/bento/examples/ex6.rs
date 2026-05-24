#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut col = Group::new();
    col.layout = Layout::Row { gap: 8.0 };
    col.width = Size::Fixed(150.0);
    let col = ui.add(col);

    let mut btn = Button::new("First button");
    btn.width = Size::Fill;
    let btn = ui.add(btn);

    let mut btn2 = Button::new("Second button");
    btn2.width = Size::Fill;
    let btn2 = ui.add(btn2);

    ui.append(col, btn);
    ui.append(col, btn2);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
