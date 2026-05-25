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

    let mut input = TextInput::new();
    input.width = Size::Fill;
    let input = ui.add(input);

    ui.asyncs.timer(0.5, move |ui| {
        println!("timer");
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
