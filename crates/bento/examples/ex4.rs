#![allow(dead_code)]
#![allow(unused)]

use bento::*;

#[component]
fn app() -> impl View {
    group()
        .direction(Direction::Column)
        .child(
            group()
                .direction(Direction::Row)
                .child(text(|| "auto".to_string()))
                .width(Size::Auto)
        )
}

#[main]
fn main() {
    App::run(app());
}
