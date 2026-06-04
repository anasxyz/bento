#![allow(dead_code)]
#![allow(unused)]

use bento::*;

#[component]
fn app() -> impl View {
    let pos = state(100.0f32);

    group()
        .child(
            rect(|| [1.0, 0.0, 0.0, 1.0])
                .width(Size::Fixed(50.0))
                .height(Size::Fixed(50.0)),
        )
        .child(
            rect(|| [0.0, 1.0, 0.0, 1.0])
                .width(Size::Fixed(50.0))
                .height(Size::Fixed(50.0))
                .x(200.0)
                .y(200.0),
        )
        .main_axis(MainAxis::Center)
        .width(Size::Fill)
        .height(Size::Fill)
}

#[main]
fn main() {
    App::run(app());
}
