#![allow(dead_code)]
#![allow(unused)]
use bento::*;
use taffy::prelude::*;

#[component]
fn app() -> impl View {
    group()
        .direction(FlexDirection::Column)
        .width(Dimension::from_percent(1.0))
        .height(Dimension::from_percent(1.0))
        .child(text(|| "hello world".into()))
        .child(text(|| "second line".into()))
}

#[main]
fn main() {
    App::run(app());
}
