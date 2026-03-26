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
* **[`glyphon`](https://github.com/grovesNL/glyphon)** for text rendering
* **[`Taffy`](https://github.com/DioxusLabs/taffy)** for layout
* **[`Tokio`](https://github.com/tokio-rs/tokio)** for async task runtime
