use bento::*;

#[component]
fn app() -> impl View {
    let count = state(0);

    effect(move || println!("count changed: {}", count.get()));

    text(move || format!("count: {}", count.get())).on(move |e: &Click| count.set(count.get() + 1))
}

fn main() {
    App::run(app());
}
