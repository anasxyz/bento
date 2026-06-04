#![allow(dead_code)]
#![allow(unused)]
use bento::*;

#[component]
fn app() -> impl View {
    let count = state(0);

    group()
        .direction(col())
        .w(fill())
        .h(fill())
        .p(16.0)
        .gap(8.0)
        .child(text(move || format!("count: {}", count.get())))
        .child(
            rect(|| [0.0, 0.7, 0.0, 1.0])
                .w(px(100.0))
                .h(px(40.0))
                .on(move |_: &Click| count.update(|n| n + 1)),
        )
}

#[main]
fn main() {
    App::run(app());
}
