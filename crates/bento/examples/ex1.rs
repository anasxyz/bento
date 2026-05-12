#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 0.0, 100.0, 100.0));
    ui.get_mut(rect).unwrap().set_hoverable(false);

    ui.listen(rect, move |e: &HoverEnter, ui| {
        println!("hover gained");
    });

    ui.listen(rect, move |e: &HoverLeave, ui| {
        println!("hover lost");
    });

    ui.listen_any(move |e: &KeyPress, ui| {
        if ui.get(rect).unwrap().is_hovered() {
            println!("hovered");
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
