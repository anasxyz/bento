use bento::*;

#[component]
fn app() -> impl View {
    let width = state(300.0f32);

    group()
        .direction(col())
        .w(px(400.0))
        .h(px(400.0))
        .child(
            rect(|| [0.0, 0.5, 1.0, 1.0])
                .w(move || px(width.get()))
                .h(px(100.0))
        )
        .child(
            rect(|| [1.0, 0.0, 0.0, 1.0])
                .w(px(100.0))
                .h(px(100.0))
                .on(move |_: &Click| {
                    width.set(width.get() + 50.0);
                })
        )
}

#[main]
fn main() {
    App::run(app());
}
