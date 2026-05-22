#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut col = Group::new();
    col.layout = Layout::Column { gap: 0.0 };
    let col = ui.add(col);

    let rect = ui.add(Rect::new(100.0, 50.0));

    ui.append(col, rect);

    ui.asyncs.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            ui.get_mut(col).unwrap().set_scroll_x(100.0);
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}

