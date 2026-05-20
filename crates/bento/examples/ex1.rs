#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let container = ui.add(Container::new(100.0, 100.0, 400.0, 300.0));
    let rect = ui.add(Rect::new(10.0, 10.0, 100.0, 50.0));
    ui.append(container, rect);

    let slider = ui.add(Slider::new(0.0, 0.0, 100.0, 20.0));
    ui.append(container, slider);

    ui.listen_global(move |e: &KeyPress, ui| {
        match e.key {
            Key::D => {
                ui.print_tree();
            }
            Key::S => {
                ui.scene().print_tree();
            }
            _ => {}
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
