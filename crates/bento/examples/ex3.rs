#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut col = Group::new();
    col.layout = Layout::Column { gap: 4.0 };
    let col = ui.add(col);

    for i in 0..50 {
        let mut row = Group::new();
        row.layout = Layout::Row { gap: 4.0 };
        let row = ui.add(row);

        let btn1 = ui.add(Button::new(&format!("Button {}-1", i)));
        let btn2 = ui.add(Button::new(&format!("Button {}-2", i)));
        let btn3 = ui.add(Button::new(&format!("Button {}-3", i)));

        ui.append(row, btn1);
        ui.append(row, btn2);
        ui.append(row, btn3);
        ui.append(col, row);
    }

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
