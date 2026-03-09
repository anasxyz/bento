use bento::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }

    fn view(&mut self) -> Element {
        row(vec![
            text("Hello world!").bold().font_size(32.0),
            text("Hello world!").bold().font_size(32.0),
        ])
        .w(Size::Percent(100.0))
        .h(Size::Percent(100.0))
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}
