use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let button = ui.add(Rect::new(50.0, 50.0, 100.0, 40.0));

    ui.listen(button, move |_e: &Click, _ui| {
        println!("clicked");
    });

    let mut count = 0;
    ui.listen_while(button, move |_e: &Click, _ui| {
        count += 1;
        println!("listen_while click {}", count);
        return count < 3;
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
