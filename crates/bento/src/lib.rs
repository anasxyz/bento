#![allow(dead_code)]
#![allow(unused)]

// reactive primitives
pub use bento_ui::{derived, effect, inspect, state};
// views
pub use bento_ui::{View, group, text, each};
// async
pub use bento_ui::{spawn, timer};
// events
pub use bento_ui::events::*;

// winit/app
pub use bento_winit::App;

// macros
pub use bento_macros::{component, main};

#[component]
fn app() -> impl View {
    let boom = state(0);
    text(move || format!("boom: {}", boom.get())).on(move |e: &Click| boom.set(boom.get() + 1))
}

#[main]
fn main() {
    App::run(app());
}
