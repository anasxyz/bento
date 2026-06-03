<h1 align="center">Bento</h1> 
<p align="center"><strong>Rust GUI framework</strong></p>

> Early development. API is unstable.

## Features
* Fast and extremely simple to use
* Reactive model inspired by [Svelte 5](https://github.com/sveltejs/svelte)
* Cross-platform: Linux / macOS / Windows / Web
* Extensible widget system
* Async support
* Custom [wgpu](https://github.com/gfx-rs/wgpu)-based renderer
* Custom layout engine
  
| Svelte 5 | Bento |
|----------|-------|
| `.svelte component` | `#[component]` |
| `$state` | `state(value)` |
| `$derived` | `derived()` |
| `$effect` | `effect()` |
| `$inspect` | `inspect!(signal)` |
| `$bindable` | signals are `Copy`, just pass them directly |
| `$props` | just function arguments in Rust |
| `$host` | web component specific, not relevant |
| `bind:value` | `.bind(signal)` |
| `on:event` | `.on(\|e: &Event\| ...)` |
| `{#await}` | `await_(async { ... })` |
| `{#each}` | `each(signal, \|item\| ...)` |
| `{#if} / {:else} / {:else if}` | `.show(\|\| ...)` |
| `{#key}` | handled automatically by the retained tree |
| `#snippet` | just a function returning `impl View` |
| `transition:` | `.transition(Fade)` |
| `in:` | `.in_transition(FlyIn::y(200))` |
| `out:` | `.out_transition(Fade)` |

## Example

```rust
#[component]
fn app() -> impl View {
    let count = state(0);
    let doubled = derived(move || count.get() * 2);

    effect(move || {
        println!("count changed: {}", count.get());
    });

    text(move || format!("count: {}, doubled: {}", count.get(), doubled.get()))
        .on(move |e: &Click| count.set(count.get() + 1))
}

#[main]
fn main() {
    App::run(app());
}
```
