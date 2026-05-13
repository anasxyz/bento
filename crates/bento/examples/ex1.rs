#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let container = ui.add(Container::new(50.0, 50.0, 300.0, 200.0));
    ui.get_mut(container)
        .unwrap()
        .set_color([0.3, 0.3, 0.3, 1.0]);

    let btn1 = ui.add(Button::new("One", 0.0, 0.0, 100.0, 50.0));
    let btn2 = ui.add(Button::new("Two", 150.0, 150.0, 100.0, 50.0));

    ui.set_children(container, [btn1, btn2]);

    ui.listen(btn1, move |_e: &Click, ui| {
        ui.remove(btn1);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
