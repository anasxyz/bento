use bento::*;

#[component]
fn app() -> impl View {
    timer(0.5, move || {
        println!("tick");
    });

    text(|| "hello".to_string())
}

#[main]
fn main() {
    App::run(app());
}
