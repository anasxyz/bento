#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let group = ui.add(Group::new());
    ui.set(group, |w| {
        w.width = Size::Fixed(200.0);
        w.height = Size::Fixed(200.0);
    });

    let input = ui.add(Editor::new());
    ui.set(input, |w| {
        w.width = Size::Fill;
        w.height = Size::Fill;
    });

    ui.append(group, input);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
