use bento::*;

#[component]
fn app() -> impl View {
    let value = state(String::new());

    group()
        .direction(col())
        .w(fill())
        .h(fill())
        .p(px(24.0))
        .gap(px(8.0))
        .child(
            text_input(value)
                .w(fill())
                .h(px(36.0))
                .on(move |e: &FocusGained| {
                    println!("focus gained");
                }),
        )
        .child(text(move || format!("value: {}", value.get())))
}

#[main]
fn main() {
    App::run(app());
}
