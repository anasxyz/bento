use bento::*;

#[component]
fn app() -> impl View {
    group()
        .direction(col())
        .w(px(300.0))
        .h(px(400.0))
        .scroll()
        .child(text(|| "item 1".into()))
        .child(text(|| "item 2".into()))
        .child(text(|| "item 3".into()))
        .child(text(|| "item 4".into()))
        .child(text(|| "item 5".into()))
        .child(text(|| "item 6".into()))
        .child(text(|| "item 7".into()))
        .child(text(|| "item 8".into()))
        .child(text(|| "item 9".into()))
        .child(text(|| "item 10".into()))
        .child(text(|| "item 11".into()))
        .child(text(|| "item 12".into()))
        .child(text(|| "item 13".into()))
        .child(text(|| "item 14".into()))
        .child(text(|| "item 15".into()))
        .child(text(|| "item 16".into()))
        .child(text(|| "item 17".into()))
        .child(text(|| "item 18".into()))
        .child(text(|| "item 19".into()))
        .child(text(|| "item 20".into()))
}

#[main]
fn main() {
    App::run(app());
}
