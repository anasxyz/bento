#![allow(dead_code)]
#![allow(unused)]

use bento_ui::*;

fn main() {
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 0.0, 100.0, 100.0));
    println!("removed rect");
    println!("{}", ui);

    let rect_widget = ui.get(rect).unwrap();
    println!("{:#?}\n", rect_widget);

    ui.remove(rect);
    println!("removed rect");
    println!("{}", ui);
}
