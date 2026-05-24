#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();
    ui.debug(true);

    let mut col = Group::new();
    col.layout = Layout::Column { gap: 4.0 };
    let col = ui.add(col);

    for i in 0..50 {
        let mut row = Group::new();
        row.layout = Layout::Row { gap: 4.0 };
        let row = ui.add(row);

        let mut r1 = Rect::new(80.0, 32.0);
        r1.set_color([0.2, 0.3, 0.8, 1.0]);
        let mut r2 = Rect::new(80.0, 32.0);
        r2.set_color([0.8, 0.2, 0.3, 1.0]);
        let mut r3 = Rect::new(80.0, 32.0);
        r3.set_color([0.2, 0.8, 0.3, 1.0]);

        let r1 = ui.add(r1);
        let r2 = ui.add(r2);
        let r3 = ui.add(r3);

        ui.append(row, r1);
        ui.append(row, r2);
        ui.append(row, r3);
        ui.append(col, row);
    }

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
