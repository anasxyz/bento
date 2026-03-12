use bento::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }

    fn view(&mut self) -> Element {
        col(vec![
            // main content
            rect()
                .w(px(100.0))
                .h(px(100.0))
                .bg(rgb(30, 30, 30)),
        ])
        .w(pct(100.0))
        .h(pct(100.0))
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}

