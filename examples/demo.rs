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
                .w(pct(100.0))
                .h(pct(100.0))
                .bg(Color::rgb(30, 30, 30)),
            // modal overlay
            modal("Hello world!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!111111111111111111111111111"),
        ])
        .w(pct(100.0))
        .h(pct(100.0))
    }
}

fn modal(title: &str) -> Element {
    col(vec![
        col(vec![
            rect()
                .w(pct(100.0))
                .h(pct(100.0))
                .bg(Color::rgba(40, 40, 40, 255))
                .border(3.0)
                .border_color(Color::rgb(80, 80, 80))
                .border_radius(8.0)
                .absolute(),
            text(title).bold().font_size(12.0).text_color(Color::WHITE).w(pct(100.0)).align_self(AlignSelf::Start).bottom(pct(30.0)),
        ])
        .w(px(300.0))
        .h(px(200.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .p([10.0, 10.0, 10.0, 10.0]),
    ])
    .w(pct(100.0))
    .h(pct(100.0))
    .absolute()
    .align_items(AlignItems::Center)
    .justify_content(JustifyContent::Center)
}

fn main() {
    MyApp::run(WindowSettings::default());
}
