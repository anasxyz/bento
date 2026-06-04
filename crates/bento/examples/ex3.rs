#![allow(dead_code)]
#![allow(unused)]
use bento::*;

#[component]
fn app() -> impl View {
    let show = state(false);

    group()
        .direction(Direction::Column)
        .gap(8.0)
        .padding(16.0)
        .child(
            rect(|| [0.2, 0.8, 0.2, 1.0])
                .width(Size::Fixed(80.0))
                .height(Size::Fixed(20.0))
                .on(move |_: &Click| {
                    show.update(|v| !v);
                }),
        )
        .when(show, || {
            rect(|| [0.8, 0.2, 0.2, 1.0])
                .width(Size::Fixed(200.0))
                .height(Size::Fixed(100.0))
        })
        .child(
            rect(|| [0.2, 0.4, 0.8, 1.0])
                .width(Size::Fixed(80.0))
                .height(Size::Fixed(20.0)),
        )
        .width(Size::Fill)
        .height(Size::Fill)
}

#[main]
fn main() {
    App::run(app());
}
