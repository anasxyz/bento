<h1 align="center">Bento</h1> 
<p align="center"><strong>Rust GUI framework</strong></p>

> Early development. API is unstable.

## Features
* Fast and simple to use
* Reactive model inspired by [`Svelte 5`](https://github.com/sveltejs/svelte) / [`SolidJS`](https://github.com/solidjs/solid)
* Cross-platform: Linux / macOS / Windows / Web
* Extensible component system
* Async support
* Custom [`wgpu`](https://github.com/gfx-rs/wgpu) renderer
* [`Taffy`](https://github.com/dioxusLabs/taffy) Layout engine
  
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
use bento::*;

#[component]
fn app() -> impl View {
    let count = state(0);

    group()
        .direction(col())
        .w(fill())
        .h(fill())
        .p(16.0)
        .gap(8.0)
        .child(text(move || format!("count: {}", count.get())))
        .child(
            rect(|| [0.0, 0.7, 0.0, 1.0])
                .w(px(100.0))
                .h(px(40.0))
                .on(move |_: &Click| count.update(|n| n + 1)),
        )
}

#[main]
fn main() {
    App::run(app());
}

```
