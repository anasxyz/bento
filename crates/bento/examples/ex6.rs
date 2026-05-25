#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut col = Group::new();
    col.layout = Layout::None;
    col.width = Size::Fixed(300.0);
    let col = ui.add(col);

    let mut input = MultilineInput::new();
    input.width = Size::Fill;
    let input = ui.add(input);

    ui.append(col, input);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
