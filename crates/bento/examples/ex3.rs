#![allow(dead_code)]
#![allow(unused)]
use bento::*;

#[component]
fn app() -> impl View {
    group()
        .direction(Direction::Column)
        .gap(10.0)
        .padding(16.0)
        // test MainAxis::Center
        .child(
            group()
                .direction(Direction::Row)
                .main_axis(MainAxis::Center)
                .gap(8.0)
                .child(text(|| "center 1".to_string()))
                .child(text(|| "center 2".to_string())),
        )
        // test MainAxis::End
        .child(
            group()
                .direction(Direction::Row)
                .main_axis(MainAxis::End)
                .gap(8.0)
                .child(text(|| "end 1".to_string()))
                .child(text(|| "end 2".to_string())),
        )
        // test CrossAxis::Stretch
        .child(
            group()
                .direction(Direction::Row)
                .cross_axis(CrossAxis::Stretch)
                .gap(8.0)
                .child(text(|| "stretch 1".to_string()))
                .child(text(|| "stretch 2".to_string())),
        )
        // test Size::Percent
        .child(text(|| "50 percent".to_string()).width(Size::Percent(50.0)))
        // test Size::FillMinus
        .child(text(|| "fill minus 100".to_string()).width(Size::FillMinus(100.0)))
        // test nested Fill
        .child(
            group()
                .direction(Direction::Row)
                .gap(8.0)
                .child(
                    group()
                        .direction(Direction::Column)
                        .gap(4.0)
                        .child(text(|| "nested fill 1".to_string()))
                        .child(text(|| "nested fill 2".to_string())),
                )
                .child(text(|| "sibling".to_string()).width(Size::Fixed(100.0))),
        )
}

#[main]
fn main() {
    App::run(app());
}
