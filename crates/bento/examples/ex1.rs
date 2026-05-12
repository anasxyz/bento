#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 0.0, 100.0, 100.0));

    ui.listen(rect, move |e: &FocusGained, ui| {
        println!("focus gained");
    });

    ui.listen(rect, move |e: &FocusLost, ui| {
        println!("focus lost");
    });

    ui.listen_any(move |e: &KeyPress, ui| {
        let old_x = ui.get(rect).unwrap().x();
        let old_y = ui.get(rect).unwrap().y();
        match e.key {
            Key::A => ui.get_mut(rect).unwrap().set_x(old_x - 10.0),
            Key::D => ui.get_mut(rect).unwrap().set_x(old_x + 10.0),
            Key::W => ui.get_mut(rect).unwrap().set_y(old_y - 10.0),
            Key::S => ui.get_mut(rect).unwrap().set_y(old_y + 10.0),
            _ => {}
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
