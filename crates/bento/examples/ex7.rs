#![allow(dead_code)]
#![allow(unused)]
use bento::*;
use taffy::prelude::*;

#[component]
fn app() -> impl View {
    group()
        .direction(FlexDirection::Row)
        .width(Dimension::from_percent(1.0))
        .height(Dimension::from_percent(1.0))
        .child(
            group()
                .direction(FlexDirection::Column)
                .width(Dimension::from_percent(1.0))
                .height(Dimension::from_percent(1.0))
                .child(text(|| "hello world".into()))
                .child(text(|| "second line".into()))
                .child(text(|| "second line".into()))
                .child(text(|| "second line".into())),
        )
        .child(
            group()
                .direction(FlexDirection::Column)
                .width(Dimension::from_percent(1.0))
                .height(Dimension::from_percent(1.0))
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
