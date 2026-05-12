use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let button = ui.add(Rect::new(50.0, 50.0, 100.0, 40.0));

    let mut count = 0;
    let conn = ui.listen(button, move |_e: &Click, ui| {
        count += 1;
        println!("click {}", count);
        if count == 3 {
            println!("unsubscribing");
            ui.listen_off(conn);
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
