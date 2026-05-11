#![allow(dead_code)]
#![allow(unused)]

use bento_ui::*;

fn main() {
    let mut ui = Ui::new();

    let rect = ui.add(Rect {});
    println!("{}", ui);

    ui.remove(rect);
    println!("{}", ui);
}
