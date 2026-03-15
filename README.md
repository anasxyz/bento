<h1 align="center">Bento</h1> 
<p align="center"><strong>Rust GUI framework</strong></p>

## Features

* Inspired by [Qt](https://www.qt.io/)
* Cross-platform, runs on Windows, macOS, and Linux
* GPU-accelerated rendering via a custom wgpu based rendering layer
* Signals and slots system for events handling
* Flexbox layout engine
* Composable styling
* Async task system (futures, background threads, delays, repeating intervals, exclusive tasks, and timeouts)
* Builtin widget library
* Font loading and management

Bento is built on top of:
* **[`winit`](https://github.com/rust-windowing/winit)** for window handling
* **[`wgpu`](https://github.com/gfx-rs/wgpu)** for 2D rendering
* **[`Glyphon`](https://github.com/grovesNL/glyphon)** for text rendering
* **[`Taffy`](https://github.com/DioxusLabs/taffy)** for layout
* **[`Tokio`](https://github.com/tokio-rs/tokio)** for async task runtime


## Examples
Simple text input:
```rust
use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = Column::new(&mut ui);
    let btn = Button::new(&mut ui, "Click me");
    ui.append(root, btn);
    ui.set_root(root);

    ui[root].layout_mut().width = Size::Percent(100.0);
    ui[root].layout_mut().height = Size::Percent(100.0);

    ui.connect(btn, Signal::Press, move |ui| {
        println!("clicked");
    });

    ui.connect(btn, Signal::Click, move |ui| {
        btn.set_text(ui, "Clicked!");
    });

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
```

![Demo3](screenshots/demo3.gif)

<img width="1847" height="1013" alt="image" src="https://github.com/user-attachments/assets/af5d9930-a7f0-4e57-8d42-3ee90cabf231" />

![Demo](screenshots/demo.gif)
