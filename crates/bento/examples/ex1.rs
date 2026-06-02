use bento::*;

fn App() -> impl View {
    let a = state(0);
    a.set(67);
    text(move || format!("a: {}", a.get()))
}

fn main() {
    App::run(App());
}
