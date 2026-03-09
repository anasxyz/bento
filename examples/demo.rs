use bento::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }

    fn view(&mut self) -> Element {
        col(vec![
            text("Hello world!"),
        ])
        .w(Size::Percent(100.0))
        .h(Size::Percent(100.0))
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}
