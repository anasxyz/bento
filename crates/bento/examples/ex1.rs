use bento::*;

fn app() -> impl View {
    group()
        .child(text(|| "hello".to_string()))
        .child(text(|| "world".to_string()))
}

fn main() {
    App::run(app());
}
