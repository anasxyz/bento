#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut input = MultilineInput::new();
    input.set_position(400.0, 200.0);
    let input = ui.add(input);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
