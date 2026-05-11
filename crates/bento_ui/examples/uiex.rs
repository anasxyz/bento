#![allow(dead_code)]
#![allow(unused)]

use bento_ui::*;

fn main() {
    let mut ui = Ui::new();

    let rect = ui.add(Rect {});

    // debug
    // println!("{}", ui.scene());
    println!("{}", ui);
}
