use bento::*;

#[component]
fn App() -> impl View {
    let count = state(0i32);
    let doubled = derived(move || count.get() * 2);

    count.set(3);

    button(move || format!("count: {}", count.get()))
}

fn main() {
    App::run(App());
}
