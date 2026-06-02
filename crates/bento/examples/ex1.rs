use bento::*;

#[component]
fn App() -> impl View {
    let a = state("Hello World");

    text(move || a.get().to_string())
}

fn main() {
    App::run(App());
}
