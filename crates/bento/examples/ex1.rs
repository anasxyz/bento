#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let container = ui.add(Container::new(400.0, 400.0, 200.0, 100.0));
    let c = ui.get_mut(container).unwrap();
    c.set_color([0.3, 0.3, 0.3, 1.0]);
    c.set_clip(true);

    let btn1 = ui.add(Button::new("One", 0.0, 0.0, 100.0, 50.0));
    let btn2 = ui.add(Button::new("Two", 0.0, 60.0, 100.0, 50.0));

    ui.set_children(container, [btn1, btn2]);

    ui.listen(btn1, move |_e: &Click, ui| {
        ui.remove(btn1);
    });

    ui.listen(btn2, move |_e: &Click, ui| {
        let new_button = ui.add(Button::new("One", 0.0, 0.0, 100.0, 50.0));
        ui.set_children(container, [new_button]);
    });

    ui.listen(container, move |e: &MouseScroll, ui| {
        if let Some(c) = ui.get_mut(container) {
            c.set_offset(0.0, c.offset_y() + e.y * 20.0);
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
