<h1 align="center">Bento</h1> 
<p align="center"><strong>Rust GUI framework</strong></p>

> Early development. API is unstable.

## Features
* Fast
* Cross-platform: Linux / macOS / Windows / Web
* Extensible widget system
* Async support
* Custom layout engine

## Example

Simple counter app:

```rust
use bento::*;

struct Counter {
    count: i32,
    label: WidgetHandle<Text>,
}

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    // Create a row container and customise its layout
    let row = ui.add(ui.root(), Group::new());
    ui.set(row, |g: &mut Group| {
        g.layout = Layout::Row {
            gap: 8.0,
            padding: [0.0; 4],
            main_axis: MainAxis::Center,
            cross_axis: CrossAxis::Center,
            wrap: false,
        };
    });

    // Create widgets and add them to the row
    let btn_dec = ui.add(row, Button::new("-"));
    let label = ui.add(row, Text::new("0"));
    let btn_inc = ui.add(row, Button::new("+"));

    // Set the apps state
    ui.set_state(Counter { count: 0, label });

    // Listen for clicks on the increment button
    ui.listen(btn_inc, move |_: &Click, ui: &mut Ui| {
        ui.with_state(|s: &mut Counter, ui: &mut Ui| {
            s.count += 1;
            ui.set(s.label, |t: &mut Text| {
                t.set_content(&format!("{}", s.count))
            });
        });
    });

    // Listen for clicks on the decrement button
    ui.listen(btn_dec, move |_: &Click, ui: &mut Ui| {
        ui.with_state(|s: &mut Counter, ui: &mut Ui| {
            s.count -= 1;
            ui.set(s.label, |t: &mut Text| {
                t.set_content(&format!("{}", s.count))
            });
        });
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
```

## Screenshots
<img src="media/show2.png" width="800" height="600">
<img src="media/show1.png" width="800" height="600">
<img src="media/demo_dock.gif" width="800" height="600">
