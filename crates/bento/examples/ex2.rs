use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 0.0, 100.0, 100.0));

    ui.listen_once(rect, move |e: &Click, ui| {
        println!("first click — now listening for keys");
        ui.listen(rect, move |e: &KeyPress, ui| {
            println!("key={:?}", e.key);
        });
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
