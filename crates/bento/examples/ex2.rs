#![allow(dead_code)]
#![allow(unused)]
use bento::*;

#[component]
fn app() -> impl View {
    // outer column filling the window
    group()
        .direction(Direction::Column)
        .gap(10.0)
        .padding(16.0)
        // top row — space between
        .child(
            group()
                .direction(Direction::Row)
                .gap(8.0)
                .main_axis(MainAxis::SpaceBetween)
                .child(text(|| "left".to_string()))
                .child(text(|| "center".to_string()))
                .child(text(|| "right".to_string()))
        )
        // middle row — fill children
        .child(
            group()
                .direction(Direction::Row)
                .gap(8.0)
                .child(text(|| "fill 1".to_string()).width(Size::Fill))
                .child(text(|| "fill 2".to_string()).width(Size::Fill))
                .child(text(|| "fixed".to_string()).width(Size::Fixed(80.0)))
        )
        // bottom row — cross axis center
        .child(
            group()
                .direction(Direction::Row)
                .gap(8.0)
                .cross_axis(CrossAxis::Center)
                .child(text(|| "small".to_string()))
                .child(text(|| "also small".to_string()))
        )
        // nested column — main axis center
        .child(
            group()
                .direction(Direction::Column)
                .gap(4.0)
                .main_axis(MainAxis::Center)
                .padding(8.0)
                .child(text(|| "nested 1".to_string()))
                .child(text(|| "nested 2".to_string()))
                .child(text(|| "nested 3".to_string()))
        )
}

#[main]
fn main() {
    App::run(app());
}
