use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    app.open_window(WindowConfig::default(), ui);
    app.run();
}

