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

    let btn1 = ui.add(Button::new("Click me"));
    let btn2 = ui.add(Button::new("Hello"));
    let btn3 = ui.add(Button::new("World"));
    let btn4 = ui.add(Button::new("Bento"));

    ui.append(col, row1);
    ui.append(col, row2);
    ui.append(row1, btn1);
    ui.append(row1, btn2);
    ui.append(row2, btn3);
    ui.append(row2, btn4);

    ui.asyncs.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            ui.get_mut(btn1).unwrap().set_text("Clickeddddddddddd!");
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
