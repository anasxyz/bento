use bento::*;

fn App() -> impl View {
    let a = state(3);
    println!("a: {}", a.get());
    a.set(67);
    println!("a: {}", a.get());

    text(|| "hello".to_string())
}

fn main() {
    App::run(App());
}
