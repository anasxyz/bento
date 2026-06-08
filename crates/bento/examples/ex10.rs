use bento::*;

#[component]
fn app() -> impl View {
    group()
        .direction(col())
        .w(fill())
        .h(fill())
        .child(
            group()
                .direction(col())
                .m_left(px(50.0))
                .w(px(200.0))
                .h(px(150.0))
                .scroll()
                .child(text("item 1"))
                .child(text("item 2"))
                .child(text("item 3"))
                .child(text("item 4"))
                .child(text("item 5555555555555555555555555"))
                .child(text("item 6"))
                .child(text("item 7"))
                .child(text("item 8"))
                .child(text("item 9"))
                .child(text("item 10")),
        )
}

#[main]
fn main() {
    App::run(app());
}
