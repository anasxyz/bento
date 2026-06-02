<h1 align="center">Bento</h1> 
<p align="center"><strong>Rust GUI framework</strong></p>

> Early development. API is unstable.

## Features
* Fast
* Reactive model inspired by Svelte / Leptos
* Cross-platform: Linux / macOS / Windows / Web
* Extensible widget system
* Async support
* Custom layout engine

## Example

```rust
fn app() -> impl View {
    let count = state(0);

    text(move || format!("count: {}", count.get()))
        .on(move |e: &Click| count.set(count.get() + 1))
}

fn main() {
    App::run(app());
}
```
