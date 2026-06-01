use bento::*;

#[component]
fn App() -> impl View {
    let a = state(0i32);
    let b = state(0i32);

    group()
        .child(text(move || {
            println!("text a closure ran");
            format!("a: {}", a.get())
        }))
        .child(text(move || {
            println!("text b closure ran");
            format!("b: {}", b.get())
        }))
}

fn main() {
    App::run(App());
}
