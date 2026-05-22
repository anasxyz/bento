#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut col = Group::new();
    col.layout = Layout::Column { gap: 8.0 };
    let col = ui.add(col);

    let mut row1 = Group::new();
    row1.layout = Layout::Row { gap: 8.0 };
    let row1 = ui.add(row1);

    let mut row2 = Group::new();
    row2.layout = Layout::Row { gap: 8.0 };
    let row2 = ui.add(row2);

    let mut row3 = Group::new();
    row3.layout = Layout::Row { gap: 8.0 };
    let row3 = ui.add(row3);

    let btn1 = ui.add(Button::new("A"));
    let btn2 = ui.add(Button::new("B"));
    let btn3 = ui.add(Button::new("C"));

    let btn4 = ui.add(Button::new("D"));
    let btn5 = ui.add(Button::new("E"));
    let btn6 = ui.add(Button::new("F"));

    let btn7 = ui.add(Button::new("G"));
    let btn8 = ui.add(Button::new("H"));
    let btn9 = ui.add(Button::new("I"));

    ui.append(col, row1);
    ui.append(col, row2);
    ui.append(col, row3);

    ui.append(row1, btn1);
    ui.append(row1, btn2);
    ui.append(row1, btn3);

    ui.append(row2, btn4);
    ui.append(row2, btn5);
    ui.append(row2, btn6);

    ui.append(row3, btn7);
    ui.append(row3, btn8);
    ui.append(row3, btn9);

    // change btn5 (middle of row2)
    // row1 and row3 should be completely silent
    // in row2, only btn5 and btn6 should reposition
    // col should reposition row2 and row3 only if row2 changes height
    ui.asyncs.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            println!("--------------------------------");
            ui.get_mut(btn5).unwrap().set_text("I got much wider!");
            ui.get_mut(btn1).unwrap().set_text("I got much\n taller!");
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
