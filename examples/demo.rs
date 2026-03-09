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
                .w(Size::Percent(100.0))
                .h(Size::Percent(100.0))
                .bg(Color::rgb(30, 30, 30)),
            // modal overlay
            modal("Hello world!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!dddddddddddd"),
        ])
        .w(Size::Percent(100.0))
        .h(Size::Percent(100.0))
    }
}

fn modal(title: &str) -> Element {
    col(vec![
        col(vec![
            rect()
                .w(Size::Percent(100.0))
                .h(Size::Percent(100.0))
                .bg(Color::rgba(40, 40, 40, 255))
                .border(3.0)
                .border_color(Color::rgb(80, 80, 80))
                .border_radius(8.0)
                .absolute(),
            text(title).bold().font_size(12.0).text_color(Color::WHITE).w(Size::Percent(100.0)).align_self(AlignSelf::Start).bottom(Size::Percent(30.0)),
        ])
        .w(Size::Fixed(300.0))
        .h(Size::Fixed(200.0))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .p([10.0, 10.0, 10.0, 10.0]),
    ])
    .w(Size::Percent(100.0))
    .h(Size::Percent(100.0))
    .absolute()
    .align_items(AlignItems::Center)
    .justify_content(JustifyContent::Center)
}

fn main() {
    MyApp::run(WindowSettings::default());
}
