use bento::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }

    fn view(&mut self) -> Element {
        row(vec![
            text("Hello world1!").bold().font_size(32.0).mx(AUTO).absolute().top(Size::Fixed(10.0)),
            text("Hello world2!").bold().font_size(32.0),
        ])
        .w(Size::Percent(100.0))
        .h(Size::Percent(100.0))
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}
