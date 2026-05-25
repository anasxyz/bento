#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut col = Group::new();
    col.layout = Layout::Column { gap: 8.0 };
    col.width = Size::Fill;
    col.height = Size::Fill;
    let col = ui.add(col);

    let mut input = MultilineInput::new();
    input.width = Size::Fill;
    input.height = Size::Fill;
    let input = ui.add(input);

    ui.append(col, input);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
