#![allow(dead_code)]
#![allow(unused)]
use bento::*;

#[snippet]
fn divider() -> impl View {
    rect(|| [0.3, 0.3, 0.3, 1.0])
        .width(Size::Fill)
        .height(Size::Fixed(2.0))
}

#[component]
fn toggle_panel(color: [f32; 4]) -> impl View {
    let open = state(false);

    group()
        .direction(Direction::Column)
        .gap(4.0)
        .child(
            rect(move || if open.get() { [0.4, 0.4, 0.4, 1.0] } else { color })
                .width(Size::Fill)
                .height(Size::Fixed(30.0))
                .on(move |_: &Click| { open.update(|v| !v); }),
        )
        .when(open, || {
            group()
                .direction(Direction::Column)
                .gap(4.0)
                .child(rect(|| [0.2, 0.6, 0.4, 1.0]).width(Size::Fill).height(Size::Fixed(20.0)))
                .child(rect(|| [0.2, 0.4, 0.6, 1.0]).width(Size::Fill).height(Size::Fixed(20.0)))
                .child(rect(|| [0.6, 0.2, 0.4, 1.0]).width(Size::Fill).height(Size::Fixed(20.0)))
        })
}

#[component]
fn app() -> impl View {
    group()
        .direction(Direction::Column)
        .gap(8.0)
        .padding(16.0)
        .child(toggle_panel([0.8, 0.2, 0.2, 1.0]))
        .child(divider())
        .child(toggle_panel([0.2, 0.4, 0.8, 1.0]))
        .child(divider())
        .child(toggle_panel([0.2, 0.8, 0.4, 1.0]))
        .width(Size::Fill)
        .height(Size::Fill)
}

#[main]
fn main() {
    App::run(app());
}
