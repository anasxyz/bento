#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let button = ui.add(Button::new("Click me!", 100.0, 100.0, 100.0, 32.0));

    ui.listen(button, move |e: &Click, ui| {
        let old_w = ui.get(button).unwrap().w();
        ui.get_mut(button).unwrap().set_w(old_w + 10.0);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
