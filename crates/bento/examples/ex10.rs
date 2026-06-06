use bento::*;

#[component]
fn app() -> impl View {
    group()
        .direction(row())
        .w(px(300.0))
        .h(px(100.0))
        .scroll()
        .child(text(|| "item 1".into()).w(px(100.0)))
        .child(text(|| "item 2".into()).w(px(100.0)))
        .child(text(|| "item 3".into()).w(px(100.0)))
        .child(text(|| "item 4".into()).w(px(100.0)))
        .child(text(|| "item 5".into()).w(px(100.0)))
        .child(text(|| "item 6".into()).w(px(100.0)))
        .child(text(|| "item 7".into()).w(px(100.0)))
        .child(text(|| "item 8".into()).w(px(100.0)))
}

#[main]
fn main() {
    App::run(app());
}
