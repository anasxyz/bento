#![allow(dead_code)]
#![allow(unused)]
use bento::*;
use taffy::prelude::*;

#[component]
fn app() -> impl View {
    group()
        .direction(row())
        .w(pct(100.0))
        .h(pct(100.0))
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
