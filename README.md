<h1 align="center">Bento</h1> 
<p align="center"><strong>Rust GUI framework</strong></p>

> Warning: Bento is in early development. The API is unstable and will change between versions. Not recommended for production use.

## Features

- Cross-platform, runs on Windows, macOS, and Linux
- Optimised, high performance, GPU-accelerated rendering with low overhead at scale
- Extensible UI widget system using `#[derive(Widget)]` trait, make your own custom UI widgets that integrate seamlessly
- Comprehensive built-in widget and styling library
- Support for multiple windows
- Rich event system with builtin input handling, widget lifecycle hooks using `connect()`, and custom event broadcasting using `emit()`
- Flexbox layout engine
- Font loading and management

Bento is built on top of:
* **[`winit`](https://github.com/rust-windowing/winit)** for window handling
* **[`wgpu`](https://github.com/gfx-rs/wgpu)** for 2D rendering
* **[`cosmic_text`](https://github.com/pop-os/cosmic-text)** for text rendering
* **[`taffy`](https://github.com/DioxusLabs/taffy)** for layout
* **[`Tokio`](https://github.com/tokio-rs/tokio)** for async task runtime

<img width="795" height="598" alt="image" src="https://github.com/user-attachments/assets/755ca1bb-91bb-470b-b2dd-b6d37eeb9382" />
