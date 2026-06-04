#![allow(dead_code)]
#![allow(unused)]
use bento::*;

#[component]
fn app() -> impl View {
    let pos = state(200.0f32);

    group()
        .main_axis(MainAxis::Center)
        .width(Size::Fill)
        .height(Size::Fill)
        .child(
            rect(|| [1.0, 0.0, 0.0, 1.0])
                .width(Size::Fixed(50.0))
                .height(Size::Fixed(50.0))
                .on(move |_: &Click| {
                    println!("[click] pos = {}", pos.get());
                    pos.update(|x| x + 10.0)
                }),
        )
        .child(
            rect(|| [0.0, 1.0, 0.0, 1.0])
                .width(Size::Fixed(50.0))
                .height(Size::Fixed(50.0))
                .x(move || pos.get())
                .y(|| 200.0),
        )
}

#[main]
fn main() {
    App::run(app());
}
