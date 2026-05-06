use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    app.open_window(WindowConfig { clear_color: Color::rgb(0, 0, 0), ..Default::default() }, ui);
    app.run();
}
