#![allow(dead_code)]
#![allow(unused)]
use bento::*;

#[component]
fn app() -> impl View {
    let a = state(0);
    let b = state(0);

    group()
        .child(text(move || format!("a: {}", a.get())).on(move |e: &Click| a.set(a.get() + 1)))
        .child(text(move || format!("b: {}", b.get())).on(move |e: &Click| b.set(b.get() + 1)))
}

#[main]
fn main() {
    App::run(app());
}
