use bento::*;

#[component]
fn app() -> impl View {
    let pos = state(100.0f32);

    group()
        .child(
            rect(|| [1.0, 0.0, 0.0, 1.0])
                .width(Size::Fixed(50.0))
                .height(Size::Fixed(50.0)),
        )
        .child(
            rect(|| [0.0, 1.0, 0.0, 1.0])
                .width(Size::Fixed(50.0))
                .height(Size::Fixed(50.0)),
        )
        .width(Size::Fill)
        .height(Size::Fill)
}

#[main]
fn main() {
    App::run(app());
}
