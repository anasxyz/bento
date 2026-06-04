#![allow(dead_code)]
#![allow(unused)]

use bento::*;

#[component]
fn app() -> impl View {
    group()
        .direction(row())
        .w(auto())
        .h(auto())
        .child(
            group()
                .direction(col())
                .w(pct(100.0))
                .h(pct(100.0))
                .child(text(|| "hello world".into()))
                .child(text(|| "second line".into()))
                .child(text(|| "second line".into()))
                .child(text(|| "second line".into())),
        )
        .child(
            group()
                .direction(col())
                .w(pct(100.0))
                .h(pct(100.0))
                .child(text(|| "hello world".into()))
                .child(text(|| "second line".into()))
                .child(text(|| "hello world".into()))
                .child(text(|| "hello world".into())),
        )
}

#[main]
fn main() {
    App::run(app());
}
