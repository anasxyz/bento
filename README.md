<h1 align="center">Bento</h1> 
<p align="center"><strong>Rust GUI framework</strong></p>

> Warning: Bento is in early development. The API is unstable and will change between versions. Not recommended for production use.

## Features

- Cross-platform, runs on Windows, macOS, and Linux
- Optimised, high performance, GPU-accelerated rendering with low overhead at scale
- Built-in widget library + Custom widgets using `#[derive(Widget)]` trait
- Event system:
    - Widget lifecycle hooks using `on::<T, Event>(handle, callback)`
    - Event broadcasting using `emit(handle, event)`
    - Custom event support coming soon
- Flexbox layout engine
- Font loading and management

Bento is built on top of:
* **[`winit`](https://github.com/rust-windowing/winit)** for window handling
* **[`wgpu`](https://github.com/gfx-rs/wgpu)** for 2D rendering
* **[`cosmic-text`](https://github.com/pop-os/cosmic-text)** for text rendering
* **[`taffy`](https://github.com/DioxusLabs/taffy)** for layout
* **[`Tokio`](https://github.com/tokio-rs/tokio)** for async task runtime

<img width="795" height="598" alt="image" src="https://github.com/user-attachments/assets/755ca1bb-91bb-470b-b2dd-b6d37eeb9382" />
