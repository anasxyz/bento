use bento::*;

#[component]
fn app() -> impl View {
    let count = state(0);

    group()
        .direction(col())
        .w(fill())
        .h(fill())
        .p(px(24.0))
        .gap(px(8.0))
        .child(text("this is static"))
        .child(text(move || format!("count: {}", count.get())))
        .child(
            rect()
                .w(px(120.0))
                .h(px(40.0))
                .on(move |_: &Click| count.set(count.get() + 1))
        )
}

#[main]
fn main() {
    App::run(app());
}
