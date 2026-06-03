use bento::*;

#[component]
fn app() -> impl View {
    let boom = state(0);

    text(move || format!("boom: {}", boom.get()))
        .on(move |e: &Click| boom.set(boom.get() + 1))
}

#[main]
fn main() {
    App::run(app());
}
