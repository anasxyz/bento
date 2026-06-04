#![allow(dead_code)]
#![allow(unused)]
use bento::*;

#[component]
fn counter(label: String) -> impl View {
    let count = state(0i32);
    group()
        .direction(Direction::Row)
        .gap(8.0)
        .child(
            rect(move || {
                let v = (count.get().abs() as f32 * 0.1).min(1.0);
                [v, 0.4, 0.8, 1.0]
            })
            .width(Size::Fixed(120.0))
            .height(Size::Fixed(24.0)),
        )
        .child(
            rect(|| [0.2, 0.8, 0.2, 1.0])
                .width(Size::Fixed(40.0))
                .height(Size::Fixed(24.0))
                .on(move |_: &Click| {
                    count.update(|v| v + 1);
                }),
        )
        .child(
            rect(|| [0.8, 0.2, 0.2, 1.0])
                .width(Size::Fixed(40.0))
                .height(Size::Fixed(24.0))
                .on(move |_: &Click| {
                    count.update(|v| v - 1);
                }),
        )
}

#[component]
fn app() -> impl View {
    group()
        .direction(Direction::Column)
        .gap(8.0)
        .padding(16.0)
        .child(counter("counter a".to_string()))
        .child(counter("counter b".to_string()))
        .child(counter("counter c".to_string()))
}

#[main]
fn main() {
    App::run(app());
}
