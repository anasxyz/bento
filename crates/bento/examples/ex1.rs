#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let button = ui.add(Button::new("Click me!", 100.0, 100.0, 100.0, 32.0));

    ui.listen(button, move |e: &Click, ui| {
        if e.button == MouseButton::Left {
            let old_w = ui.get(button).unwrap().w();
            ui.get_mut(button).unwrap().set_w(old_w + 10.0);
        }
    });

    ui.listen_once(button, move |e: &HoverEnter, ui| {
        println!("hover enter");
        ui.get_mut(button).unwrap().set_label("Hover meeeeeeeeeee!");
    });

    ui.listen_global(|e: &WindowResized, ui| {
        println!("window resized");
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
