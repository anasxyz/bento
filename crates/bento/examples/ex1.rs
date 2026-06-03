use bento::*;

#[component]
fn app() -> impl View {
    let count = state(0);
    let doubled = derived(move || count.get() * 2);

    // effect to print doubled count every time it changes
    effect(move || println!("doubled: {}", doubled.get()));

    text(move || format!("count: {}", count.get())).on(move |e: &Click| count.set(count.get() + 1))
}

fn main() {
    App::run(app());
}
