use bento::*;

fn counter() -> impl View {
    let count = state(0i32);

    let _eff = effect(move || {
        println!("count is: {}", count.get());
    });

    count.set(42);
    count.set(43);

    rect().child(text(move || format!("Count: {}", count.get())))
}

fn main() {
    App::run(counter());
}
