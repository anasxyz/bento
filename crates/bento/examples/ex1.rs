use bento::*;

fn app() -> impl View {
    let count = state(0);

    let inc = move || count.set(count.get() + 1);

    timer(2.0, move || count.set(count.get() + 1));

    text(move || format!("count: {}", count.get()))
        .on(move |e: &Click| inc())
}

fn main() {
    App::run(app());
}
