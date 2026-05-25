#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();
    ui.debug = true;

    // full width column
    let mut col = Group::new();
    col.layout = Layout::Column { gap: 8.0 };
    col.width = Size::Fill;
    let col = ui.add(col);

    // full width rect - should stretch across window
    let mut r1 = Rect::new(0.0, 40.0);
    r1.width = Size::Fill;
    r1.height = Size::Fixed(40.0);
    r1.set_color([0.8, 0.2, 0.2, 1.0]);
    let r1 = ui.add(r1);

    // half width rect
    let mut r2 = Rect::new(0.0, 40.0);
    r2.width = Size::Percent(50.0);
    r2.height = Size::Fixed(40.0);
    r2.set_color([0.2, 0.8, 0.2, 1.0]);
    let r2 = ui.add(r2);

    // fill minus 40px rect
    let mut r3 = Rect::new(0.0, 40.0);
    r3.width = Size::FillMinus(40.0);
    r3.height = Size::Fixed(40.0);
    r3.set_color([0.2, 0.2, 0.8, 1.0]);
    let r3 = ui.add(r3);

    // fixed width rect for comparison
    let mut r4 = Rect::new(200.0, 40.0);
    r4.set_color([0.8, 0.8, 0.2, 1.0]);
    let r4 = ui.add(r4);

    ui.append(col, r1);
    ui.append(col, r2);
    ui.append(col, r3);
    ui.append(col, r4);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
