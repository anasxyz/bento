#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut col = Group::new();
    col.layout = Layout::Row { gap: 8.0 };
    col.width = Size::Fixed(150.0);
    let col = ui.add(col);

    let mut btn = Button::new("First button");
    btn.width = Size::Fill;
    let btn = ui.add(btn);

    ui.append(col, btn);

    let l = ui.listen(btn, |e: &Click, ui: &mut Ui| {
        println!("Clicked button");
    });

    ui.asyncs.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            ui.unlisten(l);
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
