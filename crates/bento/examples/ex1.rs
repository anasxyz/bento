use bento::*;

#[component]
fn App() -> impl View {
    let count = state(0i32);
    let doubled = derived(move || count.get() * 2);

    count.set(3);

    rect()
        .child(text(move || format!("count: {}", count.get())))
        .child(text(move || format!("doubled: {}", doubled.get())))
}

fn main() {
    App::run(App());
}
